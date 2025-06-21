use std::collections::HashMap;

use crate::{
  log::Log,
  modmap::{ExtModuleMap, IntegralFnId, IntegralModId, NumFunUid},
  shmem_capture::{BorrowedReadBuffer, ReadOnlyBufferPtr},
  stages::call_tracing::Message,
};

use anyhow::{Result, bail, ensure};

use super::TracingInfra;

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

/// Performs a read operation on a given buffer (call tracing data collection)
///
/// After this function, no data inside a buffer is relevant to us anymore
fn update_from_buffer(
  mut raw_buff: BorrowedReadBuffer<'_>,
  modules: &ExtModuleMap,
  mut state: CallTraceMessageState,
) -> Result<CallTraceMessageState> {
  let lg = Log::get("update_from_buffer");
  let buff = &mut raw_buff.buffer;
  if buff.empty() {
    if let Some(m_id) = state.mod_id_wip {
      bail!(
        "Comms corruption - partial state with empty message following it! Module id {}",
        *m_id
      );
    }

    state.add_message(Message::ControlEnd);
    return Ok(state);
  }

  while !buff.empty() {
    let mod_id = receive_module_id(buff, &mut state)?;
    ensure!(
      modules.get_module_string_id(mod_id).is_some(),
      "Unknown module ID: {}",
      mod_id.hex_string()
    );

    if buff.empty() {
      lg.trace("Partial message");
      state.mod_id_wip = Some(mod_id);
      return Ok(state);
    }
    ensure!(
      std::mem::size_of::<u32>() == IntegralFnId::byte_size(),
      "Sanity check on function ID size"
    );
    // (we also assume the cooperating application follows protocol)
    let fn_id: u32 = buff
      .unaligned_shift_num_read()
      .map_err(|e| e.context("funciton id"))?;
    lg.trace(format!("M {:02X}", *mod_id));
    lg.trace(format!("F {:02X}", fn_id));

    state.add_message(Message::Normal(NumFunUid::new(IntegralFnId(fn_id), mod_id)));
  }
  Ok(state)
}

// obtains the "next" module id to process
fn receive_module_id(
  raw_buff: &mut ReadOnlyBufferPtr,
  state: &mut CallTraceMessageState,
) -> Result<IntegralModId> {
  Ok(if let Some(mod_id) = state.mod_id_wip {
    // module id from a previous buffer -> skip readnig for module id and instead read function id corresponding to the module id from a previous bufer
    state.mod_id_wip = None;
    mod_id
  } else {
    ensure!(
      std::mem::size_of::<u32>() == IntegralModId::byte_size(),
      "Sanity check on module ID size"
    );
    let mod_id: u32 = raw_buff
      .unaligned_shift_num_read()
      .map_err(|e| e.context("module ID"))?;
    IntegralModId(mod_id)
  })
}

fn process_messages(
  messages: &Vec<Message>,
  recorded_frequencies: &mut HashMap<NumFunUid, u64>,
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
  modules: &ExtModuleMap,
) -> Result<HashMap<NumFunUid, u64>> {
  let lg = Log::get("msghandler");
  let mut end_message_counter = 0;
  let mut state = CallTraceMessageState::new(None, vec![]);
  let mut recorded_frequencies = HashMap::new();
  loop {
    let base_ptr = infra.wait_for_full_buffer()?;
    lg.trace("Received buffer");
    let mut st = update_from_buffer(base_ptr, modules, state)?;

    infra.finish_buffer()?;

    let messages = st.extract_messages();
    state = st; // copy st into state (discards st and makes state ready for another iteration)

    if let Some(mid) = state.mod_id_wip {
      lg.trace(format!("State module id WIP: {}", *mid));
    }

    if let Some(Message::ControlEnd) = messages.first() {
      // ending message part of an end sequence should always "be alone" in a message pack
      // (checking only the first is not wrong either)
      end_message_counter += 1;
      if end_message_counter == infra.buffer_count() {
        lg.trace("End condition");
        return Ok(recorded_frequencies);
      }
    } else {
      end_message_counter = 0;
      process_messages(&messages, &mut recorded_frequencies)?;
    }
  }
}
