use std::{
  collections::HashMap,
  fs::File,
  io::{self, BufReader, BufWriter, ErrorKind, Read, Write},
  path::{Path, PathBuf},
};

use crate::{
  log::Log,
  modmap::{ExtModuleMap, IntegralFnId, IntegralModId},
};

pub struct FunctionPacketDumper {
  fnid: IntegralFnId,
  underlying_file: BufWriter<File>,
}

impl FunctionPacketDumper {
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

struct CaptureRecord {
  file: BufReader<File>,
  tests: usize,
  args: usize,
  current: usize,
}

pub struct PacketReader {
  captures: HashMap<(IntegralModId, IntegralFnId), Box<dyn PacketIterator>>,
}

impl PacketReader {
  pub fn new(
    dir: &Path,
    module_maps: &ExtModuleMap,
    buff_limit: usize,
  ) -> Result<Self, std::io::Error> {
    let lg = Log::get("PacketReader");
    let capacity = (buff_limit / module_maps.modules().count()).max(4096 * 2);
    let mut captures: HashMap<(IntegralModId, IntegralFnId), Box<dyn PacketIterator>> =
      HashMap::new();
    for module in module_maps.modules() {
      let functions = module_maps.functions(*module).map_or_else(
        || {
          Err(io::Error::other(format!(
            "Module {} did not map to any function set!",
            **module
          )))
        },
        Ok,
      )?;
      for function in functions {
        let path = dir.join(module.hex_string()).join(function.hex_string());
        let key = (*module, *function);
        if !path.exists() {
          lg.warn(format!(
            "Inserting dummy packet iterator for {:?} - missing capture file",
            path
          ));
          lg.warn(format!(
            "\tFunction name: {:?}",
            module_maps.get_function_name(*module, *function)
          ));
          lg.warn(format!(
            "\tModule name: {:?}",
            module_maps.get_module_string_id(*module)
          ));
          captures.insert(key, Box::new(EmptyPacketIter {}));
        } else {
          let mut tests = 0;
          lg.info(format!(
            "Setup {} {}",
            module.hex_string(),
            function.hex_string()
          ));
          {
            let mut record = CaptureRecord {
              file: BufReader::with_capacity(capacity, File::open(path.clone())?),
              tests: 0,
              args: 0,
              current: 0,
            };

            while record
              .read_next_packet()
              .map_err(io::Error::other)?
              .is_some()
            {
              tests += 1;
            }
          }
          lg.info(format!("\ttests {} ", tests));

          let args =
            if let Some(v) = module_maps.get_function_arg_size_descriptors(*module, *function) {
              Ok(
                v.iter()
                  .filter(|x| !matches!(x, crate::sizetype_handlers::ArgSizeTypeRef::Fixed(0)))
                  .count(),
              )
            } else {
              Err(io::Error::other("Failed look up function argument types"))
            }?;
          lg.info(format!("\targs {} ", args));
          captures.insert(
            (*module, *function),
            Box::new(CaptureRecord {
              file: BufReader::with_capacity(capacity, File::open(path)?),
              tests,
              args,
              current: 0,
            }),
          );
        }
      }
    }

    Ok(Self { captures })
  }

  pub fn read_next_packet(
    &mut self,
    module: IntegralModId,
    fun: IntegralFnId,
  ) -> Result<Option<Vec<u8>>, String> {
    if let Some(v) = self.captures.get_mut(&(module, fun)) {
      v.read_next_packet()
    } else {
      Err("Not found".to_string())
    }
  }

  pub fn get_test_count(&self, module: IntegralModId, fun: IntegralFnId) -> Option<u32> {
    self
      .captures
      .get(&(module, fun))
      .map(|v: &Box<dyn PacketIterator>| v.test_count())
  }

  pub fn get_arg_count(&self, module: IntegralModId, fun: IntegralFnId) -> Option<u32> {
    self.captures.get(&(module, fun)).map(|v| v.arg_count())
  }
}

trait PacketIterator {
  fn read_next_packet(&mut self) -> Result<Option<Vec<u8>>, String>;
  fn test_count(&self) -> u32;
  fn arg_count(&self) -> u32;
}

impl PacketIterator for CaptureRecord {
  fn read_next_packet(&mut self) -> Result<Option<Vec<u8>>, String> {
    let mut buf = [0u8; std::mem::size_of::<u32>()];

    match self.file.read_exact(&mut buf) {
      Ok(()) => (),
      Err(e) => {
        if e.kind() == ErrorKind::UnexpectedEof {
          return Ok(None);
        } else {
          return Err(format!("Failed to read packet len: {}", e));
        }
      }
    };
    let len = u32::from_le_bytes(buf);
    if len == 0 {
      return Ok(None);
    }

    let mut result = vec![0; len as usize];
    match self.file.read_exact(&mut result) {
      Ok(_) => Ok(Some(result)),
      Err(e) => Err(format!("Error when reading {} packet len, err {}", len, e)),
    }
  }

  fn test_count(&self) -> u32 {
    self.tests as u32
  }

  fn arg_count(&self) -> u32 {
    self.args as u32
  }
}

struct EmptyPacketIter {}

impl PacketIterator for EmptyPacketIter {
  fn read_next_packet(&mut self) -> Result<Option<Vec<u8>>, String> {
    Ok(None)
  }

  fn test_count(&self) -> u32 {
    0
  }

  fn arg_count(&self) -> u32 {
    0
  }
}
