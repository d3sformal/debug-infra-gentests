use std::ptr::slice_from_raw_parts;

use crate::{
  log::Log,
  modmap::{ExtModuleMap, IntegralModId, MOD_ID_SIZE_B},
  shmem_capture::mem_utils::overread_check,
  sizetype_handlers::{
    ArgSizeTypeRef, CStringTypeReader, CustomTypeReader, FixedSizeTyData, FixedSizeTyReader,
    ReadProgress, SizeTypeReader,
  },
};

use super::{
  Either, TracingInfra, buff_bounds_or_end, get_buffer_start, mem_utils::read_w_alignment_chk,
  post_free_buffer, wait_for_free_buffer,
};

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
  GotModuleIdx {
    module_idx: usize,
  },
  CapturingArgs {
    module_idx: usize,
    fn_id: u32,
    arg_idx: usize,
    buff: Vec<u8>,
  },
  Done {
    module_idx: usize,
    fn_id: u32,
    buff: Vec<u8>,
  },
}

impl PartialCaptureState {
  // TODO split up
  pub fn progress(
    self,
    raw_buff: &mut *const u8,
    mods: &ExtModuleMap,
    buff_end: *const u8,
    readers: &mut SizeTypeReaders,
  ) -> Result<Self, String> {
    match self {
      Self::Empty => {
        let lg = Log::get("progress::Empty");
        lg.trace("Starting");
        if !(*raw_buff as *const u32).is_aligned() {
          lg.trace("Adjusting alignment");
          *raw_buff = raw_buff.wrapping_add(raw_buff.align_offset(std::mem::size_of::<u32>()));
          return Ok(Self::Empty);
        }

        overread_check(*raw_buff, buff_end, MOD_ID_SIZE_B, "module ID")?;
        let rcvd_id = IntegralModId(unsafe { read_w_alignment_chk(*raw_buff)? });
        if let Some(idx) = mods.get_module_idx(&rcvd_id) {
          *raw_buff = raw_buff.wrapping_byte_add(MOD_ID_SIZE_B);
          lg.trace(format!("mod Id: {:X}", rcvd_id.0));
          Ok(Self::GotModuleIdx { module_idx: idx })
        } else {
          Err(format!("Module id not found {:X}", rcvd_id.0))
        }
      }
      Self::GotModuleIdx { module_idx } => {
        let lg = Log::get("progress::GotModuleIdx");
        lg.trace("Awaiting fnid!");
        type FnId = u32;
        const FUN_ID_SIZE: usize = std::mem::size_of::<FnId>();
        overread_check(*raw_buff, buff_end, FUN_ID_SIZE, "function ID")?;

        let rcvd_id = unsafe { read_w_alignment_chk(*raw_buff)? };
        match mods.get_function_name(module_idx, rcvd_id) {
          Some(_) => {
            *raw_buff = raw_buff.wrapping_byte_add(FUN_ID_SIZE);
            lg.trace(format!("fn Id: {}", rcvd_id));
            Ok(Self::CapturingArgs {
              module_idx: module_idx,
              fn_id: rcvd_id,
              arg_idx: 0,
              buff: Vec::new(),
            })
          }
          None => Err(format!(
            "Function id not found @ module {}: {}",
            module_idx, rcvd_id
          )),
        }
      }
      Self::CapturingArgs {
        module_idx,
        fn_id,
        arg_idx,
        mut buff,
      } => {
        let lg = Log::get("progress::Args");
        lg.trace("Capturing arguments!");
        let size_refs = mods.get_function_arg_size_descriptors(module_idx, fn_id);
        if size_refs.is_none() {
          return Err(format!(
            "Unknown function with module idx {} fn id {}",
            module_idx, fn_id
          ));
        }
        let size_refs = size_refs.unwrap();
        if arg_idx > size_refs.len() {
          return Err(format!(
            "Argument index overflow {} for args {:?} in ID m/f {}/{}",
            arg_idx, size_refs, module_idx, fn_id
          ));
        }

        for (i, desc) in size_refs.iter().enumerate().skip(arg_idx) {
          lg.trace(format!("Argument idx: {}, desc: {:?}", i, desc));
          let reader = readers.get_reader(*desc);
          if reader.is_none() {
            return Err(format!(
              "Unexpected size type that is missing a reader {:?}",
              desc
            ));
          }
          if buff_end < *raw_buff {
            return Err(format!(
              "Buffer overflow at idx {i}, m/f id {module_idx} {fn_id}"
            ));
          }
          if buff_end == *raw_buff {
            return Ok(Self::CapturingArgs {
              module_idx,
              fn_id,
              arg_idx: i,
              buff,
            });
          }

          let reader = reader.unwrap();
          let slice = slice_from_raw_parts(*raw_buff, buff_end as usize - ((*raw_buff) as usize));

          match reader.read(unsafe { &*slice })? {
            ReadProgress::Done {
              mut payload,
              consumed_bytes,
            } => {
              lg.trace(format!(
                "Reader {} done with {consumed_bytes}-byte payload:",
                i
              ));
              for p in &payload {
                print!("{:02X}", p);
              }
              println!();
              buff.append(&mut payload);
              *raw_buff = raw_buff.wrapping_add(consumed_bytes);
            }
            ReadProgress::NotYet => {
              *raw_buff = buff_end;
              return Ok(Self::CapturingArgs {
                module_idx,
                fn_id,
                arg_idx: i,
                buff,
              });
            }
            ReadProgress::Nop => {
              *raw_buff = buff_end;
              return Ok(Self::CapturingArgs {
                module_idx,
                fn_id,
                arg_idx: i,
                buff,
              });
            }
          }

          if !reader.done() {
            return Err("Sanity check failed!".to_string());
          }
          lg.trace(format!("Resetting reader {}", i));
          reader.read_reset();
        }
        Ok(Self::Done {
          module_idx,
          fn_id,
          buff,
        })
      }
      Self::Done {
        module_idx,
        fn_id,
        buff,
      } => {
        let lg = Log::get("progress::Done");
        lg.warn("Noop in arg capture progress");
        Ok(Self::Done {
          module_idx,
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
  payload: Vec<(usize, u32, Vec<u8>)>,
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
  fn extract_massages(mut self) -> (Vec<(usize, u32, Vec<u8>)>, Self) {
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
) -> Result<ArgCaptureState, String> {
  let (buff_start, buff_end) = match buff_bounds_or_end(raw_buff)? {
    Either::Left(()) => {
      if state.partial_state != PartialCaptureState::Empty {
        return Err(format!(
          "Comms corruption - partial state with empty message following it! Partial state: {:?}",
          state.partial_state
        ));
      }
      state.endmessage_counter += 1;
      return Ok(state);
    }
    Either::Right(v) => v,
  };

  raw_buff = buff_start;
  while raw_buff < buff_end {
    if raw_buff.is_null() {
      return Err("Null pointer when iterating buffer...".to_string());
    }

    let partial_state = state
      .partial_state
      .progress(&mut raw_buff, mods, buff_end, readers)?;

    state.partial_state = match partial_state {
      PartialCaptureState::Done {
        module_idx,
        fn_id,
        buff,
      } => {
        state.payload.push((module_idx, fn_id, buff));
        PartialCaptureState::Empty
      }
      st => st,
    };
  }
  Ok(state)
}

// TODO rename
pub fn wip_capture_args(
  infra: &mut TracingInfra,
  buff_size: usize,
  buff_num: usize,
  modules: &ExtModuleMap,
) -> Result<Vec<(usize, u32, Vec<u8>)>, String> {
  let lg = Log::get("arg_capture");
  let mut cache = create_sizetype_cache();

  let mut buff_idx: usize = 0;
  let mut state = ArgCaptureState::default();
  let mut results = vec![];
  loop {
    wait_for_free_buffer(infra)?;

    lg.trace(format!("Received buffer {}", buff_idx));
    let buff_offset = buff_idx * buff_size;
    let buff_ptr = get_buffer_start(infra, buff_offset)?;
    let st: ArgCaptureState =
      update_from_buffer(buff_ptr as *const u8, buff_size, modules, state, &mut cache)?;

    // Protocol: Set buffer's length to zero
    unsafe {
      (buff_ptr as *mut u32).write(0);
    }
    post_free_buffer(infra, buff_idx)?;

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
