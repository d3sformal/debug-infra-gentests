use std::{
  collections::HashMap,
  fs::File,
  io::{self, BufReader, BufWriter, ErrorKind, Read, Write},
  path::{Path, PathBuf},
  sync::{Arc, Mutex},
};

use crate::{
  log::Log,
  modmap::{ExtModuleMap, IntegralFnId, IntegralModId},
};

pub struct FunctionPacketDumper {
  _fnid: IntegralFnId,
  underlying_file: BufWriter<File>,
}

impl FunctionPacketDumper {
  pub fn new(function_id: IntegralFnId, root: &Path, capacity: usize) -> Result<Self, io::Error> {
    let name = function_id.hex_string();

    let f = File::create_new(root.join(name))?;
    let b = BufWriter::with_capacity(capacity, f);

    Ok(Self {
      _fnid: function_id,
      underlying_file: b,
    })
  }

  // Ok result returns the number of bytes written
  pub fn write(&mut self, buff: &mut [u8]) -> Result<usize, io::Error> {
    self.underlying_file.write(buff)
  }
}

pub struct ModulePacketDumper {
  _mod_id: IntegralModId,
  func_dumpers: HashMap<IntegralFnId, FunctionPacketDumper>,
  _module_root: PathBuf,
  _capacity_hint: usize,
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
      _mod_id: module_id,
      func_dumpers,
      _module_root: module_root,
      _capacity_hint: capacity_hint,
    })
  }

  pub fn get_function_dumper(
    &mut self,
    function: IntegralFnId,
  ) -> Option<&mut FunctionPacketDumper> {
    self.func_dumpers.get_mut(&function)
  }
}

pub struct ArgPacketDumper {
  dumpers: HashMap<IntegralModId, ModulePacketDumper>,
  _root: PathBuf,
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
      _root: root_out_dir.to_owned(),
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
  path: PathBuf,
  file: BufReader<File>,
  tests: usize,
  args: usize,
  idx: usize,
}

pub struct PacketReader {
  captures: HashMap<(IntegralModId, IntegralFnId), Arc<Mutex<dyn PacketIterator + Send>>>,
}

impl PacketReader {
  pub fn new(
    dir: &Path,
    module_maps: &ExtModuleMap,
    buff_limit: usize,
  ) -> Result<Self, std::io::Error> {
    let lg = Log::get("PacketReader");
    let capacity = (buff_limit / module_maps.modules().count()).max(4096 * 2);
    let mut captures: HashMap<
      (IntegralModId, IntegralFnId),
      Arc<Mutex<dyn PacketIterator + Send>>,
    > = HashMap::new();
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
          captures.insert(key, Arc::new(Mutex::new(EmptyPacketIter {})));
        } else {
          let mut tests = 0;
          lg.info(format!(
            "Setup {} {}",
            module.hex_string(),
            function.hex_string()
          ));
          {
            let mut record = CaptureRecord {
              path: path.clone(),
              file: BufReader::with_capacity(capacity, File::open(path.clone())?),
              tests: 0,
              args: 0,
              idx: 0,
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
            Arc::new(Mutex::new(CaptureRecord {
              path: path.clone(),
              file: BufReader::with_capacity(capacity, File::open(path)?),
              tests,
              args,
              idx: 0,
            })),
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
      let mut guard = v.lock().unwrap();
      guard.read_next_packet()
    } else {
      Err("Not found".to_string())
    }
  }

  pub fn try_reset(&mut self, module: IntegralModId, fun: IntegralFnId) -> Result<(), String> {
    if let Some(v) = self.captures.get_mut(&(module, fun)) {
      let mut guard = v.lock().unwrap();
      guard.try_reset()
    } else {
      Err("Not found".to_string())
    }
  }

  pub fn get_test_count(&self, module: IntegralModId, fun: IntegralFnId) -> Option<u32> {
    self
      .captures
      .get(&(module, fun))
      .map(|v| v.lock().unwrap().test_count())
  }

  pub fn get_arg_count(&self, module: IntegralModId, fun: IntegralFnId) -> Option<u32> {
    self
      .captures
      .get(&(module, fun))
      .map(|v| v.lock().unwrap().arg_count())
  }

  pub fn get_upcoming_pkt_idx(&self, module: IntegralModId, fun: IntegralFnId) -> Option<usize> {
    self
      .captures
      .get(&(module, fun))
      .map(|v| v.lock().unwrap().upcoming_packet_idx())
  }
}

trait PacketIterator {
  fn read_next_packet(&mut self) -> Result<Option<Vec<u8>>, String>;
  fn test_count(&self) -> u32;
  fn arg_count(&self) -> u32;
  fn try_reset(&mut self) -> Result<(), String>;
  fn upcoming_packet_idx(&mut self) -> usize;
}

impl PacketIterator for CaptureRecord {
  fn read_next_packet(&mut self) -> Result<Option<Vec<u8>>, String> {
    let mut buf = [0u8; std::mem::size_of::<u32>()];

    match self.file.read_exact(&mut buf) {
      Ok(()) => (),
      Err(e) => {
        if e.kind() == ErrorKind::UnexpectedEof {
          self.idx += 1;
          return Ok(None);
        } else {
          return Err(format!("Failed to read packet len: {}", e));
        }
      }
    };
    let len = u32::from_le_bytes(buf);
    if len == 0 {
      self.idx += 1;
      return Ok(None);
    }

    let mut result = vec![0; len as usize];
    match self.file.read_exact(&mut result) {
      Ok(_) => {
        self.idx += 1;
        Ok(Some(result))
      }
      Err(e) => Err(format!("Error when reading {} packet len, err {}", len, e)),
    }
  }

  fn test_count(&self) -> u32 {
    self.tests as u32
  }

  fn arg_count(&self) -> u32 {
    self.args as u32
  }

  fn try_reset(&mut self) -> Result<(), String> {
    self.file = BufReader::with_capacity(
      self.file.capacity(),
      File::open(self.path.clone()).map_err(|e| e.to_string())?,
    );
    self.idx = 0;
    Ok(())
  }

  fn upcoming_packet_idx(&mut self) -> usize {
    self.idx
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

  fn try_reset(&mut self) -> Result<(), String> {
    Ok(())
  }

  fn upcoming_packet_idx(&mut self) -> usize {
    0
  }
}

pub trait PacketProvider {
  fn get_packet(&mut self, m: IntegralModId, f: IntegralFnId, index: usize) -> Option<Vec<u8>>;
}

impl PacketProvider for PacketReader {
  fn get_packet(&mut self, m: IntegralModId, f: IntegralFnId, index: usize) -> Option<Vec<u8>> {
    let tests = self.get_test_count(m, f)?;
    if tests == 0 {
      return None;
    } else if index as u32 >= tests {
      // tries to return the first packet 
      self.try_reset(m, f).ok()?;
    } else if self.get_upcoming_pkt_idx(m, f)? != index {
      self.try_reset(m, f).ok()?;
      while self.get_upcoming_pkt_idx(m, f)? < index {
        self.read_next_packet(m, f).ok()?;
      }
    }
    self.read_next_packet(m, f).ok()?
  }
}
