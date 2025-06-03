use std::collections::HashMap;

use crate::{
  log::Log,
  modmap::{ExtModuleMap, IntegralFnId, IntegralModId},
  shmem_capture::mem_utils::ptr_add_nowrap,
  stages::call_tracing::{FunctionCallInfo, Message},
};

use anyhow::{Result, bail, ensure};

use super::{
  Either, TracingInfra, buff_bounds_or_end,
  mem_utils::{overread_check, read_w_alignment_chk},
};

pub struct CallTraceMessageState {
  mod_id_wip: Option<IntegralModId>,
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

  pub fn new(module_id: Option<IntegralModId>, messages: Vec<Message>) -> Self {
    Self {
      mod_id_wip: module_id,
      messages,
    }
  }
}

/// performs a read operation on a given buffer, after this function, no data inside a buffer is relevant to us anymore
fn update_from_buffer(
  mut raw_buff: *const u8,
  _max_size: usize,
  modules: &ExtModuleMap,
  mut state: CallTraceMessageState,
) -> Result<CallTraceMessageState> {
  let lg = Log::get("update_from_buffer");
  let (buff_start, buff_end) = match buff_bounds_or_end(raw_buff)? {
    Either::Left(()) => {
      if let Some(m_id) = state.mod_id_wip {
        bail!(
          "Comms corruption - partial state with empty message following it! Module id {}",
          *m_id
        );
      }

      state.add_message(Message::ControlEnd);
      return Ok(state);
    }
    Either::Right(v) => v,
  };

  raw_buff = buff_start;
  while raw_buff < buff_end {
    ensure!(!raw_buff.is_null(), "Null pointer when iterating buffer...");

    let mod_id = receive_module_id(&mut raw_buff, &mut state, buff_end)?;
    ensure!(
      modules.get_module_string_id(mod_id).is_some(),
      "Unknown module ID: {}",
      *mod_id
    );

    const FUNC_ID_SIZE: usize = IntegralFnId::byte_size();
    if raw_buff >= buff_end {
      if raw_buff > buff_end {
        lg.warn(format!(
          "Buffer weirdly overflown buff: {:?} end: {:?}",
          raw_buff, buff_end
        ));
      }
      lg.trace("Partial message");
      state.mod_id_wip = Some(mod_id);
      return Ok(state);
    }
    overread_check(raw_buff, buff_end, FUNC_ID_SIZE, "function ID")?;

    // SAFETY: read_w_alignment_chk + similar to buff_bounds_or_end's requirements, raw_buff within bounds as checked above
    let fn_id: u32 = unsafe { read_w_alignment_chk(raw_buff) }?;
    lg.trace(format!("M {:02X}", *mod_id));
    lg.trace(format!("F {:02X}", fn_id));

    state.add_message(Message::Normal(FunctionCallInfo::new(
      IntegralFnId(fn_id),
      mod_id,
    )));
    raw_buff = ptr_add_nowrap(raw_buff, FUNC_ID_SIZE)?;
  }
  Ok(state)
}

// obtains the "next" module id to process
fn receive_module_id(
  raw_buff: &mut *const u8,
  state: &mut CallTraceMessageState,
  buff_end: *const u8,
) -> Result<IntegralModId> {
  Ok(if let Some(mod_id) = state.mod_id_wip {
    // module id from a previous buffer -> skip readnig for module id and instead read function id corresponding to the module id from a previous bufer
    state.mod_id_wip = None;
    mod_id
  } else {
    const MODID_SIZE: usize = IntegralModId::byte_size();
    overread_check(*raw_buff, buff_end, MODID_SIZE, "module ID")?;
    // SAFETY: read_w_alignment_chk performs *const dereference & null/alignment check
    // poitner validity ensured by protocol, target type is copied, no allocation over the same memory region
    let mod_id = IntegralModId(unsafe { read_w_alignment_chk(*raw_buff)? });
    *raw_buff = ptr_add_nowrap(*raw_buff, MODID_SIZE)?;
    mod_id
  })
}

fn process_messages(
  messages: &Vec<Message>,
  recorded_frequencies: &mut HashMap<FunctionCallInfo, u64>,
) -> Result<()> {
  for m in messages {
    match m {
      Message::Normal(content) => {
        recorded_frequencies
          .entry(*content)
          .and_modify(|v| *v += 1)
          .or_insert(1);
      }
      Message::ControlEnd => {
        bail!("Unexpected end message!");
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
) -> Result<HashMap<FunctionCallInfo, u64>> {
  let lg = Log::get("msghandler");
  let mut buff_idx: usize = 0;
  let mut end_message_counter = 0;
  let mut state = CallTraceMessageState::new(None, vec![]);
  let mut recorded_frequencies = HashMap::new();
  loop {
    infra.wait_for_free_buffer()?;

    lg.trace(format!("Received buffer {}", buff_idx));
    let buff_offset = buff_idx * buff_size;
    let buff_ptr = infra.get_buffer_start(buff_offset)?;
    let mut st: CallTraceMessageState =
      update_from_buffer(buff_ptr as *const u8, buff_size, modules, state)?;

    // SAFETY: protocol, the way update_from_buffer interacts with buff_ptr ensures alignment,
    // aliasing is ensured as this buffer should only be handled here, u32@buff_ptr does not need a drop
    // Protocol: Set buffer's length to zero
    unsafe {
      (buff_ptr as *mut u32).write(0);
    }
    infra.post_free_buffer(buff_idx)?;

    let messages = st.extract_messages();
    state = st; // copy st into state (discards st and makes state ready for another iteration)

    if let Some(mid) = state.mod_id_wip {
      lg.trace(format!("State module id WIP: {}", *mid));
    }

    if let Some(Message::ControlEnd) = messages.first() {
      // ending message part of an end sequence should always "be alone" in a message pack
      // (checking only the first is not wrong either)
      end_message_counter += 1;
      if end_message_counter == buff_num {
        lg.trace("End condition");
        return Ok(recorded_frequencies);
      }
    } else {
      end_message_counter = 0;
      process_messages(&messages, &mut recorded_frequencies)?;
    }

    buff_idx += 1;
    buff_idx %= buff_num;
  }
}
