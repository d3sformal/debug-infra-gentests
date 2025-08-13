use crate::{
  log::Log,
  modmap::{ExtModuleMap, IntegralFnId, IntegralModId, NumFunUid},
  shmem_capture::{BorrowedReadBuffer, CaptureLoop, CaptureLoopState, ReadOnlyBufferPtr},
  sizetype_handlers::{
    ArgSizeTypeRef, CStringTypeReader, CustomTypeReader, FixedSizeTyReader, ReadProgress,
    SizeTypeReader,
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
  Box::new(FixedSizeTyReader::of_size(size))
}

// the readers returned by this function are reused for every argument
// (by .reset()ing the reader)
fn get_sizetype_readers() -> SizeTypeReaders {
  SizeTypeReaders {
    // one can index by the size itself to obtain the correct reader
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

//represents various argument capturing stages
// the CapturingArgs stage
#[derive(Debug, Eq, PartialEq)]
enum PartialCaptureState {
  Empty,
  GotModuleId {
    module_id: IntegralModId,
  },
  // parses each argument according to the
  // index (via SizeTypeReaders)
  CapturingArgs {
    id: NumFunUid,
    arg_idx: usize,
    buff: Vec<u8>,
  },
  Done {
    id: NumFunUid,
    // contains the argument packet
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
    // read 4 bytes
    let fn_id: u32 = raw_buff.unaligned_shift_num_read()?;
    // note: keep the type annotations to warn if implementing types change
    let fn_id: IntegralFnId = IntegralFnId::from(fn_id);

    let id = (module_id, fn_id).into();
    ensure!(
      mods.get_function_name(id).is_some(),
      "Function id not found @ module 0x{:02X}: 0x{:02X}",
      *module_id,
      *fn_id
    );
    lg.trace(format!("Fnc Id: 0x{:02X}", *fn_id));
    Ok(Self::CapturingArgs {
      id,
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
    id: NumFunUid,
    arg_idx: usize,
    mut buff: Vec<u8>,
  ) -> Result<Self> {
    let lg = Log::get("progress_read_args");
    let size_refs = mods.get_function_arg_size_descriptors(id);
    ensure!(size_refs.is_some(), "Unknown function {id:?}");
    let size_refs = size_refs.unwrap();
    if arg_idx >= size_refs.len() {
      lg.warn(format!(
        "Invalid arg index {arg_idx} for args {size_refs:?} in ID {id:?}"
      ));
    }
    // for each argument description, parse the argument from the buffer
    for (i, desc) in size_refs.iter().enumerate().skip(arg_idx) {
      lg.trace(format!("Argument idx: {i}, desc: {desc:?}"));
      if raw_buff.empty() {
        return Ok(Self::CapturingArgs {
          id,
          arg_idx: i,
          buff,
        });
      }

      // obtain an argument reader that will read the packet
      let reader = readers.get_reader(*desc);
      ensure!(
        reader.is_some(),
        "Unexpected size type that is missing a reader {:?}",
        desc
      );
      let reader = reader.unwrap();

      let slice = raw_buff.as_slice();
      match reader.read(slice)? {
        // reader is done, argument is ready
        ReadProgress::Done {
          mut payload,
          consumed_bytes,
        } => {
          // append the argument data to the packet
          buff.append(&mut payload);
          raw_buff.shift(consumed_bytes);
          // continues the loop onto the next argument
        }
        // the reader is done with the buffer
        // and the arugment is not ready yet
        ReadProgress::NotYet => {
          // skip all the bytes
          raw_buff.shift(slice.len());
          // we're not continuing the loop, buffer is empty
          return Ok(Self::CapturingArgs {
            id,
            arg_idx: i,
            buff,
          });
        }
        ReadProgress::Nop => {
          // theoretically, this is a logic error:
          // the reader did not read anything from the buffer
          // and the raw_buff is not empty - we check once more and if the slice is
          // not empty, we must announce an error
          ensure!(
            slice.is_empty(),
            "Logic error when capturing an argument: slice len: {}",
            slice.len()
          );
          // nothing happened, somehow an empty buffer check is skipped above
          lg.crit("empty buffer check is missing, this is a soft-error");
          return Ok(Self::CapturingArgs {
            id,
            arg_idx: i,
            buff,
          });
        }
      }

      ensure!(reader.done(), "Sanity check (reader done) failed");
      lg.trace(format!("Resetting reader {i}"));
      ensure!(reader.read_reset(), "Sanity check (reader reset) failed");
    }
    Ok(Self::Done { id, buff })
  }

  /// tries to parse an argument packet using the data from raw_buff
  /// This function mutates (shifts) the raw_buff
  pub fn progress(
    self,
    raw_buff: &mut ReadOnlyBufferPtr,
    mods: &ExtModuleMap,
    readers: &mut SizeTypeReaders,
  ) -> Result<Self> {
    match self {
      Self::Empty => Self::progress_read_mod_id(raw_buff, mods),
      Self::GotModuleId { module_id } => Self::progress_read_fn_id(raw_buff, mods, module_id),
      Self::CapturingArgs { id, arg_idx, buff } => {
        Self::progress_read_args(raw_buff, mods, readers, id, arg_idx, buff)
      }
      Self::Done { id, buff } => {
        let lg = Log::get("progress::Done");
        lg.warn("Noop in arg capture progress");
        Ok(Self::Done { id, buff })
      }
    }
  }
}

#[derive(Debug)]
struct ArgCaptureState {
  // the state of the packet capture
  partial_state: PartialCaptureState,
  endmsg_counter: usize,
}

impl Default for ArgCaptureState {
  fn default() -> Self {
    Self {
      endmsg_counter: 0,
      partial_state: PartialCaptureState::Empty,
    }
  }
}

pub fn perform_arg_capture(
  infra: &mut TracingInfra,
  modules: &ExtModuleMap,
  capture_target: &mut ArgPacketDumper,
) -> Result<()> {
  let capture = ArgCapture {
    readers: get_sizetype_readers(),
    capture_target,
  };

  capture
    .run(infra, modules)
    .map_err(|e| e.context("arg_capture"))?;
  Ok(())
}

struct ArgCapture<'a> {
  readers: SizeTypeReaders,
  capture_target: &'a mut ArgPacketDumper,
}

impl CaptureLoopState for ArgCaptureState {
  fn get_end_message_count(&self) -> usize {
    self.endmsg_counter
  }

  fn reset_end_message_count(&mut self) {
    self.endmsg_counter = 0;
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
      state.endmsg_counter += 1;
      return Ok(state);
    }
    while !buff.empty() {
      let partial_state = state
        .partial_state
        .progress(buff, modules, &mut self.readers)?;

      state.partial_state = match partial_state {
        PartialCaptureState::Done { id, mut buff } => {
          // save the received packet
          if let Some(dumper) = self.capture_target.get_packet_dumper(id) {
            dumper.dump(&mut buff)?;
          }
          Log::get("argCap update_from_buffer").trace(format!("{buff:?}"));
          // restart from an empty state
          PartialCaptureState::Empty
        }
        st => st, // continue from the non-Done state
      };
    }

    Ok(state)
  }

  fn process_state(&mut self, state: Self::State) -> Result<Self::State> {
    Ok(state)
  }
}
