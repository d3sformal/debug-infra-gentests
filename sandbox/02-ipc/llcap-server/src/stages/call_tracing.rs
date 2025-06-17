use anyhow::{Result, anyhow, bail, ensure};
use std::{
  collections::HashSet,
  fs::File,
  io::{self, BufRead, BufReader, Write},
  path::{Path, PathBuf},
};

use crate::{
  constants::Constants,
  log::Log,
  modmap::{ExtModuleMap, IntegralFnId, IntegralModId, NumFunUid, TextFunUid},
};

pub enum Message {
  Normal(NumFunUid),
  ControlEnd,
}

pub fn print_summary(sorted_pairs: &mut [(NumFunUid, u64)], mods: &ExtModuleMap) {
  let lg = Log::get("summary");
  let mut seen_modules: HashSet<IntegralModId> = HashSet::new();

  for (idx, (fninfo, freq)) in sorted_pairs.iter().enumerate() {
    let modstr = mods.get_module_string_id(fninfo.module_id);
    let fn_name = mods.get_function_name(fninfo.module_id, fninfo.function_id);
    seen_modules.insert(fninfo.module_id);
    if modstr.and(fn_name).is_none() {
      lg.warn(format!(
        "Function ID or module ID confusion. Fun ID: {} {:?} Mod ID: {} {:?}",
        *fninfo.function_id, fn_name, *fninfo.module_id, modstr
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

fn get_line_input() -> Result<String> {
  let mut user_input = String::new();
  std::io::stdin().read_line(&mut user_input)?;
  Ok(user_input)
}

pub fn obtain_function_id_selection(
  ordered_traces: &[NumFunUid],
  mapping: &ExtModuleMap,
) -> Vec<TextFunUid> {
  let lg = Log::get("obtain_function_ids");
  println!("Enter indicies of function to test (single line, numbers separated by spaces):");
  let user_input = get_line_input().expect("Error collecting input");

  let mut result = vec![];

  for inp in user_input.trim().split(' ').map(|v| v.trim()) {
    match inp.parse::<usize>() {
      Ok(i) => {
        if i < ordered_traces.len() {
          let NumFunUid {
            function_id,
            module_id,
          } = ordered_traces[i];
          let fn_str = mapping.get_function_name(module_id, function_id);
          let mod_str = mapping.get_module_string_id(module_id);

          match (fn_str, mod_str) {
            (None, _) | (_, None) => {
              lg.crit(format!("Skipping index {inp} - invalid fn-id or mod-id"));
            }
            (Some(fs), Some(ms)) => result.push(TextFunUid {
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

pub fn export_tracing_selection(selection: &[TextFunUid], mapping: &ExtModuleMap) -> Result<()> {
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

  ensure!(
    !path.is_dir(),
    "Path {} a directory",
    path.to_string_lossy()
  );

  let mut file = File::create(&path).map_err(|e| anyhow!(e).context("export_tracing_seleciton"))?;
  for selected in selection {
    let mod_hash = mapping.get_module_hash_by_name(&selected.fn_module);
    ensure!(
      mod_hash.is_some(),
      "Could not resolve module hash from {}",
      selected.fn_module
    );
    let mod_hash = mod_hash.unwrap();
    let fn_id = mapping.get_function_id(mod_hash, &selected.fn_name);
    ensure!(
      fn_id.is_some(),
      "Could not resolve id of function {}, mod: {}, mapping: {:?}",
      selected.fn_name,
      selected.fn_module,
      mapping
    );
    let fn_id = *fn_id.unwrap();

    let to_write = format!(
      "{}\x00{}\x00{}\x00{}\n",
      selected.fn_module, mod_hash.0, selected.fn_name, *fn_id
    )
    .into_bytes();

    let wr_res = file.write(&to_write)?;
    ensure!(
      wr_res == to_write.len(),
      "Exporting of selection written unexpected amount: {} compared to {}",
      wr_res,
      to_write.len()
    );
  }
  lg.info(format!("Successfully exported to {:?}", path));
  Ok(())
}

pub fn import_tracing_selection(path: &Path) -> Result<Vec<TextFunUid>> {
  let mut result = Vec::with_capacity(8);

  let mut f =
    BufReader::new(File::open(path).map_err(|e| anyhow!(e).context("import_tracing_seleciton"))?);

  let mut line = String::with_capacity(256);
  while let Ok(len) = f.read_line(&mut line) {
    if len == 0 {
      break;
    }

    let mut it = line.trim().split('\x00');
    let module = it.next();
    let function = it.nth(1);
    ensure!(module.is_some(), "Module name not found in {:?}", line);
    ensure!(function.is_some(), "Function name not found in {:?}", line);

    result.push(TextFunUid {
      fn_module: module.unwrap().to_string(),
      fn_name: function.unwrap().to_string(),
    });

    line.clear();
  }

  Ok(result)
}

pub type ImportFormat = Vec<(NumFunUid, u64)>;
pub type ExportFormat = ImportFormat;

pub fn export_data(sorted_data: &ExportFormat, out_path: PathBuf) -> Result<()> {
  let lg = Log::get("call_tracing::export_data");

  let mut f = File::create(&out_path)?;
  for (fninfo, freq) in sorted_data {
    let module_hash = fninfo.module_id;

    f.write_fmt(format_args!(
      "{}-{}-{}\n",
      freq, *fninfo.function_id, module_hash.0
    ))
    .map_err(|e| anyhow!(e).context("export_data"))?;
  }

  lg.info(format!("Exported call tracing data to {:?}", out_path));
  Ok(())
}

pub fn import_data(in_path: PathBuf, modmap: &ExtModuleMap) -> Result<ImportFormat> {
  let mut result = vec![];

  let f = File::open(&in_path)?;
  let mut reader = io::BufReader::new(f);
  let mut line = String::new();

  while let Ok(c) = reader.read_line(&mut line) {
    if c == 0 {
      break;
    } else if c == 1 {
      continue;
    }

    let split: Vec<&str> = line.trim().split('-').collect();
    ensure!(
      split.len() == 3,
      "Invalid import format, split len: {} of {}",
      split.len(),
      line
    );

    let parse_res: (Result<u64, _>, Result<u32, _>, Result<u32, _>) = match split.as_slice() {
      &[s1, s2, s3] => (s1.parse(), s2.parse(), s3.parse()),
      _ => unreachable!("Split must be of length 3!"),
    };
    line.clear();

    if let (Ok(fr), Ok(fnid), Ok(modhash)) = parse_res {
      let (fnid, mod_id) = (IntegralFnId(fnid), IntegralModId(modhash));
      ensure!(
        modmap
          .get_function_arg_size_descriptors(mod_id, fnid)
          .is_some(),
        "Function not found, module: {}, fn: {}",
        *mod_id,
        *fnid
      );
      result.push((NumFunUid::new(fnid, mod_id), fr));
    } else {
      bail!(
        "Failed to parse import, invalid format in one of the numbers {:?}",
        parse_res
      );
    }
  }

  Ok(result)
}
