use std::collections::{HashMap, HashSet};

use args::Cli;
use call_tracing::{FunctionCallInfo, ModIdT};
use clap::Parser;
use log::Log;
use modmap::ExtModuleMap;
mod args;
mod call_tracing;
mod constants;
mod libc_wrappers;
mod log;
mod modmap;
mod shmem_capture;
use shmem_capture::{call_tracing::msg_handler, cleanup_shmem, deinit_tracing, init_tracing};

pub fn print_summary(freqs: &HashMap<FunctionCallInfo, u64>, mods: &ExtModuleMap) {
  let lg = Log::get("summary");
  let mut pairs = freqs.iter().collect::<Vec<(_, _)>>();
  let mut seen_modules: HashSet<ModIdT> = HashSet::new();
  pairs.sort_by(|a, b| b.1.cmp(a.1));
  for (idx, (fninfo, freq)) in pairs.iter().enumerate() {
    let modstr = mods.get_module_string_id(fninfo.module_id);
    let fn_name = mods.get_function_name(fninfo.module_id, fninfo.function_id);
    seen_modules.insert(fninfo.module_id);
    if modstr.and(fn_name).is_none() {
      lg.warn(format!(
        "Function ID or module ID confusion. Fun ID: {} {:?} Mod ID: {} {:?}",
        fninfo.function_id, fn_name, fninfo.module_id, modstr
      ));
      continue;
    }

    println!(
      "{idx} - {} - {freq} - {} (module {})",
      fninfo.function_id,
      fn_name.unwrap(),
      modstr.unwrap()
    );
  }
  mods.print_summary();
  println!("Total traced calls: {}", freqs.values().sum::<u64>());
  println!("Traces originated from {} modules", seen_modules.len());
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

  let modules = ExtModuleMap::try_from(cli.modmap.clone());
  if modules.is_err() {
    lg.crit(format!(
      "Could not parse module mapping from {}:\n{}",
      cli.modmap.to_string_lossy(),
      modules.unwrap_err()
    ));
    return Err("".to_owned());
  }

  let modules = modules.unwrap();
  let mut recorded_frequencies: HashMap<FunctionCallInfo, u64> = HashMap::new();

  match cli.stage {
    args::Stage::TraceCalls {
      buff_count,
      buff_size,
      out_file: _,
    } => {
      if cli.cleanup {
        lg.info("Cleanup");
        return cleanup_shmem(&cli.fd_prefix);
      }
      lg.info("Initializing semaphores");

      let mut tracing_infra = init_tracing(&cli.fd_prefix, buff_count, buff_size)?;

      lg.info("Listening!");

      if let Err(e) = msg_handler(
        &mut tracing_infra,
        buff_size as usize,
        buff_count as usize,
        &modules,
        &mut recorded_frequencies,
      ) {
        lg.crit(&e);
      }

      print_summary(&recorded_frequencies, &modules);

      lg.info("Exiting...");

      deinit_tracing(tracing_infra)?;
    }
    args::Stage::CaptureArgs {
      in_file: _,
      out_dir: _,
      mem_limit: _,
    } => todo!(),
  }

  Ok(())
}
