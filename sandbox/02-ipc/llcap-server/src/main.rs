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
  arg_capture::wip_capture_args, call_tracing::msg_handler, cleanup_shmem, deinit_tracing,
  init_tracing,
};
use stages::call_tracing::{
  FunctionCallInfo, export_data, export_tracing_selection, import_data,
  obtain_function_id_selection, print_summary,
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

fn main() -> Result<(), String> {
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
    return cleanup_shmem(&cli.fd_prefix);
  }

  let modules = obtain_module_map(&cli.modmap)?;

  match cli.stage {
    args::Stage::TraceCalls {
      buff_count,
      buff_size,
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
        let mut tracing_infra = init_tracing(&cli.fd_prefix, buff_count, buff_size)?;

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

        let _ = export_data(&pairs, &modules, out_path)
          .inspect_err(|e| lg.crit(format!("Export failed: {e}")));

        lg.info("Export done");
      }

      let traces = pairs.iter().map(|x| x.0).collect::<Vec<FunctionCallInfo>>();
      let selected_fns = obtain_function_id_selection(&traces, &modules);
      export_tracing_selection(&selected_fns, &modules)?;

      lg.info("Exiting...");
    }
    args::Stage::CaptureArgs {
      buff_count,
      buff_size,
      in_file: _,
      out_dir: _,
      mem_limit: _,
    } => {
      lg.info("Initializing tracing infrastructure");
      let mut tracing_infra = init_tracing(&cli.fd_prefix, buff_count, buff_size)?;

      lg.info("Listening!");

      wip_capture_args(
        &mut tracing_infra,
        buff_size as usize,
        buff_count as usize,
        &modules,
      )?;

      lg.info("Shutting down tracing infrastructure...");
      deinit_tracing(tracing_infra)?;
    }
  }

  Ok(())
}
