use std::collections::{HashMap, HashSet};

use args::Cli;
use call_tracing::{FunctionCallInfo, ModIdT};
use clap::Parser;
use log::Log;
use modmap::ExtModuleMap;
use shmem_capture::{
  ShmHandle, clean_semaphores, deinit_semaphores, deinit_shmem, init_semaphores, init_shmem,
  msg_handler,
};
use zmq_capture::zmq_call_trace;
mod args;
mod call_tracing;
mod constants;
mod log;
mod modmap;
mod shmem_capture;
mod zmq_capture;

pub fn print_summary(freqs: &HashMap<FunctionCallInfo, u64>, mods: &ExtModuleMap) {
  let mut pairs = freqs.iter().collect::<Vec<(_, _)>>();
  let mut seen_modules: HashSet<ModIdT> = HashSet::new();
  pairs.sort_by(|a, b| b.1.cmp(a.1));
  for (idx, (fninfo, freq)) in pairs.iter().enumerate() {
    let modstr = mods.get_module_string_id(fninfo.module_id);
    let fn_name = mods.get_function_name(fninfo.module_id, fninfo.function_id);
    seen_modules.insert(fninfo.module_id);
    if modstr.and(fn_name).is_none() {
      eprintln!(
        "Warn: function id or module id confusion with fnid: {} moid: {}",
        fninfo.function_id, fninfo.module_id
      );
      continue;
    }

    println!(
      "{idx} - {freq} - {} (module {})",
      fn_name.unwrap(),
      modstr.unwrap()
    );
  }
  mods.print_summary();
  println!("Total traced calls: {}", freqs.values().sum::<u64>());
  println!("Traces originated from {} modules", seen_modules.len());
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), String> {
  let cli = Cli::try_parse();
  if let Err(e) = cli {
    eprintln!("{}", e);
    return Err("".to_owned());
  }
  let cli = cli.unwrap();
  Log::set_verbosity(cli.verbose);
  let lg = Log::get("main");
  lg.info(&format!("Verbosity: {}", cli.verbose));

  let modules = ExtModuleMap::try_from(cli.modmap.clone());
  if modules.is_err() {
    lg.crit(&format!(
      "Could not parse module mapping from {}:\n{}",
      cli.modmap.to_string_lossy(),
      modules.unwrap_err()
    ));
    return Err("".to_owned());
  }

  let modules = modules.unwrap();
  let mut recorded_frequencies: HashMap<FunctionCallInfo, u64> = HashMap::new();

  match cli.method {
    args::Type::ZeroMQ { socket } => {
      zmq_call_trace(&socket, &modules, &mut recorded_frequencies).await;
      print_summary(&recorded_frequencies, &modules);
    }
    args::Type::Shmem {
      fd_prefix,
      buff_count,
      buff_size,
      cleanup,
    } => {
      if cleanup {
        lg.info("Cleanup");
        return clean_semaphores(&fd_prefix);
      }
      lg.info("Initializing semaphores");
      let (mut semfree, mut semfull) = init_semaphores(&fd_prefix, buff_count)?;
      lg.info("Initializing shmem");
      let (meta_shm, buffers_shm) = match init_shmem(&fd_prefix, buff_count, buff_size) {
        Err(e) => {
          match deinit_semaphores(semfree, semfull) {
            // both arms MUST return Err! or the unreachable! macro below panics!
            Ok(()) => Err(e),
            Err(e2) => Err(format!(
              "Failed to clean up semaphores when mmap failed: {e2}, map failure: {e}"
            )),
          }?; // <-- unreachable does not panic if both arms return an Err

          unreachable!(
            "The above uses ? operator on an ensured Err variant (returned in both match arms)"
          );
        }
        Ok(a) => Ok::<(ShmHandle, ShmHandle), String>(a),
      }?;

      lg.info("Listening!");

      if let Err(e) = msg_handler(
        &mut semfull,
        &mut semfree,
        &buffers_shm,
        buff_size as usize,
        buff_count as usize,
        &modules,
        &mut recorded_frequencies,
      ) {
        eprintln!("{e}");
      }
      print_summary(&recorded_frequencies, &modules);

      lg.info("Exiting...");

      let shm_uninit = deinit_shmem(meta_shm, buffers_shm);
      let sem_uninit = deinit_semaphores(semfree, semfull);

      let goodbye_errors = [shm_uninit, sem_uninit]
        .iter()
        .fold("".to_string(), |acc, v| {
          if v.is_err() {
            acc + v.as_ref().unwrap_err()
          } else {
            acc
          }
        });
      if !goodbye_errors.is_empty() {
        return Err(format!("Failed deinit! {goodbye_errors}"));
      }
    }
  }

  Ok(())
}
