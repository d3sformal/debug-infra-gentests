use std::collections::HashMap;

use crate::{
  log::Log,
  modmap::{ExtModuleMap, IntegralModId, MOD_ID_SIZE_B},
  stages::call_tracing::{FunctionCallInfo, Message, ModIdT},
};

use super::{TracingInfra, mem_utils::read_w_alignment_chk};

pub struct CallTraceMessageState {
  modid_wip: Option<ModIdT>,
  messages: Vec<Message>,
}

impl CallTraceMessageState {
  pub fn extract_messages(&mut self) -> Vec<Message> {
    let mut msgs = vec![];
    std::mem::swap(&mut msgs, &mut self.messages);
    msgs
  }

  pub fn add_message(&mut self, msg: Message) {
    self.messages.push(msg);
  }

  pub fn new(module_id: Option<ModIdT>, messages: Vec<Message>) -> Self {
    Self {
      modid_wip: module_id,
      messages,
    }
  }
}

enum Either<T, S> {
  Left(T),
  Right(S),
}

// raw_buff arg: poitner validity must be ensured by protocol, target type is copied, no allocation over the same memory region
// Okay Left = return state, no further processing needed
// Okay Right = [start, end) buffer's data bounds
fn buff_bounds_or_end(
  raw_buff: *const u8,
  state: &mut CallTraceMessageState,
) -> Result<Either<(), (*const u8, *const u8)>, String> {
  // SAFETY: read_w_alignment_chk performs *const dereference & null/alignment check
  let valid_size: u32 = unsafe { read_w_alignment_chk(raw_buff) }?;

  if valid_size == 0 {
    if let Some(m_id) = state.modid_wip {
      return Err(format!(
        "Comms corruption - partial state with empty message following it! Module id {}",
        m_id
      ));
    }

    state.add_message(Message::ControlEnd);
    return Ok(Either::Left(()));
  }

  let start = raw_buff.wrapping_byte_add(std::mem::size_of_val(&valid_size));
  let buff_end = start.wrapping_byte_add(valid_size as usize);
  Ok(Either::Right((start, buff_end)))
}

/// performs a read operation on a given buffer, after this function, no data inside a buffer is relevant to us anymore
fn update_from_buffer(
  mut raw_buff: *const u8,
  _max_size: usize,
  mods: &ExtModuleMap,
  mut state: CallTraceMessageState,
) -> Result<CallTraceMessageState, String> {
  let lg = Log::get("update_from_buffer");
  let (buff_start, buff_end) = match buff_bounds_or_end(raw_buff, &mut state)? {
    Either::Left(()) => return Ok(state),
    Either::Right(v) => v,
  };

  raw_buff = buff_start;
  while raw_buff < buff_end {
    if raw_buff.is_null() {
      return Err("Null pointer when iterating buffer...".to_string());
    }

    let module_id: usize = determine_module_id(&mut raw_buff, mods, &mut state, buff_end)?;

    type FnId = u32;
    const FUNN_ID_SIZE: usize = std::mem::size_of::<FnId>();

    if raw_buff >= buff_end {
      if raw_buff > buff_end {
        lg.warn(format!(
          "Buffer weirdly overdlown buff: {:?} end: {:?}",
          raw_buff, buff_end
        ));
      }
      lg.trace("Partial message");
      state.modid_wip = Some(module_id);
      return Ok(state);
    } else if raw_buff.wrapping_byte_add(FUNN_ID_SIZE) > buff_end {
      return Err(format!(
        "Would overread function ID... ptr: {:?} to read: {} end: {:?}",
        raw_buff, FUNN_ID_SIZE, buff_end
      ));
    }

    // SAFETY: read_w_alignment_chk + similar to buff_bounds_or_end's requirements, raw_buff within bounds as checked above
    let fn_id: FnId = unsafe { read_w_alignment_chk(raw_buff) }?;

    state.add_message(Message::Normal(FunctionCallInfo::new(
      fn_id,
      module_id as usize,
    )));
    raw_buff = raw_buff.wrapping_byte_add(FUNN_ID_SIZE);
  }
  Ok(state)
}

// obtains the "next" module id to process
fn determine_module_id(
  raw_buff: &mut *const u8,
  mods: &ExtModuleMap,
  state: &mut CallTraceMessageState,
  buff_end: *const u8,
) -> Result<usize, String> {
  Ok(if let Some(id) = state.modid_wip {
    // module id from a previous buffer -> skip readnig for module id and instead read function id corresponding to the module id from a previous bufer
    state.modid_wip = None;
    id
  } else {
    if raw_buff.wrapping_byte_add(MOD_ID_SIZE_B) > buff_end {
      return Err(format!(
        "Would over-read a module ID... ptr: {:?} offset: {} end: {:?}",
        raw_buff, MOD_ID_SIZE_B, buff_end
      ));
    }

    // SAFETY: read_w_alignment_chk performs *const dereference & null/alignment check
    // poitner validity ensured by protocol, target type is copied, no allocation over the same memory region
    let rcvd_id = IntegralModId(unsafe { read_w_alignment_chk(*raw_buff)? });
    if let Some(id) = mods.get_module_id(&rcvd_id) {
      *raw_buff = raw_buff.wrapping_byte_add(MOD_ID_SIZE_B);
      id
    } else {
      return Err(format!("Module id not found {:X}", rcvd_id.0));
    }
  })
}

fn wait_for_free_buffer(infra: &mut TracingInfra) -> Result<(), String> {
  let sem_res = infra.sem_full.try_wait();
  if let Err(e) = sem_res {
    Err(format!("While waiting for buffer: {}", e))
  } else {
    Ok(())
  }
}

fn post_free_buffer(infra: &mut TracingInfra, dbg_buff_idx: usize) -> Result<(), String> {
  let sem_res = infra.sem_free.try_post();
  if let Err(e) = sem_res {
    return Err(format!(
      "While posting a free buffer (idx {dbg_buff_idx}): {}",
      e
    ));
  }
  Ok(())
}

// get the start of a buffer at an offset
fn get_buffer_start(infra: &TracingInfra, buff_offset: usize) -> Result<*mut u8, String> {
  let buffers = &infra.backing_buffer;
  if buff_offset >= buffers.len() as usize {
    return Err(format!(
      "Calculated offset too large: {}, compared to the buffers len {}",
      buff_offset,
      buffers.len()
    ));
  }
  let buff_ptr = buffers.mem.wrapping_byte_add(buff_offset) as *mut u8;

  if buff_ptr.is_null() || buff_ptr < buffers.mem as *mut u8 {
    return Err(format!(
      "Buffer pointer is invalid: {:?}, mem: {:?}",
      buff_ptr, buffers
    ));
  }
  Ok(buff_ptr)
}

fn process_messages(
  messages: &Vec<Message>,
  recorded_frequencies: &mut HashMap<FunctionCallInfo, u64>,
) -> Result<(), String> {
  for m in messages {
    match m {
      Message::Normal(content) => {
        recorded_frequencies
          .entry(*content)
          .and_modify(|v| *v += 1)
          .or_insert(1);
      }
      Message::ControlEnd => {
        return Err("Unexpected end message!".to_string());
      }
    }
  }
  Ok(())
}

pub fn msg_handler(
  infra: &mut TracingInfra,
  buff_size: usize,
  buff_num: usize,
  modules: &ExtModuleMap,
  recorded_frequencies: &mut HashMap<FunctionCallInfo, u64>,
) -> Result<(), String> {
  let lg = Log::get("msghandler");
  let mut buff_idx: usize = 0;
  let mut end_message_counter = 0;
  let mut state = CallTraceMessageState::new(None, vec![]);
  loop {
    wait_for_free_buffer(infra)?;

    lg.trace(format!("Received buffer {}", buff_idx));
    let buff_offset = buff_idx * buff_size;
    let buff_ptr = get_buffer_start(infra, buff_offset)?;
    let mut st: CallTraceMessageState =
      update_from_buffer(buff_ptr as *const u8, buff_size, modules, state)?;

    // SAFETY: protocol, the way update_from_buffer interacts with buff_ptr ensures alignment,
    // aliasing is ensured as this buffer should only be handled here
    // Protocol: Set buffer's length to zero
    unsafe {
      *(buff_ptr as *mut u32) = 0;
    }
    post_free_buffer(infra, buff_idx)?;

    let messages = st.extract_messages();
    state = st; // copy st into state (discards st and makes state ready for another iteration)

    if let Some(mid) = state.modid_wip {
      lg.trace(format!("State module id WIP: {mid}"));
    }

    if let Some(Message::ControlEnd) = messages.first() {
      // ending message part of an end sequence should always "be alone" in a message pack
      // (checking only the first is not wrong either)
      end_message_counter += 1;
      if end_message_counter == buff_num {
        lg.trace("End condition");
        return Ok(());
      }
    } else {
      end_message_counter = 0;
      process_messages(&messages, recorded_frequencies)?;
    }

    buff_idx += 1;
    buff_idx %= buff_num;
  }
}
