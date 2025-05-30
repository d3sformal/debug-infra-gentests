use std::{process::Command, thread::sleep, time::Duration};

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
  arg_capture::perform_arg_capture,
  call_tracing::msg_handler,
  cleanup, deinit_tracing, init_tracing, send_arg_capture_metadata, send_call_tracing_metadata,
  send_test_metadata,
  zmq_channels::{ZmqArgumentPacketServer, zmq_packet_chnl_name},
};
use stages::{
  arg_capture::{ArgPacketDumper, PacketReader},
  call_tracing::{
    FunctionCallInfo, export_data, export_tracing_selection, import_data, import_tracing_selection,
    obtain_function_id_selection, print_summary,
  },
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

#[tokio::main(flavor = "current_thread")]
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

        send_call_tracing_metadata(&cli.fd_prefix, buff_count, buff_size).await?;

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
      command
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
      
      let meta_fut = send_arg_capture_metadata(&cli.fd_prefix, buff_count, buff_size);

      let mut test = cmd
      .spawn()
      .map_err(|e| format!("Failed to spawn from command: {e}"))?;
      meta_fut.await?;

      lg.info("Listening!");

      perform_arg_capture(
        &mut tracing_infra,
        buff_size as usize,
        buff_count as usize,
        &modules,
        &mut dumper,
      )?;

      if test.try_wait().is_err() {
        lg.warn("child did not exit, waiting 5s");
        sleep(Duration::from_secs(5));
      }

      lg.info("Shutting down tracing infrastructure...");
      deinit_tracing(tracing_infra)?;
    }
    args::Stage::Test {
      selection_file,
      capture_dir,
      mem_limit,
      command,
    } => {
      lg.info("Reading function selection");
      let selection = import_tracing_selection(&selection_file)?;
      lg.info("Masking");
      modules.mask_include(&selection)?;
      lg.info("Setting up function packet reader");

      let mut packet_reader = PacketReader::new(&capture_dir, &modules, mem_limit as usize)
        .map_err(|e| format!("Packet reader setup failed {e}"))?;

      lg.info("Setting up function packet server");
      let mut svr = ZmqArgumentPacketServer::new(&zmq_packet_chnl_name(&cli.fd_prefix)).await?;

      let mut cmd = cmd_from_args(&command)?;

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

          let meta_fut = send_test_metadata(
            &cli.fd_prefix,
            buff_count,
            buff_size,
            *module,
            *function,
            arg_count,
            test_count,
          );

          let mut test = cmd
            .spawn()
            .map_err(|e| format!("Failed to spawn from command: {e}"))?;
          meta_fut.await?;

          while let Some(data) = packet_reader.read_next_packet(*module, *function)? {
            lg.info(format!("Sending\n{} {:?}", &data.len(), data));
            svr.process_msg(Some(data)).await?;
          }

          if test.try_wait().is_err() {
            lg.warn("child did not exit, waiting 5s");
            sleep(Duration::from_secs(5));
          }
        }
      }
    }
  }
  
  lg.info("Exiting...");
  Ok(())
}
