use std::{
  fs::File,
  ops::DerefMut,
  path::PathBuf,
  process::{Command, Stdio},
  sync::{Arc, Mutex},
  time::Duration,
};

use anyhow::{Result, anyhow, bail, ensure};
use args::Cli;
use clap::Parser;
use log::Log;

use modmap::ExtModuleMap;
mod args;
mod constants;
mod libc_wrappers;
mod log;
mod modmap;
mod shmem_capture;
mod sizetype_handlers;
mod stages;
use shmem_capture::{
  MetadataPublisher,
  arg_capture::perform_arg_capture,
  call_tracing::msg_handler,
  cleanup,
  hooklib_commons::{META_MEM_NAME, META_SEM_ACK, META_SEM_DATA},
  send_arg_capture_metadata, send_call_tracing_metadata, send_test_metadata,
};
use stages::{
  arg_capture::{ArgPacketDumper, PacketReader},
  call_tracing::{
    export_call_trace_data, export_tracing_selection, import_call_trace_data,
    import_tracing_selection, obtain_function_id_selection, print_call_tracing_summary,
  },
  testing::test_server_job,
};
use tokio::time::{sleep, timeout};

use crate::{
  modmap::{IntegralFnId, IntegralModId, NumFunUid},
  shmem_capture::TracingInfra,
};

fn obtain_module_map(path: &std::path::PathBuf) -> Result<ExtModuleMap> {
  let lg = Log::get("obtain_module_map");
  let mb_modules = ExtModuleMap::try_from(path);
  match mb_modules {
    Ok(m) => Ok(m),
    Err(e) => {
      lg.crit(format!(
        "Could not parse module mapping from {}:\n{}",
        path.to_string_lossy(),
        e
      ));
      bail!("Could not parse module mapping");
    }
  }
}

fn cmd_from_args(args: &[String]) -> Result<Command> {
  ensure!(!args.is_empty(), "Command must be specified");
  let mut cmd = std::process::Command::new(args.first().unwrap());
  cmd.args(args.iter().skip(1));
  Ok(cmd)
}

#[tokio::main()]
async fn main() -> Result<()> {
  Log::set_verbosity(255);
  let lg = Log::get("main");
  let cli = Cli::try_parse()?;
  Log::set_verbosity(cli.verbose);
  lg.info(format!("Verbosity: {}", cli.verbose));

  if cli.cleanup {
    lg.info("Cleanup");
    return cleanup(&cli.fd_prefix);
  }

  let mut modules = obtain_module_map(&cli.modmap)?;

  let (buff_count, buff_size) = (cli.buff_count, cli.buff_size);
  let mem_cstr = std::ffi::CStr::from_bytes_with_nul(META_MEM_NAME)?;
  let sem_str = String::from_utf8(META_SEM_DATA.split_last().unwrap().1.to_vec())?;
  let ack_str = String::from_utf8(META_SEM_ACK.split_last().unwrap().1.to_vec())?;
  let metadata_svr = Arc::new(Mutex::new(MetadataPublisher::new(
    mem_cstr, &sem_str, &ack_str,
  )?));

  match cli.stage {
    args::Stage::TraceCalls {
      mut out_file,
      import_path,
    } => {
      if import_path.is_some() {
        out_file = None;
      }

      let mut pairs = if let Some(in_path) = import_path {
        lg.trace("Importing");
        let result = import_call_trace_data(in_path, &modules)?;
        lg.info("Import done");
        result
      } else {
        lg.info("Initializing tracing infrastructure");
        let mut tracing_infra =
          TracingInfra::try_new(&cli.fd_prefix, buff_count, buff_size).await?;

        let mut guard = metadata_svr.lock().unwrap();
        send_call_tracing_metadata(guard.deref_mut(), buff_count, buff_size)?;

        lg.info("Listening!");

        let pairs = match msg_handler(
          &mut tracing_infra,
          buff_size as usize,
          buff_count as usize,
          &modules,
        ) {
          Ok(freqs) => Ok(freqs.into_iter().collect::<Vec<(_, _)>>()),
          Err(e) => {
            lg.crit(e.to_string());
            Err(e)
          }
        };
        lg.info("Shutting down call tracing infrastructure...");
        tracing_infra.deinit()?;

        pairs?
      };

      pairs.sort_by(|a, b| b.1.cmp(&a.1));
      print_call_tracing_summary(&mut pairs, &modules);

      if let Some(out_path) = out_file {
        lg.trace("Exporting");

        let _ = export_call_trace_data(&pairs, out_path)
          .inspect_err(|e| lg.crit(format!("Export failed: {e}")));

        lg.info("Export done");
      }

      let traces = pairs.iter().map(|x| x.0).collect::<Vec<NumFunUid>>();
      let selected_fns = obtain_function_id_selection(&traces, &modules);
      export_tracing_selection(&selected_fns, &modules)?;
    }
    args::Stage::CaptureArgs {
      selection_file,
      out_dir,
      mem_limit,
      command,
    } => {
      lg.info("Reading function selection");
      let selection = import_tracing_selection(&selection_file)?;

      lg.info("Masking");
      modules.mask_include(&selection)?;

      lg.info("Setting up function packet dumping");
      let mut dumper = ArgPacketDumper::new(&out_dir, &modules, mem_limit as usize)?;

      lg.info("Initializing tracing infrastructure");
      let mut tracing_infra = TracingInfra::try_new(&cli.fd_prefix, buff_count, buff_size).await?;

      let mut cmd = cmd_from_args(&command)?;
      let mut guard = metadata_svr.lock().unwrap();
      let meta_fut = send_arg_capture_metadata(guard.deref_mut(), buff_count, buff_size);

      let _ = cmd
        .spawn()
        .map_err(|e| anyhow!(e).context("Failed to spawn from command"))?;
      meta_fut?;

      lg.info("Listening!");

      perform_arg_capture(
        &mut tracing_infra,
        buff_size as usize,
        buff_count as usize,
        &modules,
        &mut dumper,
      )?;

      lg.info("Shutting down tracing infrastructure...");
      tracing_infra.deinit()?;
    }
    args::Stage::Test {
      selection_file,
      capture_dir,
      mem_limit,
      test_output,
      command,
    } => {
      let command = Arc::new(command);
      lg.info("Reading function selection");
      let selection = import_tracing_selection(&selection_file)?;
      lg.info("Masking");
      modules.mask_include(&selection)?;

      let modules = Arc::new(modules);
      lg.info("Setting up function packet reader");

      let packet_reader = PacketReader::new(&capture_dir, &modules, mem_limit as usize)
        .map_err(|e| anyhow!(e).context("Packet reader setup failed"))?;

      lg.info("Setting up function packet server");
      let (end_tx, end_rx) = tokio::sync::oneshot::channel::<()>();
      let (ready_tx, ready_rx) = tokio::sync::oneshot::channel::<()>();
      let results = Arc::new(Mutex::new(Vec::with_capacity(500)));
      let svr = tokio::spawn(test_server_job(
        cli.fd_prefix.clone(),
        capture_dir,
        modules.clone(),
        mem_limit as usize,
        (ready_tx, end_rx),
        results.clone(),
      ));
      let output_gen = Arc::new(TestOutputPathGen::new(test_output));
      // wait for server to be ready
      match timeout(Duration::from_secs(10), ready_rx).await {
        Ok(Ok(())) => lg.trace("Server ready"),
        Err(_) => bail!("server ready timeout"),
        Ok(Err(e)) => bail!("server ready error: {}", e),
      }

      let mut futs = vec![];

      for module in modules.modules() {
        for function in modules.functions(*module).unwrap() {
          let test_count = packet_reader
            .get_packet_count(*module, *function)
            .ok_or(anyhow!(
              "Not found tests: {} {}",
              module.hex_string(),
              function.hex_string()
            ))?;
          let arg_count = packet_reader
            .get_arg_count(*module, *function)
            .ok_or(anyhow!(
              "Not found args: {} {}",
              module.hex_string(),
              function.hex_string()
            ))?;

          if test_count == 0 || arg_count == 0 {
            Log::get("send_test_metadata").warn(format!(
              "Skipping M: {} F: {} due to zero arg/test count a: {}, t:{}",
              module.hex_string(),
              function.hex_string(),
              arg_count,
              test_count
            ));
            continue;
          }

          lg.info(format!(
            "Run program for fn m: {} f: {}",
            module.hex_string(),
            function.hex_string()
          ));

          let test_job = tokio::spawn(test_job(
            metadata_svr.clone(),
            buff_count,
            buff_size,
            *module,
            *function,
            arg_count,
            test_count,
            command.clone(),
            output_gen.clone(),
          ));

          futs.push(test_job);
        }
      }
      lg.info("Waiting for jobs to finish...");
      for fut in futs {
        fut.await??;
      }
      lg.info("Waiting for server to exit...");
      let defer_res_end_svr = end_tx.send(()).map_err(|_| anyhow!("failed to end server"));
      let defer_res_joins = svr.await.map_err(|e| anyhow!(e).context("joins"));

      lg.info("---------------------------------------------------------------");
      let mut results = results.lock().unwrap();
      lg.info(format!("Test results ({}): ", results.len()));
      lg.info("Module ID | Function ID |  Call  | Packet | Result");
      results.sort_by(|a, b| a.3.0.cmp(&b.3.0));
      results.sort_by(|a, b| a.2.0.cmp(&b.2.0));
      results.sort_by(|a, b| a.1.cmp(&b.1));
      results.sort_by(|a, b| a.0.cmp(&b.0));
      for result in results.iter() {
        let s = format!(
          "{:^10}|{:^13}|{:^8}|{:^8}| {:?}",
          result.0.hex_string(),
          result.1.hex_string(),
          result.2.0,
          result.3.0,
          result.4
        );
        lg.info(s);
      }
      lg.info("---------------------------------------------------------------");
      lg.trace("Cleaning up");
      defer_res_end_svr?;
      defer_res_joins??;
    }
  }

  let metadata_svr = Arc::try_unwrap(metadata_svr)
    .map_err(|_| anyhow!("Failed to unwrap from arc... this is not expected"))?;
  metadata_svr.into_inner().unwrap().deinit()?;

  lg.info("Exiting...");
  Ok(())
}

async fn test_job(
  metadata_svr: Arc<Mutex<MetadataPublisher<'_>>>,
  buff_count: u32,
  buff_size: u32,
  m: IntegralModId,
  f: IntegralFnId,
  arg_count: u32,
  test_count: u32,
  command: Arc<Vec<String>>,
  output_gen: Arc<Option<TestOutputPathGen>>,
) -> Result<()> {
  // let mut tests = vec![];
  for call_idx in 0..test_count {
    {
      let mut guard = metadata_svr.lock().unwrap();
      send_test_metadata(
        guard.deref_mut(),
        buff_count,
        buff_size,
        m,
        f,
        arg_count,
        test_count,
        call_idx + 2,
      )?;
    }
    let mut cmd = cmd_from_args(&command)?;
    if let Some(output_gen) = output_gen.as_ref() {
      let out_path = output_gen.get_out_path(m, f, call_idx + 1);
      let err_path = output_gen.get_err_path(m, f, call_idx + 1);
      cmd.stdout(Stdio::from(
        File::create(out_path).map_err(|e| anyhow!(e).context("Stdout file creation failed"))?,
      ));
      cmd.stderr(Stdio::from(
        File::create(err_path).map_err(|e| anyhow!(e).context("Stderr file creation failed"))?,
      ));
    }
    let mut test = cmd
      .spawn()
      .map_err(|e| anyhow!(e).context("spawn from command"))?;

    let _ = test.wait();
    // TODO decide:
    // fully parallel test execution VS costs of seeking/re-reading packet capture files
    // when two non-subsequent packets are requested
    // for parallel impl, uncomment the below and the 1st line in this fn
    //    tests.push(test);
  }
  // while !tests.iter_mut().all(|t| t.try_wait().is_ok()) {
  //   sleep(Duration::from_millis(300)).await;
  // }
  // additional sleep here to ensure that server processes all the incoming "test end" messages
  // before we "kill" it
  sleep(Duration::from_millis(300)).await;
  Ok(())
}

struct TestOutputPathGen {
  dir: bool,
  base: PathBuf,
}

impl TestOutputPathGen {
  pub fn new(base: Option<PathBuf>) -> Option<Self> {
    if let Some(base) = base {
      Self {
        dir: base.clone().is_dir(),
        base,
      }
      .into()
    } else {
      None
    }
  }

  pub fn get_out_path(&self, m: IntegralModId, f: IntegralFnId, id: u32) -> PathBuf {
    self.get_path(format!(
      "M{}-F{}-{}.out",
      m.hex_string(),
      f.hex_string(),
      id
    ))
  }

  pub fn get_err_path(&self, m: IntegralModId, f: IntegralFnId, id: u32) -> PathBuf {
    self.get_path(format!(
      "M{}-F{}-{}.err",
      m.hex_string(),
      f.hex_string(),
      id
    ))
  }

  fn get_path(&self, dir_append_variant: String) -> PathBuf {
    if self.dir {
      self.base.join(dir_append_variant)
    } else {
      self.base.clone()
    }
  }
}
