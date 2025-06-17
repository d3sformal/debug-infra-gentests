use std::{
  collections::HashMap,
  fs::File,
  io::{BufReader, BufWriter, ErrorKind, Read, Write},
  path::{Path, PathBuf},
  sync::{Arc, Mutex, MutexGuard},
};

use anyhow::{Result, anyhow, bail, ensure};

use crate::{
  log::Log,
  modmap::{ExtModuleMap, IntegralFnId, IntegralModId},
};

/// dumps argument packet data into a file
pub struct FunctionPacketDumper {
  _fnid: IntegralFnId,
  underlying_file: BufWriter<File>,
}

impl FunctionPacketDumper {
  pub fn new(function_id: IntegralFnId, root: &Path, capacity: usize) -> Result<Self> {
    let name = function_id.hex_string();

    let f = File::create_new(root.join(name))?;
    let b = BufWriter::with_capacity(capacity, f);

    Ok(Self {
      _fnid: function_id,
      underlying_file: b,
    })
  }

  /// Dumps the raw packet
  ///
  /// Ok variant contains the total number of bytes written
  pub fn dump(&mut self, packet_payload: &mut [u8]) -> Result<usize> {
    let n = self
      .underlying_file
      .write(&(packet_payload.len() as u32).to_le_bytes())
      .map_err(|e| anyhow!(e).context("Packet length dump failed"))?;
    self
      .underlying_file
      .write(packet_payload)
      .map_err(|e| anyhow!(e))
      .map(|v| v + n)
  }
}

/// aggregates function packet dumpers associated with a module
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
  ) -> Result<Self> {
    let dir_name = module_id.hex_string();
    let module_root = packet_root.join(dir_name);
    std::fs::create_dir_all(&module_root)?;

    let mut func_dumpers = HashMap::new();

    for fnid in function_ids {
      let fndumper = FunctionPacketDumper::new(*fnid, &module_root, capacity_hint)?;

      ensure!(
        func_dumpers.insert(*fnid, fndumper).is_none(),
        "Duplication of function IDs is not expected, duplicated fnid: {}",
        **fnid
      );
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

/// dumps argument packet data to a persistent (filesystem) structure
pub struct ArgPacketDumper {
  dumpers: HashMap<IntegralModId, ModulePacketDumper>,
  _root: PathBuf,
}

impl ArgPacketDumper {
  /// creates a dumper that writes data into the root_out_dir directory
  pub fn new(root_out_dir: &Path, module_maps: &ExtModuleMap, mem_limit: usize) -> Result<Self> {
    let mut result_map = HashMap::new();

    let capacity = (mem_limit / module_maps.modules().count()).max(4096 * 2);

    for module in module_maps.modules() {
      let functions = module_maps.functions(*module);
      ensure!(
        functions.is_some(),
        "Module {} did not map to any function set!",
        **module
      );
      let mut functions = functions.unwrap();

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

/// facilitates read-only access to a single argument capture data stream
/// (multiple packets inside a capture for a single function)
struct CaptureReader {
  path: PathBuf,
  file: BufReader<File>,
  packets: usize,
  args: usize,
  idx: usize,
}

/// exposes iteration over individual argument packets on individual function basis
/// and metadata (packet counts, ...)
pub struct PacketReader {
  captures: HashMap<(IntegralModId, IntegralFnId), Arc<Mutex<dyn PacketIterator + Send>>>,
}

impl PacketReader {
  pub fn new(dir: &Path, module_maps: &ExtModuleMap, buff_limit: usize) -> Result<Self> {
    let lg = Log::get("PacketReader");
    let capacity = (buff_limit / module_maps.modules().count()).max(4096 * 2);
    let mut captures: HashMap<
      (IntegralModId, IntegralFnId),
      Arc<Mutex<dyn PacketIterator + Send>>,
    > = HashMap::new();
    for module in module_maps.modules() {
      let functions = module_maps.functions(*module);
      ensure!(
        functions.is_some(),
        "Module {} did not map to any function set!",
        **module
      );
      let functions = functions.unwrap();

      for function in functions {
        let path = dir.join(module.hex_string()).join(function.hex_string());
        let key = (*module, *function);
        if !path.exists() {
          lg.warn(format!(
            "Inserting dummy packet iterator for {:?} - missing capture file\n\tFunction name: {:?} \n\tModule name: {:?}",
            path,
            module_maps.get_function_name(*module, *function),
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
            let mut record = CaptureReader {
              path: path.clone(),
              file: BufReader::with_capacity(capacity, File::open(path.clone())?),
              packets: 0,
              args: 0,
              idx: 0,
            };

            // a single pass on all packets to count them (and validate them)
            while record.read_next_packet()?.is_some() {
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
              Err(anyhow!("Failed look up function argument types"))
            }?;
          lg.info(format!("\targs {} ", args));
          captures.insert(
            (*module, *function),
            Arc::new(Mutex::new(CaptureReader {
              path: path.clone(),
              file: BufReader::with_capacity(capacity, File::open(path)?),
              packets: tests,
              args,
              idx: 0,
            })),
          );
        }
      }
    }

    Ok(Self { captures })
  }

  fn get_locked_capture_iterator(
    &mut self,
    m: IntegralModId,
    f: IntegralFnId,
  ) -> Result<MutexGuard<'_, dyn PacketIterator + Send + 'static>> {
    if let Some(v) = self.captures.get_mut(&(m, f)) {
      Ok(v.lock().unwrap())
    } else {
      bail!(
        "Not found in packet reader m/f {} {}",
        m.hex_string(),
        f.hex_string()
      )
    }
  }

  pub fn read_next_packet(
    &mut self,
    module: IntegralModId,
    fun: IntegralFnId,
  ) -> Result<Option<Vec<u8>>> {
    let mut it = self
      .get_locked_capture_iterator(module, fun)
      .map_err(|e| e.context("read_next_packet"))?;
    it.read_next_packet()
  }

  pub fn try_reset(&mut self, module: IntegralModId, fun: IntegralFnId) -> Result<()> {
    let mut it = self
      .get_locked_capture_iterator(module, fun)
      .map_err(|e| e.context("try_reset"))?;
    it.try_reset()
  }

  pub fn get_packet_count(&self, module: IntegralModId, fun: IntegralFnId) -> Option<u32> {
    self
      .captures
      .get(&(module, fun))
      .map(|v| v.lock().unwrap().packet_count())
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

/// represents a forward packet iterator that may be reset to the begining
trait PacketIterator {
  fn read_next_packet(&mut self) -> Result<Option<Vec<u8>>>;
  fn packet_count(&self) -> u32;
  fn arg_count(&self) -> u32;
  fn try_reset(&mut self) -> Result<()>;
  fn upcoming_packet_idx(&mut self) -> usize;
}

/// see [`FunctionPacketDumper::dump`]
impl PacketIterator for CaptureReader {
  fn read_next_packet(&mut self) -> Result<Option<Vec<u8>>> {
    let mut buf = [0u8; std::mem::size_of::<u32>()];
    // packet size
    match self.file.read_exact(&mut buf) {
      Ok(()) => (),
      Err(e) => {
        if e.kind() == ErrorKind::UnexpectedEof {
          self.idx += 1;
          return Ok(None);
        }
        bail!("Failed to read packet len: {}", e)
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
      Err(e) => Err(anyhow!("Error when reading {} packet len, err {}", len, e)),
    }
  }

  fn packet_count(&self) -> u32 {
    self.packets as u32
  }

  fn arg_count(&self) -> u32 {
    self.args as u32
  }

  fn try_reset(&mut self) -> Result<()> {
    self.file = BufReader::with_capacity(self.file.capacity(), File::open(self.path.clone())?);
    self.idx = 0;
    Ok(())
  }

  fn upcoming_packet_idx(&mut self) -> usize {
    self.idx
  }
}

struct EmptyPacketIter {}

impl PacketIterator for EmptyPacketIter {
  fn read_next_packet(&mut self) -> Result<Option<Vec<u8>>> {
    Ok(None)
  }

  fn packet_count(&self) -> u32 {
    0
  }

  fn arg_count(&self) -> u32 {
    0
  }

  fn try_reset(&mut self) -> Result<()> {
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
    let tests = self.get_packet_count(m, f)?;
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
