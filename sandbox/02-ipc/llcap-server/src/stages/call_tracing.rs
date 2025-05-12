use std::{collections::HashSet, fs::File, io::Write, path::PathBuf};

use crate::{constants::Constants, log::Log, modmap::ExtModuleMap};

pub type ModIdT = usize;

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
pub struct FunctionCallInfo {
  pub function_id: u32,
  pub module_id: ModIdT,
}

impl FunctionCallInfo {
  pub fn new(fn_id: u32, mod_id: ModIdT) -> Self {
    Self {
      function_id: fn_id,
      module_id: mod_id,
    }
  }
}

pub enum Message {
  Normal(FunctionCallInfo),
  ControlEnd,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LLVMFunId {
  pub fn_name: String,
  pub fn_module: String,
}

pub fn print_summary(sorted_pairs: &mut Vec<(&FunctionCallInfo, &u64)>, mods: &ExtModuleMap) {
  let lg = Log::get("summary");
  let mut seen_modules: HashSet<ModIdT> = HashSet::new();

  for (idx, (fninfo, freq)) in sorted_pairs.iter().enumerate() {
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
      "{idx} - {freq} - {} (module {})",
      fn_name.unwrap(),
      modstr.unwrap()
    );
  }
  mods.print_summary();
  println!(
    "Total traced calls: {}",
    sorted_pairs.iter_mut().map(|x| x.1).sum::<u64>()
  );
  println!("Traces originated from {} modules", seen_modules.len());
}

fn get_line_input() -> Result<String, std::io::Error> {
  let mut user_input = String::new();
  std::io::stdin().read_line(&mut user_input)?;
  Ok(user_input)
}

pub fn obtain_function_id_selection(
  ordered_traces: &[&FunctionCallInfo],
  mapping: &ExtModuleMap,
) -> Vec<LLVMFunId> {
  let lg = Log::get("obtain_function_ids");
  println!("Enter indicies of function to test (single line, numbers separated by spaces):");
  let user_input = get_line_input().expect("Error collecting input");

  let mut result = vec![];

  for inp in user_input.trim().split(' ').map(|v| v.trim()) {
    match inp.parse::<usize>() {
      Ok(i) => {
        if i < ordered_traces.len() {
          let FunctionCallInfo {
            function_id,
            module_id,
          } = ordered_traces[i];
          let fn_str = mapping.get_function_name(*module_id, *function_id);
          let mod_str = mapping.get_module_string_id(*module_id);

          match (fn_str, mod_str) {
            (None, _) | (_, None) => {
              lg.crit(format!("Skipping index {inp} - invalid fn-id or mod-id"));
            }
            (Some(fs), Some(ms)) => result.push(LLVMFunId {
              fn_name: fs.clone(),
              fn_module: ms.clone(),
            }),
          }
        } else {
          lg.crit(format!("Skipping index {inp} - out of range"));
        }
      }

      Err(e) => {
        lg.crit(format!("Could not parse the input: {}", e));
        return vec![];
      }
    }
  }

  result
}

pub fn export_tracing_selection(selection: &[LLVMFunId]) -> Result<(), String> {
  let lg = Log::get("export_tracing_selection");
  let default_path = Constants::default_selected_functions_path();
  println!(
    "Enter path (relative to working directory) where selected traces should be saved: ({default_path} is default)"
  );
  let user_input = get_line_input().expect("Error collecting input");
  let user_input = user_input.trim();

  let path = if user_input.is_empty() {
    lg.info("Using default path");
    PathBuf::from(default_path)
  } else {
    PathBuf::from(user_input)
  };

  if path.is_dir() {
    return Err("Path is a directory".to_string());
  }

  let file = File::create(&path);
  if let Ok(mut file) = file {
    for selected in selection {
      let to_write = format!("{}\x00{}\n", selected.fn_module, selected.fn_name);
      let expected_len = selected.fn_module.len() + selected.fn_name.len() + 2;

      let wr_res = file.write(to_write.as_bytes());
      if let Ok(written_bytes) = wr_res {
        if expected_len != written_bytes {
          lg.crit("Something went wrong when writing the file - byte count mismatch");
        }
      } else {
        lg.crit(format!(
          "Something went wrong when writing the file {:?}",
          wr_res.err().unwrap()
        ));
      }
    }

    lg.info(format!("Successfully exported to {:?}", path));
    Ok(())
  } else {
    Err(format!("Could not open for writing: {:?}", path))
  }
}
