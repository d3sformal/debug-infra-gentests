use std::{
  collections::HashSet,
  fs::File,
  io::{self, BufRead, BufReader, Write},
  path::{Path, PathBuf},
};

use crate::{
  constants::Constants,
  log::Log,
  modmap::{ExtModuleMap, IntegralFnId, IntegralModId, ModIdxT},
};

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
pub struct FunctionCallInfo {
  pub function_id: IntegralFnId,
  pub module_idx: ModIdxT,
}

impl FunctionCallInfo {
  pub fn new(fn_id: IntegralFnId, mod_idx: ModIdxT) -> Self {
    Self {
      function_id: fn_id,
      module_idx: mod_idx,
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

pub fn print_summary(sorted_pairs: &mut [(FunctionCallInfo, u64)], mods: &ExtModuleMap) {
  let lg = Log::get("summary");
  let mut seen_modules: HashSet<ModIdxT> = HashSet::new();

  for (idx, (fninfo, freq)) in sorted_pairs.iter().enumerate() {
    let modstr = mods.get_module_string_id(fninfo.module_idx);
    let fn_name = mods.get_function_name(fninfo.module_idx, fninfo.function_id);
    seen_modules.insert(fninfo.module_idx);
    if modstr.and(fn_name).is_none() {
      lg.warn(format!(
        "Function ID or module ID confusion. Fun ID: {} {:?} Mod ID: {} {:?}",
        *fninfo.function_id, fn_name, fninfo.module_idx, modstr
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
  ordered_traces: &[FunctionCallInfo],
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
            module_idx,
          } = ordered_traces[i];
          let fn_str = mapping.get_function_name(module_idx, function_id);
          let mod_str = mapping.get_module_string_id(module_idx);

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

pub fn export_tracing_selection(
  selection: &[LLVMFunId],
  mapping: &ExtModuleMap,
) -> Result<(), String> {
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
      let mod_hash = mapping.find_module_hash_by_name(&selected.fn_module);
      if mod_hash.is_none() {
        return Err(format!(
          "Could not resolve module hash from {}",
          selected.fn_module
        ));
      }
      let mod_hash = mod_hash.unwrap();
      let fn_id = mapping.get_function_id(
        mapping.get_module_idx(&mod_hash).unwrap(),
        &selected.fn_name,
      );
      if fn_id.is_none() {
        return Err(format!(
          "Could not resolve id of function {}, mod: {}, mapping: {:?}",
          selected.fn_name, selected.fn_module, mapping
        ));
      }
      let fn_id = *fn_id.unwrap();

      let to_write = format!(
        "{}\x00{}\x00{}\x00{}\n",
        selected.fn_module, mod_hash.0, selected.fn_name, *fn_id
      );

      let wr_res = file.write(to_write.as_bytes());
      if let Err(e) = wr_res {
        lg.crit(format!(
          "Something went wrong when writing the file {:?}",
          e
        ));
      }
    }

    lg.info(format!("Successfully exported to {:?}", path));
    Ok(())
  } else {
    Err(format!("Could not open for writing: {:?}", path))
  }
}

pub fn import_tracing_selection(path: &Path) -> Result<Vec<LLVMFunId>, String> {
  let mut result = Vec::with_capacity(8);

  let mut f = BufReader::new(File::open(path).map_err(|e| e.to_string())?);

  let mut line = String::with_capacity(256);
  while let Ok(len) = f.read_line(&mut line) {
    if len == 0 {
      break;
    }

    let mut it = line.trim().split('\x00');
    let module = it.next();
    let function = it.nth(1);
    if module.is_none() {
      return Err(format!("Module name not found in {:?}", line));
    } else if function.is_none() {
      return Err(format!("Function name not found in {:?}", line));
    }

    result.push(LLVMFunId {
      fn_module: module.unwrap().to_string(),
      fn_name: function.unwrap().to_string(),
    });

    line.clear();
  }

  Ok(result)
}

pub type ImportFormat = Vec<(FunctionCallInfo, u64)>;
pub type ExportFormat = ImportFormat;

pub fn export_data(
  sorted_data: &ExportFormat,
  modmap: &ExtModuleMap,
  out_path: PathBuf,
) -> Result<(), String> {
  let lg = Log::get("call_tracing::export_data");

  let mut f = File::create(&out_path).map_err(|e| format!("{}", e))?;
  for datum in sorted_data {
    let module_hash = modmap.get_module_hash_by_idx(datum.0.module_idx);
    if module_hash.is_none() {
      return Err("Invalid module mapping, internal structures incorrect".to_string());
    }
    let module_hash = module_hash.unwrap();

    f.write_fmt(format_args!(
      "{}-{}-{}\n",
      datum.1, *datum.0.function_id, module_hash.0
    ))
    .map_err(|e| format!("Write failed: {}", e))?;
  }

  lg.info(format!("Exported call tracing data to {:?}", out_path));
  Ok(())
}

pub fn import_data(in_path: PathBuf, modmap: &ExtModuleMap) -> Result<ImportFormat, String> {
  let mut result = vec![];

  let f = File::open(&in_path).map_err(|e| format!("{}", e))?;
  let mut reader = io::BufReader::new(f);
  let mut line = String::new();

  while let Ok(c) = reader.read_line(&mut line) {
    if c == 0 {
      break;
    } else if c == 1 {
      continue;
    }

    let split: Vec<&str> = line.trim().split('-').collect();
    if split.len() != 3 {
      return Err(format!(
        "Invalid import format, split len: {} of {}",
        split.len(),
        line
      ));
    }

    let parse_res: (Result<u64, _>, Result<u32, _>, Result<u32, _>) = match split.as_slice() {
      &[s1, s2, s3] => (s1.parse(), s2.parse(), s3.parse()),
      _ => unreachable!("Split must be of length 3!"),
    };
    line.clear();

    if let (Ok(fr), Ok(fnid), Ok(modhash)) = parse_res {
      if let Some(module_idx) = modmap.get_module_idx(&IntegralModId(modhash)) {
        result.push((FunctionCallInfo::new(IntegralFnId(fnid), module_idx), fr));
      } else {
        return Err(format!(
          "Failed to map module hash (0x{:X}) to a module index",
          modhash
        ));
      }
    } else {
      return Err(format!(
        "Failed to parse import, invalid format in one of the numbers {:?}",
        parse_res
      ));
    }
  }

  Ok(result)
}
