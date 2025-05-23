use std::collections::HashSet;
use std::ops::Deref;
use std::{collections::HashMap, fs, path::PathBuf};

use num_traits::Num;

use crate::Log;
use crate::constants::Constants;
use crate::sizetype_handlers::ArgSizeTypeRef;
use crate::stages::call_tracing::LLVMFunId;

#[derive(Debug)]
pub struct FunctionMap {
  fnid_to_demangled_name: HashMap<IntegralFnId, String>,
  demangled_name_to_fnid: HashMap<String, IntegralFnId>,
  fnid_to_argument_sizes: HashMap<IntegralFnId, Vec<ArgSizeTypeRef>>,
}

impl FunctionMap {
  pub fn new(
    values: &[(IntegralFnId, String)],
    readers: HashMap<IntegralFnId, Vec<ArgSizeTypeRef>>,
  ) -> Self {
    Self {
      fnid_to_argument_sizes: readers,
      demangled_name_to_fnid: HashMap::from_iter(values.iter().map(|(x, y)| (y.clone(), *x))),
      fnid_to_demangled_name: HashMap::from_iter(values.to_owned()),
    }
  }

  pub fn is_empty(&self) -> bool {
    self.fnid_to_argument_sizes.is_empty()
  }

  pub fn functions(
    &self,
  ) -> std::collections::hash_map::Keys<'_, IntegralFnId, std::string::String> {
    self.fnid_to_demangled_name.keys()
  }

  pub fn get_name(&self, id: IntegralFnId) -> Option<&String> {
    self.fnid_to_demangled_name.get(&id)
  }

  pub fn get_arg_size_ref(&self, id: IntegralFnId) -> Option<&Vec<ArgSizeTypeRef>> {
    self.fnid_to_argument_sizes.get(&id)
  }

  pub fn get_id(&self, name: &String) -> Option<&IntegralFnId> {
    self.demangled_name_to_fnid.get(name)
  }

  pub fn mask_include(&mut self, fn_ids: &HashSet<IntegralFnId>) -> Result<(), String> {
    let counter_ids = self
      .fnid_to_argument_sizes
      .keys()
      .filter(|k| !fn_ids.contains(k))
      .cloned()
      .collect::<Vec<_>>();
    let lg = Log::get("mask_include");
    for counter_id in counter_ids {
      let expected_name = match self.fnid_to_demangled_name.remove(&counter_id) {
        Some(x) => Ok(x),
        None => Err(format!("Could not find function {}", counter_id.0)),
      }?;

      if self.demangled_name_to_fnid.remove(&expected_name).is_none() {
        return Err(format!(
          "Inconsistent structures: demangled -> id missing {}",
          expected_name
        ));
      } else if self.fnid_to_argument_sizes.remove(&counter_id).is_none() {
        return Err(format!(
          "Inconsistent structures: id -> argsizes missing {} {}",
          counter_id.0, expected_name
        ));
      }
      lg.trace(format!(
        "Masked out function {} from module {}",
        counter_id.hex_string(),
        counter_id.hex_string()
      ));
    }
    Ok(())
  }
}

fn bytes_to_num<T: num_traits::Num>(inp: &[u8]) -> Result<T, String>
where
  <T as num_traits::Num>::FromStrRadixErr: ToString,
{
  let radix10 = Constants::parse_fnid_radix();
  String::from_utf8(inp.to_vec())
    .map_err(|e| e.to_string())
    .and_then(|v| T::from_str_radix(&v, radix10).map_err(|e| e.to_string()))
}

fn parse_fn_id_tuple(inp: &[&[u8]]) -> Result<(IntegralFnId, String, Vec<u16>), String> {
  if inp.len() < 3 {
    return Err("Invalid ID format, expecting at least 3 items per line".into());
  }

  let try_name = String::from_utf8(inp[0].to_vec());
  if let Err(e) = try_name {
    return Err(e.to_string());
  }

  let fnid: IntegralFnId = IntegralFnId(bytes_to_num(inp[1])?);

  let arg_count: usize = bytes_to_num(inp[2])?;
  if 3 + arg_count != inp.len() {
    return Err("Invalid argumetn count - not in sync with the data".to_string());
  }

  let mut argument_specifiers: Vec<u16> = Vec::new();
  for v in inp.iter().skip(3) {
    let arg_spec: u16 = bytes_to_num(v)?;
    argument_specifiers.push(arg_spec);
  }

  Ok((fnid, try_name.unwrap(), argument_specifiers))
}

impl TryFrom<&[&[u8]]> for FunctionMap {
  type Error = String;
  fn try_from(value: &[&[u8]]) -> Result<Self, Self::Error> {
    let newline_split = value.iter().filter(|v| !v.is_empty());
    let zero_splits: Vec<Vec<&[u8]>> = newline_split
      .map(|v: &&[u8]| v.split(|v| *v == 0x0).collect::<Vec<&[u8]>>())
      .filter(|v| !v.is_empty())
      .collect();

    let parsed_pairs = zero_splits
      .iter()
      .map(|split_pair| parse_fn_id_tuple(split_pair));

    let mut target = Self::new(&[], HashMap::new());

    let lg = Log::get("FunctionMap::TryFrom[u8]");
    for pair_res in parsed_pairs {
      match pair_res {
        Err(e) => lg.warn(format!("Could not parse function ID pair: {e}")),
        Ok((id, name, specifiers)) => {
          if let Some(old_name) = target.fnid_to_demangled_name.insert(id, name.clone()) {
            lg.warn(format!(
              "Duplicate function ID - demangled name, this should not happen within a module! Function ID: {}, name 1: {}, name 2: {}",
              *id, old_name, name
            ));
          }
          target.demangled_name_to_fnid.insert(name.clone(), id);

          let mut size_types: Vec<ArgSizeTypeRef> = Vec::with_capacity(specifiers.len());

          for sz_type in &specifiers {
            let spec = ArgSizeTypeRef::try_from(*sz_type)?;
            size_types.push(spec);
          }

          lg.trace(format!(
            "Added fn:\n{}:\n\tid:{}\targs: {:?}",
            name, *id, size_types
          ));
          if let Some(old_thing) = target.fnid_to_argument_sizes.insert(id, size_types) {
            lg.warn(format!(
              "Duplicate function ID - argument size type, this should not happen within a module! Function ID: {}, name: {}, old types: {:?}, new specifiers: {:?}",
              *id, name, old_thing, specifiers
            ));
          }
        }
      }
    }

    Ok(target)
  }
}

fn u32_to_hex_string(num: u32) -> String {
  let [b1, b2, b3, b4] = num.to_le_bytes();
  format!("{:02X}{:02X}{:02X}{:02X}", b1, b2, b3, b4)
}

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub struct IntegralFnId(pub u32);

impl IntegralFnId {
  pub fn hex_string(&self) -> String {
    u32_to_hex_string(self.0)
  }

  // a compile check for the size, if this one fails, also see Self::size
  fn _helper_fn(x: Self) -> u32 {
    x.0
  }

  pub const fn byte_size() -> usize {
    std::mem::size_of::<u32>()
  }

  pub const fn size(&self) -> usize {
    Self::byte_size()
  }
}

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub struct IntegralModId(pub u32);

impl IntegralModId {
  pub fn hex_string(&self) -> String {
    u32_to_hex_string(self.0)
  }

  // a compile check for the size, if this one fails, also see Self::size
  fn _helper_fn(x: Self) -> u32 {
    x.0
  }

  pub const fn byte_size() -> usize {
    std::mem::size_of::<u32>()
  }

  pub const fn size(&self) -> usize {
    Self::byte_size()
  }
}

fn try_from_hex_string<T: Num + std::ops::ShlAssign<u32> + std::ops::AddAssign<u32>>(
  value: &str,
) -> Result<T, String> {
  if value.chars().count() != std::mem::size_of::<T>() * 2 {
    return Err("Invalid size".to_string());
  }

  let mut inner: T = T::zero();
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
  Ok(inner)
}

impl Deref for IntegralModId {
  type Target = u32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl Deref for IntegralFnId {
  type Target = u32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl TryFrom<&str> for IntegralModId {
  type Error = String;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    try_from_hex_string(value).map(Self)
  }
}

impl TryFrom<&str> for IntegralFnId {
  type Error = String;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    try_from_hex_string(value).map(Self)
  }
}

// URGENT TODO: get rid of modidx, document!
#[derive(Debug)]
pub struct ExtModuleMap {
  function_ids: HashMap<IntegralModId, FunctionMap>,
  module_paths: HashMap<IntegralModId, String>,
}

impl Default for ExtModuleMap {
  fn default() -> Self {
    Self::new()
  }
}

impl ExtModuleMap {
  pub fn new() -> Self {
    Self {
      function_ids: HashMap::new(),
      module_paths: HashMap::new(),
    }
  }

  pub fn modules(&self) -> std::collections::hash_map::Keys<'_, IntegralModId, FunctionMap> {
    self.function_ids.keys()
  }

  pub fn functions(
    &self,
    mod_id: IntegralModId,
  ) -> Option<std::collections::hash_map::Keys<'_, IntegralFnId, std::string::String>> {
    self.function_ids.get(&mod_id).map(|f| f.functions())
  }

  pub fn mask_include(&mut self, targets: &[LLVMFunId]) -> Result<(), String> {
    let lg = Log::get("mask_include");
    lg.info(format!("Masking {} values", targets.len()));

    let mut allowlist_fn: HashMap<IntegralModId, HashSet<IntegralFnId>> = HashMap::new();
    for id in targets {
      let (m, f) = (&id.fn_module, &id.fn_name);
      let mod_id = match self.find_module_hash_by_name(m) {
        Some(x) => x,
        None => {
          lg.warn(format!("Module hash for name {m} not found"));
          continue;
        }
      };

      if let Some(fn_id) = self.get_function_id(mod_id, f).cloned() {
        allowlist_fn
          .entry(mod_id)
          .and_modify(|set| {
            set.insert(fn_id);
          })
          .or_insert(HashSet::from_iter([fn_id].into_iter()));
      } else {
        lg.warn(format!("Function {f} not found in module {:02X}", mod_id.0));
      }
    }

    for modid in self
      .module_paths
      .keys()
      .filter(|m| allowlist_fn.contains_key(*m))
    {
      let functions = self.function_ids.get_mut(modid).unwrap();
      functions.mask_include(allowlist_fn.get(modid).unwrap())?;
      lg.info(format!(
        "Functions {:?} remain in module {}",
        allowlist_fn
          .get(modid)
          .unwrap()
          .iter()
          .map(|x| (x.hex_string(), self.get_function_name(*modid, *x).unwrap()))
          .collect::<Vec<_>>(),
        modid.hex_string()
      ));
    }

    let mods = self.function_ids.keys().cloned().collect::<Vec<_>>();
    for md in mods {
      let fun = &self.function_ids[&md];
      if fun.is_empty() || !allowlist_fn.contains_key(&md) {
        self.function_ids.remove(&md);
        self.module_paths.remove(&md);
        lg.info(format!("Removed module {}", md.hex_string()));
      }
    }

    Ok(())
  }

  pub fn add_module(&mut self, path_to_modfile: &PathBuf) -> Result<(), String> {
    let modhash = if let Some(hash_res) = path_to_modfile
      .file_name()
      .and_then(|v| v.to_str())
      .and_then(|v| IntegralModId::try_from(v).into())
    {
      hash_res
    } else {
      Err(format!("Invalid path {:?}", path_to_modfile))
    }?;

    if self.function_ids.contains_key(&modhash) {
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

    let fn_map = FunctionMap::try_from(fn_map)?;
    self.function_ids.insert(modhash, fn_map);
    self.module_paths.insert(modhash, module_str_id);
    Ok(())
  }

  pub fn find_module_hash_by_name(&self, name: &String) -> Option<IntegralModId> {
    self
      .module_paths
      .iter()
      .find(|(_, v)| *v == name)
      .map(|v| *v.0)
  }

  pub fn get_function_name(&self, mod_id: IntegralModId, fn_id: IntegralFnId) -> Option<&String> {
    self.function_ids.get(&mod_id)?.get_name(fn_id)
  }

  pub fn get_function_id(&self, mod_id: IntegralModId, fn_name: &String) -> Option<&IntegralFnId> {
    self.function_ids.get(&mod_id)?.get_id(fn_name)
  }

  pub fn get_function_arg_size_descriptors(
    &self,
    mod_id: IntegralModId,
    fn_id: IntegralFnId,
  ) -> Option<&Vec<ArgSizeTypeRef>> {
    self.function_ids.get(&mod_id)?.get_arg_size_ref(fn_id)
  }

  pub fn get_module_string_id(&self, mod_id: IntegralModId) -> Option<&String> {
    self.module_paths.get(&mod_id)
  }

  pub fn print_summary(&self) {
    println!("Module map summary:");
    println!("Total Modules loaded: {}", self.function_ids.len());
    println!(
      "Total Functions loaded: {}",
      self
        .function_ids
        .values()
        .map(|fnids| fnids.fnid_to_demangled_name.len())
        .sum::<usize>()
    );
  }
}

impl TryFrom<&PathBuf> for ExtModuleMap {
  type Error = String;

  fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
    if !path.exists() || !path.is_dir() {
      return Err(format!("{} is not a directory", path.to_string_lossy()));
    }

    let mut target = ExtModuleMap::new();

    let dir = std::fs::read_dir(path)
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
