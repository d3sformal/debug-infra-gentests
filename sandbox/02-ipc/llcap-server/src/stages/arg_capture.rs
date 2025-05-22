use std::{
  collections::HashMap,
  fs::File,
  io::{self, BufWriter, Write},
  path::{Path, PathBuf},
};

use crate::modmap::{ExtModuleMap, IntegralFnId, IntegralModId};

pub struct FunctionPacketDumper {
  fnid: IntegralFnId,
  underlying_file: BufWriter<File>,
}

impl FunctionPacketDumper {
  // TODO: make single source of truth along with conversion in the other direciton

  pub fn new(function_id: IntegralFnId, root: &Path, capacity: usize) -> Result<Self, io::Error> {
    let name = function_id.hex_string();

    let f = File::create_new(root.join(name))?;
    let b = BufWriter::with_capacity(capacity, f);

    Ok(Self {
      fnid: function_id,
      underlying_file: b,
    })
  }

  pub fn flush(&mut self) -> Result<(), io::Error> {
    self.underlying_file.flush()
  }

  // Ok result returns the number of bytes written
  pub fn write(&mut self, buff: &mut [u8]) -> Result<usize, io::Error> {
    self.underlying_file.write(buff)
  }
}

pub struct ModulePacketDumper {
  mod_id: IntegralModId,
  func_dumpers: HashMap<IntegralFnId, FunctionPacketDumper>,
  module_root: PathBuf,
  capacity_hint: usize,
}

impl ModulePacketDumper {
  pub fn new(
    module_id: IntegralModId,
    packet_root: &Path,
    function_ids: &mut dyn Iterator<Item = &IntegralFnId>,
    capacity_hint: usize,
  ) -> Result<Self, io::Error> {
    let dir_name = module_id.hex_string();
    let module_root = packet_root.join(dir_name);
    std::fs::create_dir_all(&module_root)?;

    let mut func_dumpers = HashMap::new();

    for fnid in function_ids {
      let fndumper = FunctionPacketDumper::new(*fnid, &module_root, capacity_hint)?;

      if func_dumpers.insert(*fnid, fndumper).is_some() {
        return Err(std::io::Error::other(format!(
          "Duplication of function IDs is not expected! Duplicated fnid: {}",
          **fnid
        )));
      }
    }

    Ok(Self {
      mod_id: module_id,
      func_dumpers,
      module_root,
      capacity_hint,
    })
  }

  pub fn get_function_dumper(
    &mut self,
    function: IntegralFnId,
  ) -> Option<&mut FunctionPacketDumper> {
    self.func_dumpers.get_mut(&function)
  }

  pub fn flush(&mut self) -> Result<(), io::Error> {
    for kv in self.func_dumpers.values_mut() {
      kv.flush()?;
    }
    Ok(())
  }
}

pub struct ArgPacketDumper {
  dumpers: HashMap<IntegralModId, ModulePacketDumper>,
  root: PathBuf,
}

impl ArgPacketDumper {
  pub fn new(
    root_out_dir: &Path,
    module_maps: &ExtModuleMap,
    mem_limit: usize,
  ) -> Result<Self, io::Error> {
    let mut result_map = HashMap::new();

    let capacity = (mem_limit / module_maps.modules().count()).max(4096 * 2);

    for module in module_maps.modules() {
      let mut functions = module_maps.functions(*module).map_or_else(
        || {
          Err(io::Error::other(format!(
            "Module {} did not map to any function set!",
            **module
          )))
        },
        Ok,
      )?;

      result_map.insert(
        *module,
        ModulePacketDumper::new(*module, root_out_dir, &mut functions, capacity)?,
      );
    }

    Ok(Self {
      dumpers: result_map,
      root: root_out_dir.to_owned(),
    })
  }

  pub fn get_packet_dumper(
    &mut self,
    module: IntegralModId,
    function: IntegralFnId,
  ) -> Option<&mut FunctionPacketDumper> {
    if let Some(md) = self.dumpers.get_mut(&module) {
      md.get_function_dumper(function)
    } else {
      None
    }
  }
}
