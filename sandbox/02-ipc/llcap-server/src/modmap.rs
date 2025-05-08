use std::{collections::HashMap, fs, path::PathBuf};

use crate::Log;
use crate::constants::Constants;

#[derive(Debug)]
pub struct FunctionMap {
  fnid_to_demangled_name: HashMap<u32, String>,
}

impl FunctionMap {
  pub fn new(values: &[(u32, String)]) -> Self {
    Self {
      fnid_to_demangled_name: HashMap::from_iter(values.to_owned()),
    }
  }

  pub fn insert(&mut self, id: u32, name: String) -> Option<String> {
    self.fnid_to_demangled_name.insert(id, name)
  }

  pub fn get(&self, id: u32) -> Option<&String> {
    self.fnid_to_demangled_name.get(&id)
  }
}

fn parse_fn_id_pair(inp: &[&[u8]]) -> Result<(u32, String), String> {
  if inp.len() != 2 {
    return Err("Invalid ID format, expecting two items per line".into());
  }

  let try_name = String::from_utf8(inp[0].to_vec());
  if let Err(e) = try_name {
    return Err(e.to_string());
  }

  let try_num = String::from_utf8(inp[1].to_vec())
    .map_err(|e| e.to_string())
    .and_then(|v| {
      u32::from_str_radix(&v, Constants::parse_fnid_radix()).map_err(|e| e.to_string())
    });

  try_num.map(|id| (id, try_name.unwrap()))
}

impl From<&[&[u8]]> for FunctionMap {
  fn from(value: &[&[u8]]) -> Self {
    let newline_split = value.iter().filter(|v| !v.is_empty());
    let zero_splits: Vec<Vec<&[u8]>> = newline_split
      .map(|v: &&[u8]| v.split(|v| *v == 0x0).collect::<Vec<&[u8]>>())
      .filter(|v| !v.is_empty())
      .collect();

    let parsed_pairs = zero_splits
      .iter()
      .map(|split_pair| parse_fn_id_pair(split_pair));

    let mut target = Self::new(&[]);

    let lg = Log::get("FunctionMap::From(..bytes..)");
    for pair_res in parsed_pairs {
      match pair_res {
        Err(e) => lg.warn(format!("Could not parse function ID pair: {e}")),
        Ok((id, name)) => {
          if let Some(old_name) = target.insert(id, name.clone()) {
            lg.warn(format!(
              "Duplicate function ID, this should not happen within a module! Function ID: {}, name 1: {}, name 2: {}",
              id, old_name, name
            ));
          }
        }
      }
    }

    target
  }
}

#[derive(Hash, Debug, PartialEq, Eq)]
pub struct IntegralModId(pub u32);

pub type RcvdModId = IntegralModId;
pub const MOD_ID_SIZE_B: usize = std::mem::size_of_val(&(IntegralModId(0)).0);
pub const MOD_NAME_SIZE: usize = MOD_ID_SIZE_B * 2;

impl TryFrom<&str> for IntegralModId {
  type Error = String;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    if value.chars().count() != MOD_NAME_SIZE {
      return Err("Invalid size".to_string());
    }

    let mut inner: u32 = 0;
    for v in value.chars() {
      inner <<= 4; // in order to not "over shift" in the last iteration

      if !v.is_ascii() {
        return Err(format!("Invalid module id (ascii): {value}"));
      }

      let v = v.to_ascii_uppercase();
      let num_val = match v {
        '0'..='9' => v as u32 - '0' as u32,
        'A'..='F' => v as u32 - 'A' as u32 + 10,
        _ => return Err(format!("Invalid module id (char): {value}")),
      };

      inner += num_val;
    }
    Ok(Self(inner))
  }
}

#[derive(Debug)]
pub struct ExtModuleMap {
  modhash_to_modidx: HashMap<RcvdModId, usize>,
  function_ids: Vec<FunctionMap>,
  module_paths: Vec<String>,
}

impl Default for ExtModuleMap {
  fn default() -> Self {
    Self::new()
  }
}

impl ExtModuleMap {
  pub fn new() -> Self {
    Self {
      function_ids: vec![],
      modhash_to_modidx: HashMap::new(),
      module_paths: vec![],
    }
  }

  pub fn add_module(&mut self, path_to_modfile: &PathBuf) -> Result<(), String> {
    let index = self.function_ids.len();
    let modhash = if let Some(hash_res) = path_to_modfile
      .file_name()
      .filter(|p| p.len() == MOD_NAME_SIZE)
      .and_then(|v| v.to_str())
      .and_then(|v| IntegralModId::try_from(v).into())
    {
      hash_res
    } else {
      Err(format!("Invalid path {:?}", path_to_modfile))
    }?;

    if self.modhash_to_modidx.contains_key(&modhash) {
      return Err("Duplicate module hash!".to_owned());
    }

    let contents = fs::read(path_to_modfile).map_err(|e| {
      format!(
        "Failed to read file {}: {}",
        path_to_modfile.to_string_lossy(),
        e
      )
    })?;
    let lines: Vec<&[u8]> = contents.split(|x| x == &0xa).collect();

    let (module_str_id, fn_map) = if let Some((head, tail)) = lines.split_first() {
      String::from_utf8(head.to_vec())
        .map_err(|e| {
          format!(
            "Could not parse string id of a module (path): {:?}, error: {}",
            head, e
          )
        })
        .map(|v| (v, tail))
    } else {
      Err("Empty module file".to_owned())
    }?;

    let fn_map = FunctionMap::from(fn_map);

    self.modhash_to_modidx.insert(modhash, index);
    self.function_ids.push(fn_map);
    self.module_paths.push(module_str_id);
    Ok(())
  }

  pub fn get_module_id(&self, hash: &RcvdModId) -> Option<usize> {
    self.modhash_to_modidx.get(hash).copied()
  }

  pub fn get_function_name(&self, mod_id: usize, fn_id: u32) -> Option<&String> {
    if mod_id >= self.function_ids.len() {
      None
    } else if let Some(fn_name) = self.function_ids[mod_id].get(fn_id) {
      Some(fn_name)
    } else {
      None
    }
  }

  pub fn get_module_string_id(&self, mod_id: usize) -> Option<&String> {
    self.module_paths.get(mod_id)
  }

  pub fn print_summary(&self) {
    println!("Module map summary:");
    println!("Total Modules loaded: {}", self.modhash_to_modidx.len());
    println!(
      "Total Functions loaded: {}",
      self
        .function_ids
        .iter()
        .map(|fnids| fnids.fnid_to_demangled_name.len())
        .sum::<usize>()
    );
  }
}

impl TryFrom<PathBuf> for ExtModuleMap {
  type Error = String;

  fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
    if !path.exists() || !path.is_dir() {
      return Err(format!("{} is not a directory", path.to_string_lossy()));
    }

    let mut target = ExtModuleMap::new();

    let dir = std::fs::read_dir(&path)
      .map_err(|x| format!("Cannot open directory {}: {}", path.to_string_lossy(), x))?;

    for file in dir {
      let res = match file {
        Err(e) => Err(format!("Module file could not be listed: {}", e)),
        Ok(entry) => target.add_module(&entry.path()),
      };

      if let Err(e) = res {
        Log::get("ExtmoduleMap::try_from(PathBuff)").warn(format!("Failed to read module: {}", e));
      }
    }

    Ok(target)
  }
}
