use crate::{
  log::Log,
  modmap::{ExtModuleMap, IntegralFnId, IntegralModId},
  shmem_capture::{
    BorrowedReadBuffer, CaptureLoop, CaptureLoopState, ReadOnlyBufferPtr, run_capture,
  },
  sizetype_handlers::{
    ArgSizeTypeRef, CStringTypeReader, CustomTypeReader, FixedSizeTyData, FixedSizeTyReader,
    ReadProgress, SizeTypeReader,
  },
  stages::arg_capture::ArgPacketDumper,
};
use anyhow::{Result, ensure};

use super::TracingInfra;

struct SizeTypeReaders {
  fixed: [Box<dyn SizeTypeReader>; 17],
  c_str: Box<dyn SizeTypeReader>,
  custom: Box<dyn SizeTypeReader>,
}

fn boxed_ty_reader(size: usize) -> Box<FixedSizeTyReader> {
  Box::new(FixedSizeTyReader::new(FixedSizeTyData::of_size(size)))
}

fn create_sizetype_cache() -> SizeTypeReaders {
  SizeTypeReaders {
    fixed: [
      boxed_ty_reader(0),
      boxed_ty_reader(1),
      boxed_ty_reader(2),
      boxed_ty_reader(3),
      boxed_ty_reader(4),
      boxed_ty_reader(5),
      boxed_ty_reader(6),
      boxed_ty_reader(7),
      boxed_ty_reader(8),
      boxed_ty_reader(9),
      boxed_ty_reader(10),
      boxed_ty_reader(11),
      boxed_ty_reader(12),
      boxed_ty_reader(13),
      boxed_ty_reader(14),
      boxed_ty_reader(15),
      boxed_ty_reader(16),
    ],
    c_str: Box::new(CStringTypeReader::new()),
    custom: Box::new(CustomTypeReader::new()),
  }
}

impl SizeTypeReaders {
  pub fn get_reader(&mut self, sz: ArgSizeTypeRef) -> Option<&mut Box<dyn SizeTypeReader>> {
    let res = match sz {
      ArgSizeTypeRef::Fixed(i) => match i {
        0..=16 => &mut self.fixed[i],
        _ => return None,
      },
      ArgSizeTypeRef::Cstr => &mut self.c_str,
      ArgSizeTypeRef::Custom => &mut self.custom,
    };
    Some(res)
  }
}

#[derive(Debug, Eq, PartialEq)]
enum PartialCaptureState {
  Empty,
  GotModuleId {
    module_id: IntegralModId,
  },
  CapturingArgs {
    module_id: IntegralModId,
    fn_id: IntegralFnId,
    arg_idx: usize,
    buff: Vec<u8>,
  },
  Done {
    module_id: IntegralModId,
    fn_id: IntegralFnId,
    buff: Vec<u8>,
  },
}

impl PartialCaptureState {
  /// reads module ID and shifts raw_buff by the size of module ID
  fn progress_read_mod_id(raw_buff: &mut ReadOnlyBufferPtr, mods: &ExtModuleMap) -> Result<Self> {
    let lg = Log::get("progress_get_module_id");
    lg.trace("Start");
    // keep the type annotation to warn if implementing types change
    let rcvd_id: u32 = raw_buff.unaligned_shift_num_read()?;
    let rcvd_id = IntegralModId::from(rcvd_id);
    ensure!(
      mods.get_module_string_id(rcvd_id).is_some(),
      "Module ID {} is unknown",
      *rcvd_id
    );
    lg.trace(format!("Mod Id: 0x{:02X}", *rcvd_id));
    Ok(Self::GotModuleId { module_id: rcvd_id })
  }

  /// reads function ID and shifts raw_buff by the size of function ID
  fn progress_read_fn_id(
    raw_buff: &mut ReadOnlyBufferPtr,
    mods: &ExtModuleMap,
    module_id: IntegralModId,
  ) -> Result<Self> {
    let lg = Log::get("progress::progress_read_fn_id");
    lg.trace("Reading fnid");
    let fn_id: u32 = raw_buff.unaligned_shift_num_read()?;
    // keep the type annotation to warn if implementing types change
    let fn_id: IntegralFnId = IntegralFnId::from(fn_id);

    ensure!(
      mods.get_function_name(module_id, fn_id).is_some(),
      "Function id not found @ module 0x{:02X}: 0x{:02X}",
      *module_id,
      *fn_id
    );
    lg.trace(format!("Fnc Id: 0x{:02X}", *fn_id));
    Ok(Self::CapturingArgs {
      module_id,
      fn_id,
      arg_idx: 0,
      buff: Vec::new(),
    })
  }

  /// reads all arguments necessary/available and shifts raw_buff
  /// according to the argument types (could be determined dynamically)
  fn progress_read_args(
    raw_buff: &mut ReadOnlyBufferPtr,
    mods: &ExtModuleMap,
    readers: &mut SizeTypeReaders,
    module_id: IntegralModId,
    fn_id: IntegralFnId,
    arg_idx: usize,
    mut buff: Vec<u8>,
  ) -> Result<Self> {
    let lg = Log::get("progress_read_args");
    let size_refs = mods.get_function_arg_size_descriptors(module_id, fn_id);
    ensure!(
      size_refs.is_some(),
      "Unknown function with module idx {} fn id {}",
      *module_id,
      *fn_id
    );
    let size_refs = size_refs.unwrap();
    if arg_idx >= size_refs.len() {
      lg.warn(format!(
        "Invalid arg index {} for args {:?} in ID m/f {}/{}",
        arg_idx,
        size_refs,
        module_id.hex_string(),
        fn_id.hex_string()
      ));
    }

    for (i, desc) in size_refs.iter().enumerate().skip(arg_idx) {
      lg.trace(format!("Argument idx: {}, desc: {:?}", i, desc));
      if raw_buff.empty() {
        return Ok(Self::CapturingArgs {
          module_id,
          fn_id,
          arg_idx: i,
          buff,
        });
      }

      let reader = readers.get_reader(*desc);
      ensure!(
        reader.is_some(),
        "Unexpected size type that is missing a reader {:?}",
        desc
      );
      let reader = reader.unwrap();

      let slice = raw_buff.as_slice();
      match reader.read(slice)? {
        ReadProgress::Done {
          mut payload,
          consumed_bytes,
        } => {
          buff.append(&mut payload);
          raw_buff.shift(consumed_bytes);
        }
        ReadProgress::NotYet => {
          raw_buff.shift(slice.len());
          return Ok(Self::CapturingArgs {
            module_id,
            fn_id,
            arg_idx: i,
            buff,
          });
        }
        ReadProgress::Nop => {
          raw_buff.shift(slice.len());
          return Ok(Self::CapturingArgs {
            module_id,
            fn_id,
            arg_idx: i,
            buff,
          });
        }
      }

      ensure!(reader.done(), "Sanity check (reader done) failed");
      lg.trace(format!("Resetting reader {}", i));
      reader.read_reset();
    }
    Ok(Self::Done {
      module_id,
      fn_id,
      buff,
    })
  }

  /// This function mutates (shifts) the raw_buff within the bounds of buff_end
  pub fn progress(
    self,
    raw_buff: &mut ReadOnlyBufferPtr,
    mods: &ExtModuleMap,
    readers: &mut SizeTypeReaders,
  ) -> Result<Self> {
    match self {
      Self::Empty => Self::progress_read_mod_id(raw_buff, mods),
      Self::GotModuleId { module_id } => Self::progress_read_fn_id(raw_buff, mods, module_id),
      Self::CapturingArgs {
        module_id,
        fn_id,
        arg_idx,
        buff,
      } => Self::progress_read_args(raw_buff, mods, readers, module_id, fn_id, arg_idx, buff),
      Self::Done {
        module_id,
        fn_id,
        buff,
      } => {
        let lg = Log::get("progress::Done");
        lg.warn("Noop in arg capture progress");
        Ok(Self::Done {
          module_id,
          fn_id,
          buff,
        })
      }
    }
  }
}

#[derive(Debug)]
struct ArgCaptureState {
  partial_state: PartialCaptureState,
  payload: Vec<(IntegralModId, IntegralFnId, Vec<u8>)>,
  endmessage_counter: usize,
}

impl Default for ArgCaptureState {
  fn default() -> Self {
    Self {
      endmessage_counter: 0,
      payload: Vec::new(),
      partial_state: PartialCaptureState::Empty,
    }
  }
}

impl ArgCaptureState {
  fn extract_messages(&mut self) -> Vec<(IntegralModId, IntegralFnId, Vec<u8>)> {
    let mut res = vec![];
    std::mem::swap(&mut res, &mut self.payload);
    res
  }
}

pub fn perform_arg_capture(
  infra: &mut TracingInfra,
  modules: &ExtModuleMap,
  capture_target: &mut ArgPacketDumper,
) -> Result<Vec<(IntegralModId, IntegralFnId, Vec<u8>)>> {
  let capture = ArgCapture {
    cache: create_sizetype_cache(),
    results: vec![],
    capture_target,
  };

  let finished = run_capture(capture, infra, modules).map_err(|e| e.context("arg_capture"))?;
  Ok(finished.results)
}

struct ArgCapture<'a> {
  cache: SizeTypeReaders,
  results: Vec<(IntegralModId, IntegralFnId, Vec<u8>)>,
  capture_target: &'a mut ArgPacketDumper,
}

impl CaptureLoopState for ArgCaptureState {
  fn get_end_message_count(&self) -> usize {
    self.endmessage_counter
  }

  fn reset_end_message_count(&mut self) {
    self.endmessage_counter = 0;
  }
}

impl<'a> CaptureLoop for ArgCapture<'a> {
  type State = ArgCaptureState;

  fn update_from_buffer<'b>(
    &mut self,
    mut state: Self::State,
    mut buffer: BorrowedReadBuffer<'b>,
    modules: &ExtModuleMap,
  ) -> Result<Self::State> {
    let buff = &mut buffer.buffer;
    if buff.empty() {
      ensure!(
        state.partial_state == PartialCaptureState::Empty,
        "Comms corruption - partial state with empty message following it! Partial state: {:?}",
        state.partial_state
      );
      state.endmessage_counter += 1;
      return Ok(state);
    }
    while !buff.empty() {
      let partial_state = state
        .partial_state
        .progress(buff, modules, &mut self.cache)?;

      state.partial_state = match partial_state {
        PartialCaptureState::Done {
          module_id: mod_id,
          fn_id,
          mut buff,
        } => {
          if let Some(dumper) = self.capture_target.get_packet_dumper(mod_id, fn_id) {
            dumper.dump(&mut buff)?;
          }
          Log::get("argCap update_from_buffer").trace(format!("{:?}", buff));
          state.payload.push((mod_id, fn_id, buff));
          // start from an empty state
          PartialCaptureState::Empty
        }
        st => st, // continue from the non-Done state
      };
    }

    Ok(state)
  }

  fn process_state(&mut self, mut state: Self::State) -> Result<Self::State> {
    let mut msgs = state.extract_messages();
    self.results.append(&mut msgs);
    Ok(state)
  }
}
