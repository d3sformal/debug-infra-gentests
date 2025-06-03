use std::{mem::MaybeUninit, ptr::slice_from_raw_parts};

use crate::{
  log::Log,
  modmap::{ExtModuleMap, IntegralFnId, IntegralModId},
  shmem_capture::mem_utils::{overread_check, ptr_add_nowrap},
  sizetype_handlers::{
    ArgSizeTypeRef, CStringTypeReader, CustomTypeReader, FixedSizeTyData, FixedSizeTyReader,
    ReadProgress, SizeTypeReader,
  },
  stages::arg_capture::ArgPacketDumper,
};
use anyhow::{Result, ensure};

use super::{Either, TracingInfra, buff_bounds_or_end, mem_utils::read_w_alignment_chk};

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

// SAFETY: raw_buff points to 4 bytes of valid data
unsafe fn read_integral_id_from_unaligned<T: From<u32>>(raw_buff: *const u8) -> Result<T> {
  let mut uninit_id = MaybeUninit::<u32>::uninit();
  // SAFETY: from_le_bytes requrides [u8; N], so read will be align-valid
  // as long as raw_buff is valid
  uninit_id.write(u32::from_le_bytes(unsafe {
    read_w_alignment_chk(raw_buff)?
  }));
  // SAFETY: read performed filled all the bytes of the MaybeUninit
  let rcvd_id = T::from(unsafe { uninit_id.assume_init() });
  Ok(rcvd_id)
}

impl PartialCaptureState {
  // SAFETY of all non-pub progress_* functions is based on safety of fn progress

  unsafe fn progress_helper_read_u32<T: From<u32>>(
    raw_buff: *const u8,
    buff_end: *const u8,
  ) -> Result<T> {
    overread_check(
      raw_buff,
      buff_end,
      std::mem::size_of::<u32>(),
      "read ID u32",
    )?;
    // SAFETY: line above
    unsafe { read_integral_id_from_unaligned(raw_buff) }
  }

  fn progress_get_module_id(
    raw_buff: &mut *const u8,
    mods: &ExtModuleMap,
    buff_end: *const u8,
  ) -> Result<Self> {
    let lg = Log::get("progress_get_module_id");
    lg.trace("Start");
    // SAFETY: progress-* above
    let rcvd_id: IntegralModId = unsafe { Self::progress_helper_read_u32(*raw_buff, buff_end) }?;
    *raw_buff = ptr_add_nowrap(*raw_buff, std::mem::size_of::<u32>())?;

    ensure!(
      mods.get_module_string_id(rcvd_id).is_some(),
      "Module ID {} is unknown",
      *rcvd_id
    );
    lg.trace(format!("Mod Id: {}", *rcvd_id));
    Ok(Self::GotModuleId { module_id: rcvd_id })
  }

  fn progress_read_fn_id(
    raw_buff: &mut *const u8,
    mods: &ExtModuleMap,
    buff_end: *const u8,
    module_id: IntegralModId,
  ) -> Result<Self> {
    let lg = Log::get("progress::progress_read_fn_id");
    lg.trace("Reading fnid");
    // SAFETY: progress-* above
    let fn_id: IntegralFnId = unsafe { Self::progress_helper_read_u32(*raw_buff, buff_end) }?;
    *raw_buff = ptr_add_nowrap(*raw_buff, std::mem::size_of::<u32>())?;

    ensure!(
      mods.get_function_name(module_id, fn_id).is_some(),
      "Function id not found @ module {:02X}: {:02X}",
      *module_id,
      *fn_id
    );
    lg.trace(format!("Fnc Id: {}", *fn_id));
    Ok(Self::CapturingArgs {
      module_id,
      fn_id,
      arg_idx: 0,
      buff: Vec::new(),
    })
  }

  fn progress_read_args(
    raw_buff: &mut *const u8,
    mods: &ExtModuleMap,
    buff_end: *const u8,
    readers: &mut SizeTypeReaders,
    module_id: IntegralModId,
    fn_id: IntegralFnId,
    arg_idx: usize,
    mut buff: Vec<u8>,
  ) -> Result<Self> {
    let lg = Log::get("progress_read_args");
    lg.trace("Capturing arguments!");
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
      let reader = readers.get_reader(*desc);
      ensure!(
        reader.is_some(),
        "Unexpected size type that is missing a reader {:?}",
        desc
      );
      ensure!(
        buff_end >= *raw_buff,
        "Buffer overflow at idx {i}, m/f id {} {}",
        *module_id,
        *fn_id
      );
      if buff_end == *raw_buff {
        return Ok(Self::CapturingArgs {
          module_id,
          fn_id,
          arg_idx: i,
          buff,
        });
      }

      let len = buff_end as usize - ((*raw_buff) as usize);
      let reader = reader.unwrap();
      // SAFETY: validity of len calculation guaranteed by ifs above, validity of ptrs themselves by the function SAFETY, calculation of lenght by ::<u8> whose size is 1
      let slice = unsafe { slice_from_raw_parts::<u8>(*raw_buff, len).as_ref() };
      ensure!(
        slice.is_some(),
        "Safety invariant violated on ptr range [{:?} {:?}] len: {len}",
        raw_buff,
        buff_end
      );
      match reader.read(slice.unwrap())? {
        ReadProgress::Done {
          mut payload,
          consumed_bytes,
        } => {
          buff.append(&mut payload);
          *raw_buff = raw_buff.wrapping_add(consumed_bytes);
        }
        ReadProgress::NotYet => {
          *raw_buff = buff_end;
          return Ok(Self::CapturingArgs {
            module_id,
            fn_id,
            arg_idx: i,
            buff,
          });
        }
        ReadProgress::Nop => {
          *raw_buff = buff_end;
          return Ok(Self::CapturingArgs {
            module_id,
            fn_id,
            arg_idx: i,
            buff,
          });
        }
      }

      ensure!(reader.done(), "Sanity check failed");
      lg.trace(format!("Resetting reader {}", i));
      reader.read_reset();
    }
    Ok(Self::Done {
      module_id,
      fn_id,
      buff,
    })
  }

  /// SAFETY: raw_buff and buff_end are not null & pointers to the same data buffer with raw_buff >= buff_end
  pub unsafe fn progress(
    self,
    raw_buff: &mut *const u8,
    mods: &ExtModuleMap,
    buff_end: *const u8,
    readers: &mut SizeTypeReaders,
  ) -> Result<Self> {
    debug_assert!(!raw_buff.is_null());
    debug_assert!(!buff_end.is_null());
    debug_assert!(buff_end as usize >= *raw_buff as usize);
    match self {
      Self::Empty => Self::progress_get_module_id(raw_buff, mods, buff_end),
      Self::GotModuleId { module_id } => {
        Self::progress_read_fn_id(raw_buff, mods, buff_end, module_id)
      }
      Self::CapturingArgs {
        module_id,
        fn_id,
        arg_idx,
        buff,
      } => Self::progress_read_args(
        raw_buff, mods, buff_end, readers, module_id, fn_id, arg_idx, buff,
      ),
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
  fn extract_massages(mut self) -> (Vec<(IntegralModId, IntegralFnId, Vec<u8>)>, Self) {
    let mut res = vec![];
    std::mem::swap(&mut res, &mut self.payload);
    (res, self)
  }
}

fn update_from_buffer(
  mut raw_buff: *const u8,
  _max_size: usize,
  mods: &ExtModuleMap,
  mut state: ArgCaptureState,
  readers: &mut SizeTypeReaders,
  capture_target: &mut ArgPacketDumper,
) -> Result<ArgCaptureState> {
  let (buff_start, buff_end) = match buff_bounds_or_end(raw_buff)? {
    Either::Left(()) => {
      ensure!(
        state.partial_state == PartialCaptureState::Empty,
        "Comms corruption - partial state with empty message following it! Partial state: {:?}",
        state.partial_state
      );
      state.endmessage_counter += 1;
      return Ok(state);
    }
    Either::Right(v) => v,
  };

  raw_buff = buff_start;
  while raw_buff < buff_end {
    ensure!(
      !raw_buff.is_null(),
      "Null pointer when iterating a received buffer"
    );

    // SAFETY: buff_bounds_or_end returns valid pointers + check above
    // + protocol ensures that once acquired, buffer is not changed
    // until "freed" by code running on this very path
    let partial_state = unsafe {
      state
        .partial_state
        .progress(&mut raw_buff, mods, buff_end, readers)?
    };

    state.partial_state = match partial_state {
      PartialCaptureState::Done {
        module_id: mod_id,
        fn_id,
        mut buff,
      } => {
        if let Some(dumper) = capture_target.get_packet_dumper(mod_id, fn_id) {
          dumper
            .write(&mut (buff.len() as u32).to_le_bytes())
            .map_err(|e| {
              e.context(format!(
                "Packet len dumping failed for {} mod {}",
                fn_id.hex_string(),
                mod_id.hex_string()
              ))
            })?;
          dumper.write(&mut buff).map_err(|e| {
            e.context(format!(
              "Packet content dumping failed for {} mod {}",
              fn_id.hex_string(),
              mod_id.hex_string()
            ))
          })?;
        }
        Log::get("AT update_from_buffer").trace(format!("{:?}", buff));
        state.payload.push((mod_id, fn_id, buff));
        PartialCaptureState::Empty
      }
      st => st,
    };
  }
  Ok(state)
}

pub fn perform_arg_capture(
  infra: &mut TracingInfra,
  buff_size: usize,
  buff_num: usize,
  modules: &ExtModuleMap,
  capture_target: &mut ArgPacketDumper,
) -> Result<Vec<(IntegralModId, IntegralFnId, Vec<u8>)>> {
  let lg = Log::get("arg_capture");
  let mut cache = create_sizetype_cache();

  let mut buff_idx: usize = 0;
  let mut state = ArgCaptureState::default();
  let mut results = vec![];
  loop {
    infra.wait_for_free_buffer()?;

    lg.trace(format!("Received buffer {}", buff_idx));
    let buff_offset = buff_idx * buff_size;
    let buff_ptr = infra.get_buffer_start(buff_offset)?;
    let st: ArgCaptureState = update_from_buffer(
      buff_ptr as *const u8,
      buff_size,
      modules,
      state,
      &mut cache,
      capture_target,
    )?;

    // Protocol: Set buffer's length to zero
    // SAFETY: get_buffer_start returns valid pointers to at least u32
    unsafe {
      (buff_ptr as *mut u32).write(0);
    }
    infra.post_free_buffer(buff_idx)?;

    let (mut msgs, new_state) = st.extract_massages();
    state = new_state;
    results.append(&mut msgs);

    if state.endmessage_counter == buff_num {
      lg.trace("End condition");
      return Ok(results);
    }

    buff_idx += 1;
    buff_idx %= buff_num;
  }
}
