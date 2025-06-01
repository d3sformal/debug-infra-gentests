use std::{
  ops::DerefMut,
  process::Command,
  sync::{Arc, Mutex},
};

use args::Cli;
use clap::Parser;
use libc_wrappers::wrappers::to_cstr;
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
  cleanup, deinit_tracing,
  hooklib_commons::{META_MEM_NAME, META_SEM_ACK, META_SEM_DATA},
  init_tracing, send_arg_capture_metadata, send_call_tracing_metadata, send_test_metadata,
};
use stages::{
  arg_capture::{ArgPacketDumper, PacketReader},
  call_tracing::{
    FunctionCallInfo, export_data, export_tracing_selection, import_data, import_tracing_selection,
    obtain_function_id_selection, print_summary,
  },
  testing::test_server_job,
};

fn obtain_module_map(path: &std::path::PathBuf) -> Result<ExtModuleMap, String> {
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
      Err("".to_owned())
    }
  }
}

fn cmd_from_args(args: &[String]) -> Result<Command, String> {
  if args.is_empty() {
    Err("Command must be specified".to_string())
  } else {
    let mut cmd = std::process::Command::new(args.first().unwrap());
    cmd.args(args.iter().skip(1));
    Ok(cmd)
  }
}

#[tokio::main()]
async fn main() -> Result<(), String> {
  Log::set_verbosity(255);
  let lg = Log::get("main");
  let cli = Cli::try_parse();
  if let Err(e) = cli {
    lg.crit(format!("{}", e));
    return Err("".to_owned());
  }
  let cli = cli.unwrap();
  Log::set_verbosity(cli.verbose);
  lg.info(format!("Verbosity: {}", cli.verbose));

  if cli.cleanup {
    lg.info("Cleanup");
    return cleanup(&cli.fd_prefix);
  }

  let mut modules = obtain_module_map(&cli.modmap)?;

  let (buff_count, buff_size) = (cli.buff_count, cli.buff_size);
  let mem_cstr = String::from_utf8(META_MEM_NAME.to_vec()).map_err(|e| e.to_string())?;
  let sem_str =
    String::from_utf8(META_SEM_DATA.split_last().unwrap().1.to_vec()).map_err(|e| e.to_string())?;
  let ack_str =
    String::from_utf8(META_SEM_ACK.split_last().unwrap().1.to_vec()).map_err(|e| e.to_string())?;
  let metadata_svr = Arc::new(Mutex::new(MetadataPublisher::new(
    unsafe { to_cstr(&mem_cstr) },
    &sem_str,
    &ack_str,
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
        let result = import_data(in_path, &modules)?;
        lg.info("Import done");
        result
      } else {
        lg.info("Initializing tracing infrastructure");
        let mut tracing_infra = init_tracing(&cli.fd_prefix, buff_count, buff_size).await?;

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
            lg.crit(&e);
            Err(e)
          }
        };
        lg.info("Shutting down call tracing infrastructure...");
        deinit_tracing(tracing_infra)?;

        pairs?
      };

      pairs.sort_by(|a, b| b.1.cmp(&a.1));
      print_summary(&mut pairs, &modules);

      if let Some(out_path) = out_file {
        lg.trace("Exporting");

        let _ =
          export_data(&pairs, out_path).inspect_err(|e| lg.crit(format!("Export failed: {e}")));

        lg.info("Export done");
      }

      let traces = pairs.iter().map(|x| x.0).collect::<Vec<FunctionCallInfo>>();
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
      let mut dumper =
        ArgPacketDumper::new(&out_dir, &modules, mem_limit as usize).map_err(|x| x.to_string())?;

      lg.info("Initializing tracing infrastructure");
      let mut tracing_infra = init_tracing(&cli.fd_prefix, buff_count, buff_size).await?;

      let mut cmd = cmd_from_args(&command)?;
      let mut guard = metadata_svr.lock().unwrap();
      let meta_fut = send_arg_capture_metadata(guard.deref_mut(), buff_count, buff_size);

      let _ = cmd
        .spawn()
        .map_err(|e| format!("Failed to spawn from command: {e}"))?;
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
      deinit_tracing(tracing_infra)?;
    }
    args::Stage::Test {
      selection_file,
      capture_dir,
      mem_limit,
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
        .map_err(|e| format!("Packet reader setup failed {e}"))?;

      lg.info("Setting up function packet server");
      let (end_tx, end_rx) = tokio::sync::oneshot::channel::<()>();
      let (ready_tx, ready_rx) = tokio::sync::oneshot::channel::<()>();
      let results = Arc::new(Mutex::new(Vec::with_capacity(500)));
      let svr = tokio::spawn(test_server_job(
        cli.fd_prefix.clone(),
        capture_dir,
        modules.clone(),
        mem_limit as usize,
        end_rx,
        ready_tx,
        results.clone(),
      ));

      // wait for server to be ready
      ready_rx.await.map_err(|e| e.to_string())?;

      let mut futs = vec![];

      for module in modules.modules() {
        for function in modules.functions(*module).unwrap() {
          let test_count = packet_reader
            .get_test_count(*module, *function)
            .ok_or(format!(
              "Not found tests: {} {}",
              module.hex_string(),
              function.hex_string()
            ))?;
          let arg_count = packet_reader
            .get_arg_count(*module, *function)
            .ok_or(format!(
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

          let module = *module;
          let function = *function;
          let command = command.clone();
          let metadata_svr = metadata_svr.clone();
          let (buff_size, buff_count) = (buff_size, buff_count);
          let test_job = tokio::spawn(async move {
            {
              let mut guard = metadata_svr.lock().unwrap();
              send_test_metadata(
                guard.deref_mut(),
                buff_count,
                buff_size,
                module,
                function,
                arg_count,
                test_count,
              )?;
            }

            let mut test = cmd_from_args(&command)?
              .spawn()
              .map_err(|e| format!("Failed to spawn from command: {e}"))?;

            let _ = test.wait();
            Ok::<(), String>(())
          });

          futs.push(test_job);
        }
      }
      lg.info("Waiting for jobs to finish...");
      for fut in futs {
        fut.await.map_err(|e| e.to_string())??;
      }
      lg.info("Waiting for server to exit...");
      end_tx.send(()).map_err(|_| "failed to end server")?;
      svr.await.map_err(|e| e.to_string())??;
      lg.info("---------------------------------------------------------------");
      let mut results = results.lock().unwrap();
      lg.info(format!("Test results ({}): ", results.len()));
      results.sort_by(|a, b| a.2.cmp(&b.2));
      results.sort_by(|a, b| a.1.cmp(&b.1));
      results.sort_by(|a, b| a.0.cmp(&b.0));
      for result in results.iter() {
        let s = format!(
          "{} {} {} {:?}",
          result.0.hex_string(),
          result.1.hex_string(),
          result.2,
          result.3
        );
        lg.info(s);
      }
      lg.info("---------------------------------------------------------------");
    }
  }

  lg.info("Exiting...");
  Ok(())
}
