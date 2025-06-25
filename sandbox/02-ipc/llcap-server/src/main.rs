use std::{
  fs::File,
  ops::DerefMut,
  os::unix::process::ExitStatusExt,
  path::PathBuf,
  process::{Command, Stdio},
  sync::{Arc, Mutex},
  time::Duration,
};

use anyhow::{Context, Result, anyhow, bail, ensure};
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
  call_tracing::perform_call_tracing,
  cleanup,
  hooklib_commons::{META_MEM_NAME, META_SEM_ACK, META_SEM_DATA},
  send_arg_capture_metadata, send_test_metadata,
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
  shmem_capture::{
    FinalizerInfraInfo, InfraParams, TestParams, TracingInfra, send_call_tracing_metadata,
  },
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

/// sets up a thread that monitors the child in the background
///
/// the monitor returns a oneshot receiver which will be filled with a boolean flag indicating success or failure of the monitor initialization, the monitor's join handle is also returned
///
/// the monitor performs the IPC finalizing sequence when a program crashes (is terminated by a signal)
///
async fn spawn_process_monitor(
  mut child: std::process::Child,
  fin_info: FinalizerInfraInfo,
) -> (
  tokio::sync::oneshot::Receiver<bool>,
  tokio::task::JoinHandle<()>,
) {
  let (monitor_ok_tx, monitor_ok_rx) = tokio::sync::oneshot::channel::<bool>();
  // tokio::spawn ensures the monitor runs without an await
  let child_monitor = tokio::spawn(async move {
    let lg = Log::get("child_monitor");
    lg.trace("child monitor launched");
    // attaches to the crucial semaphore
    let fnlzr_infra = match fin_info.try_open() {
      Ok(infra) => infra,
      Err(e) => {
        lg.crit(format!("Failed to init finalizer... manual cleanup most likely required, please terminate llcap-server and perform cleanup (--cleanup)\nError: {}", e));
        // if the send call is Err => the other end of the channel hung up (unreasonable to react here)
        let _ = monitor_ok_tx.send(false);
        return;
      }
    };
    let _ = monitor_ok_tx.send(true);

    // polls with a delay until child exits
    // if exits by signal, artificially terminates the connection via fnlzr_infra
    loop {
      match child.try_wait() {
        Ok(Some(code)) => {
          if let Some(sig) = code.signal() {
            lg.progress(format!("App terminated by signal {}", sig));
            // wait just to be absolutely sure
            std::thread::sleep(Duration::from_millis(300));
            let _ = fnlzr_infra.finalization_flush().inspect_err(|e| lg.crit(format!("Failed to finalize comms... manual cleanup most likely required, please terminate llcap-server and perform cleanup (--cleanup)\nError: {}", e)));
            break;
          } else {
            lg.progress(format!("App terminated with exit code {:?}", code.code()));
            break;
          }
        }
        Ok(_) => (),
        Err(e) => {
          lg.crit(e.to_string());
          break;
        }
      }
      std::thread::sleep(Duration::from_millis(300));
    }
    lg.trace("child monitor stopped");
  });

  (monitor_ok_rx, child_monitor)
}

/// a generic driver function that wraps around the launch of the requested
/// application & the monitoring thread
///
/// cmd - command to be executed as a child process
/// meta_sender - function that performs metadata sending (according to capture type)
/// capture - function performing the capture (more like a closure that wraps the real invokation)
///
/// the function simply abstracts away the child monitor thread
async fn drive_instrumented_application<MetadataSender, CaptureHandler, R>(
  mut cmd: Command,
  finalizer_info: FinalizerInfraInfo,
  metadata_svr: Arc<Mutex<MetadataPublisher>>,
  meta_sender: MetadataSender,
  capture: CaptureHandler,
  infra_params: InfraParams,
) -> Result<R>
where
  CaptureHandler: FnOnce() -> Result<R>,
  MetadataSender: FnOnce(&mut MetadataPublisher, InfraParams) -> Result<()>,
{
  let lg = Log::get("app_driver");

  {
    // do not hold accorss awaits
    let mut guard = metadata_svr.lock().unwrap();
    meta_sender(guard.deref_mut(), infra_params)?;
  }

  let spawned_child: std::process::Child = cmd
    .spawn()
    .map_err(|e| anyhow!(e).context("Failed to spawn from command"))?;

  let (monitor_ready_rx, child_monitor) =
    spawn_process_monitor(spawned_child, finalizer_info).await;
  lg.trace("Waiting for monitor start");
  match timeout(Duration::from_secs(10), monitor_ready_rx).await {
    Ok(Ok(val)) => ensure!(val, "Monitor NOT ready"),
    Err(_) => bail!("Monitor ready timeout"),
    Ok(Err(e)) => bail!("monitor ready error: {}", e),
  }

  lg.progress("Monitor ready, listening!");

  let res = capture()?;

  child_monitor
    .await
    .map_err(|e| anyhow!("Child monitor join {:?}", e))?;
  Ok(res)
}

// shorthands for the creation of the MetadataPublisher

fn create_meta_svr(
  mem_cstr: &std::ffi::CStr,
  sem_str: String,
  ack_str: String,
) -> Result<Arc<Mutex<MetadataPublisher>>> {
  Ok(Arc::new(Mutex::new(
    MetadataPublisher::new(mem_cstr, &sem_str, &ack_str).context("cleanup required")?,
  )))
}

fn try_meta_svr_arc_deinit(metadata_svr: Arc<Mutex<MetadataPublisher>>) -> Result<()> {
  Arc::try_unwrap(metadata_svr).map_or(
    Err(anyhow!("Failed to unwrap from arc... this is not expected")),
    |ms| ms.into_inner().unwrap().deinit(),
  )
}

#[tokio::main()]
async fn main() -> Result<()> {
  let lg = Log::get("main");
  let cli = Cli::try_parse()?;
  Log::set_verbosity(cli.verbose);
  lg.progress(format!("Verbosity: {}", cli.verbose));

  if cli.cleanup {
    lg.progress("Cleanup");
    return cleanup(&cli.fd_prefix);
  }

  let mut modules = obtain_module_map(&cli.modmap)?;

  let (buff_count, buff_size) = (cli.buff_count, cli.buff_size);
  let mem_cstr = std::ffi::CStr::from_bytes_with_nul(META_MEM_NAME)?;
  let sem_str = String::from_utf8(META_SEM_DATA.split_last().unwrap().1.to_vec())?;
  let ack_str = String::from_utf8(META_SEM_ACK.split_last().unwrap().1.to_vec())?;

  match cli.stage {
    args::Stage::TraceCalls {
      mut out_file,
      import_path,
      selection_path,
      command,
    } => {
      if import_path.is_some() {
        out_file = None;
      }

      let mut pairs = if let Some(in_path) = import_path {
        lg.trace("Importing");
        let result = import_call_trace_data(in_path, &modules)?;
        lg.progress("Import done");
        result
      } else {
        let command = command.ok_or(anyhow!(
          "command must be present if import_path is not specified"
        ))?;

        lg.progress("Initializing tracing infrastructure");
        let (mut tracing_infra, finalizer_info) =
          TracingInfra::try_new(&cli.fd_prefix, buff_count, buff_size)?;
        let metadata_svr = create_meta_svr(mem_cstr, sem_str, ack_str)?;
        let infra_params = InfraParams {
          buff_count: tracing_infra.buffer_count() as u32,
          buff_len: tracing_infra.buffer_size() as u32,
        };

        let result = drive_instrumented_application(
          cmd_from_args(&command)?,
          finalizer_info,
          metadata_svr.clone(),
          send_call_tracing_metadata,
          || perform_call_tracing(&mut tracing_infra, &modules),
          infra_params,
        )
        .await
        .map(|freqs| freqs.into_iter().collect::<Vec<(_, _)>>());

        lg.progress("Shutting down tracing infrastructure...");
        // we simply chain Results together to perform cleanup, in edge cases, even despite this effort, --cleanup is still requried
        let real_result = tracing_infra
          .deinit()
          .inspect_err(|e| lg.crit(format!("You might need to perform cleanup: {e}")))
          .map(|_| result)?;

        // this should not really fail unless metadata_svr is cloned and persisted somewhere it should not be (i.e we should be the sole owners of metadata_svr here)
        try_meta_svr_arc_deinit(metadata_svr)?;
        real_result?
      };

      pairs.sort_by(|a, b| b.1.cmp(&a.1));
      print_call_tracing_summary(&mut pairs, &modules);

      if let Some(out_path) = out_file {
        lg.trace("Exporting");

        let _ = export_call_trace_data(&pairs, out_path)
          .inspect_err(|e| lg.crit(format!("Export failed: {e}")));

        lg.progress("Export done");
      }

      let traces = pairs.iter().map(|x| x.0).collect::<Vec<NumFunUid>>();
      let selected_fns = loop {
        let sel = obtain_function_id_selection(&traces, &modules);
        if let Ok(selection) = sel {
          break selection;
        } else {
          lg.crit(sel.unwrap_err().to_string());
        }
      };
      export_tracing_selection(&selected_fns, &modules, selection_path)?;
    }
    args::Stage::CaptureArgs {
      selection_file,
      out_dir,
      mem_limit,
      command,
    } => {
      lg.progress("Reading function selection");
      let selection = import_tracing_selection(&selection_file)?;

      lg.progress("Masking");
      modules.mask_include(&selection)?;

      lg.progress("Setting up function packet dumping");
      let mut dumper = ArgPacketDumper::new(&out_dir, &modules, mem_limit as usize)?;
      lg.progress("Initializing tracing infrastructure");
      let (mut tracing_infra, finalizer_info) =
        TracingInfra::try_new(&cli.fd_prefix, buff_count, buff_size)?;
      let metadata_svr = create_meta_svr(mem_cstr, sem_str, ack_str)?;
      let infra_params = InfraParams {
        buff_count: tracing_infra.buffer_count() as u32,
        buff_len: tracing_infra.buffer_size() as u32,
      };

      // for comments, see the match arm for the TraceCalls subcommand
      let result = drive_instrumented_application(
        cmd_from_args(&command)?,
        finalizer_info,
        metadata_svr.clone(),
        send_arg_capture_metadata,
        || perform_arg_capture(&mut tracing_infra, &modules, &mut dumper),
        infra_params,
      )
      .await;

      lg.progress("Shutting down tracing infrastructure...");
      let real_result = tracing_infra
        .deinit()
        .inspect_err(|e| lg.crit(format!("You might need to perform cleanup: {e}")))
        .map(|_| result);
      try_meta_svr_arc_deinit(metadata_svr)?;
      let _ = real_result?;
    }
    args::Stage::Test {
      selection_file,
      capture_dir,
      mem_limit,
      test_output,
      command,
    } => {
      let command = Arc::new(command);
      lg.progress("Reading function selection");
      let selection = import_tracing_selection(&selection_file)?;
      lg.progress("Masking");
      modules.mask_include(&selection)?;

      let modules = Arc::new(modules);
      lg.progress("Setting up function packet reader");

      let packet_reader = PacketReader::new(&capture_dir, &modules, mem_limit as usize)
        .map_err(|e| anyhow!(e).context("Packet reader setup failed"))?;

      lg.progress("Setting up function packet server");
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
      let metadata_svr = create_meta_svr(mem_cstr, sem_str, ack_str)?;

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

          lg.progress(format!(
            "Run program for fn m: {} f: {}",
            module.hex_string(),
            function.hex_string()
          ));

          let test_job = tokio::spawn(test_job(
            metadata_svr.clone(),
            InfraParams {
              buff_count,
              buff_len: buff_size,
            },
            NumFunUid {
              function_id: *function,
              module_id: *module,
            },
            arg_count,
            test_count,
            command.clone(),
            output_gen.clone(),
          ));

          futs.push(test_job);
        }
      }
      lg.progress("Waiting for jobs to finish...");
      for fut in futs {
        fut.await??;
      }
      lg.progress("Waiting for server to exit...");
      let defer_res_end_svr = end_tx.send(()).map_err(|_| anyhow!("failed to end server"));
      let defer_res_joins = svr.await.map_err(|e| anyhow!(e).context("joins"));

      lg.progress("---------------------------------------------------------------");
      let mut results = results.lock().unwrap();
      lg.progress(format!("Test results ({}): ", results.len()));
      lg.progress("Module ID | Function ID |  Call  | Packet | Result");
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
        lg.progress(s);
      }
      lg.progress("---------------------------------------------------------------");
      lg.trace("Cleaning up");
      defer_res_end_svr?;
      defer_res_joins??;
      try_meta_svr_arc_deinit(metadata_svr)?;
    }
  }
  lg.progress("Exiting...");
  Ok(())
}

async fn test_job(
  metadata_svr: Arc<Mutex<MetadataPublisher>>,
  infra_params: InfraParams,
  fn_uid: NumFunUid,
  arg_count: u32,
  test_count: u32,
  command: Arc<Vec<String>>,
  output_gen: Arc<Option<TestOutputPathGen>>,
) -> Result<()> {
  let (m, f) = (fn_uid.module_id, fn_uid.function_id);
  // let mut tests = vec![];
  for call_idx in 0..test_count {
    {
      let mut guard = metadata_svr.lock().unwrap();
      send_test_metadata(
        guard.deref_mut(),
        infra_params.clone(),
        NumFunUid {
          function_id: f,
          module_id: m,
        },
        TestParams {
          arg_count,
          test_count,
          target_call_number: call_idx + 2,
        },
      )?;
    }
    let mut cmd = cmd_from_args(&command)?;
    if let Some(output_gen) = output_gen.as_ref() {
      let out_path = output_gen.get_out_path(m, f, call_idx + 1);
      let err_path = output_gen.get_err_path(m, f, call_idx + 1);
      cmd.stdout(Stdio::from(File::create(out_path.clone()).map_err(
        |e| anyhow!(e).context(format!("Stdout file creation failed: {:?}", out_path)),
      )?));
      cmd.stderr(Stdio::from(File::create(err_path).map_err(|e| {
        anyhow!(e).context(format!("Stderr file creation failed {:?}", out_path))
      })?));
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
