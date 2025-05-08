use std::collections::HashMap;

use libc::{O_CREAT, c_void};

use crate::{
  call_tracing::{FunctionCallInfo, Message, ModIdT},
  libc_wrappers::{
    fd::try_shm_unlink_fd, sem::Semaphore, shared_memory::ShmemHandle, wrappers::to_cstr,
  },
  log::Log,
  modmap::{ExtModuleMap, IntegralModId, MOD_ID_SIZE_B},
};

#[repr(C)]
pub struct MetaDescriptor {
  pub buff_count: u32,
  pub buff_size: u32,
  pub total_len: u32,
}
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

pub fn clean_semaphores(prefix: &str) -> Result<(), String> {
  let free_name = format!("{prefix}semfree");
  let full_name = format!("{prefix}semfull");

  for name in &[free_name, full_name] {
    eprintln!("Cleanup {}", name);
    let res = Semaphore::try_open(name, 0, O_CREAT.into(), None);
    if let Ok(sem) = res {
      let _ =
        deinit_semaphore_single(sem).inspect_err(|e| eprintln!("Cleanup of opened {name}: {e}"));
    } else {
      eprintln!("Cleanup {}: {}", name, res.err().unwrap())
    }
  }

  let meta_tmp = format!("{prefix}shmmeta\x00");
  let buffs_tmp = format!("{prefix}shmbuffs\x00");
  // SAFETY: line above
  for name in [unsafe { to_cstr(&meta_tmp) }, unsafe {
    to_cstr(&buffs_tmp)
  }] {
    eprintln!("Cleanup {:?}", name);
    if let Err(e) = try_shm_unlink_fd(name) {
      eprintln!("Cleanup {:?}: {e}", name);
    }
  }

  Ok(())
}

pub fn init_semaphores(prefix: &str, n_buffs: u32) -> Result<(Semaphore, Semaphore), String> {
  let free_name = format!("{prefix}semfree");
  let full_name = format!("{prefix}semfull");

  let free_sem = Semaphore::try_open_exclusive(&free_name, n_buffs)?;
  let full_sem = Semaphore::try_open_exclusive(&full_name, 0);

  if let Err(e) = full_sem {
    match deinit_semaphore_single(free_sem) {
      Ok(()) => Err(e),
      Err(e2) => Err(format!(
        "Failed cleanup after FULL semaphore init failure: {e2}, init failure: {e}"
      )),
    }
  } else {
    Ok((free_sem, full_sem.unwrap()))
  }
}

fn deinit_semaphore_single(sem: Semaphore) -> Result<(), String> {
  match sem.try_close() {
    Ok(sem) => sem,
    Err((_, err)) => return Err(err),
  }
  .try_destroy()
  .map_err(|(_, s)| s)
}

pub fn deinit_semaphores(free_handle: Semaphore, full_handle: Semaphore) -> Result<(), String> {
  deinit_semaphore_single(free_handle)
    .map_err(|e| format!("When closing free semaphore: {e}"))
    .and_then(|_| deinit_semaphore_single(full_handle))
    .map_err(|e| format!("When closing full semaphore: {e}"))
}

pub fn init_shmem(
  prefix: &str,
  buff_count: u32,
  buff_len: u32,
) -> Result<(ShmemHandle, ShmemHandle), String> {
  let meta_tmp = format!("{prefix}shmmeta\x00");
  // SAFETY: line above
  let metacstr = unsafe { to_cstr(&meta_tmp) };
  let meta_mem_handle =
    ShmemHandle::try_mmap(metacstr, std::mem::size_of::<MetaDescriptor>() as u32)?;

  {
    let target_descriptor = MetaDescriptor {
      buff_count,
      buff_size: buff_len,
      total_len: buff_count * buff_len,
    };
    aligned_to::<MetaDescriptor>(meta_mem_handle.mem as *const u8)?;
    // SAFETY: line above
    unsafe {
      *(meta_mem_handle.mem as *mut MetaDescriptor) = target_descriptor;
    }
  }

  let buffs_tmp = format!("{prefix}shmbuffs\x00");
  // SAFETY: line above
  let buffscstr = unsafe { to_cstr(&buffs_tmp) };

  let result = ShmemHandle::try_mmap(buffscstr, buff_count * buff_len);

  if let Ok(res) = result {
    Ok((meta_mem_handle, res))
  } else {
    match meta_mem_handle.try_unmap() {
      Err(e) => Err(format!(
        "Failed to map shared memory, initialization failed: {e}"
      )),
      Ok(()) => Err(format!(
        "Initialization failed and was successfully undone, original error: {}",
        result.err().unwrap()
      )),
    }
  }
}

pub fn deinit_shmem(meta_mem: ShmemHandle, buffers_mem: ShmemHandle) -> Result<(), String> {
  let resmeta = meta_mem
    .try_unmap()
    .map_err(|e| format!("When unmapping meta mem: {e}"));
  let resbuffs = buffers_mem
    .try_unmap()
    .map_err(|e| format!("When unmapping buffers mem: {e}"));

  if resmeta.is_err() || resbuffs.is_err() {
    let err_str = |res: Result<(), String>| {
      if let Err(e) = res { e } else { "".to_owned() }
    };
    let meta_err = err_str(resmeta);
    let buffs_err = err_str(resbuffs);

    Err("Shmem deinit errors: ".to_string() + &meta_err + " " + &buffs_err)
  } else {
    Ok(())
  }
}

fn aligned_to<T>(ptr: *const u8) -> Result<(), String> {
  if !(ptr as *const T).is_aligned() {
    Err(format!(
      "Pointer {:?} unaligned to type of size {}",
      ptr,
      std::mem::size_of::<T>()
    ))
  } else {
    Ok(())
  }
}

/// SAFETY: "reasonable" T (intended for primitive types)
/// performs a *const dereference
unsafe fn read_w_alignment_chk<T: Copy>(ptr: *const u8) -> Result<T, String> {
  aligned_to::<T>(ptr)?;
  if ptr.is_null() {
    return Err("Null ptr encountered".to_owned());
  }
  // SAFETY: line above + Copy does not create aliased memory
  Ok(unsafe { *(ptr as *const T) })
}

fn update_from_buffer(
  mut raw_buff: *const u8,
  _max_size: usize,
  mods: &ExtModuleMap,
  mut state: CallTraceMessageState,
) -> Result<CallTraceMessageState, String> {
  let lg = Log::get("update_from_buffer");
  // SAFETY: read_w_alignment_chk performs *const dereference & null/alignment check
  // poitner validity ensured by protocol, target type is copied, no allocation over the same memory region
  let valid_size: u32 = unsafe { read_w_alignment_chk(raw_buff) }?;

  if valid_size == 0 {
    if let Some(m_id) = state.modid_wip {
      return Err(format!(
        "Comms corrupt - partial state with empty message following it! Module id {}",
        m_id
      ));
    }

    state.add_message(Message::ControlEnd);
    return Ok(state);
  }

  let start = raw_buff.wrapping_byte_add(4);
  let buff_end = start.wrapping_byte_add(valid_size as usize);
  raw_buff = start;
  while raw_buff < buff_end {
    if raw_buff.is_null() {
      return Err("Null pointer when iterating buffer...".to_string());
    }

    let module_id: usize = determine_module_id(&mut raw_buff, mods, &mut state, buff_end)?;

    const FNID_SIZE: usize = std::mem::size_of::<u32>();
    if raw_buff >= buff_end {
      if raw_buff > buff_end {
        lg.warn(&format!(
          "Buffer weirdly overdlown buff: {:?} end: {:?}",
          raw_buff, buff_end
        ));
      }
      lg.trace("Buffer ended mid-message");
      state.modid_wip = Some(module_id);
      return Ok(state);
    } else if raw_buff.wrapping_byte_add(FNID_SIZE) > buff_end {
      return Err(format!(
        "Would overread function ID... ptr: {:?} offs: {} end: {:?}",
        raw_buff, MOD_ID_SIZE_B, buff_end
      ));
    }

    // SAFETY: same as function start raw_buff within bounds as checked above
    let fn_id: u32 = unsafe { read_w_alignment_chk(raw_buff) }?;

    state.add_message(Message::Normal(FunctionCallInfo::new(
      fn_id,
      module_id as usize,
    )));
    raw_buff = raw_buff.wrapping_byte_add(FNID_SIZE);
  }
  Ok(state)
}

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

pub fn msg_handler(
  full_sem: &mut Semaphore,
  free_sem: &mut Semaphore,
  buffers: &ShmemHandle,
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
    let sem_res = full_sem.try_wait();
    if let Err(e) = sem_res {
      return Err(format!("While waiting for buffer: {}", e));
    }
    lg.trace(&format!("Received buffer {}", buff_idx));
    let buff_offset = buff_idx * buff_size;
    if buff_offset >= buffers.len() as usize {
      return Err(format!(
        "Calculated offset too large: {}, compared to the buffers len {}",
        buff_offset,
        buffers.len()
      ));
    }
    let buff_ptr = (buffers.mem as *const c_void).wrapping_byte_add(buff_offset) as *mut c_void;

    if buff_ptr.is_null() || buff_ptr < buffers.mem {
      return Err(format!(
        "Buffer pointer is invalid: {:?}, size {buff_size}, num {buff_num}, mem: {:?}",
        buff_ptr, buffers
      ));
    }

    let res: Result<CallTraceMessageState, String> =
      update_from_buffer(buff_ptr as *const u8, buff_size, modules, state);
    if let Ok(mut st) = res {
      // SAFETY: protocol, the way update_from_buffer interacts with buff_ptr ensures alignment,
      // aliasing is ensured as this buffer should only be handled here
      // Protocol: Set buffer's length to zero
      unsafe {
        *(buff_ptr as *mut u32) = 0;
      }

      let messages = st.extract_messages();
      state = st;

      if let Some(mid) = state.modid_wip {
        lg.trace(&format!("State module id WIP: {mid}"));
      }

      if let Some(Message::ControlEnd) = messages.first() {
        end_message_counter += 1;
        if end_message_counter == buff_num {
          lg.trace("End condition");
          return Ok(());
        }
      } else {
        end_message_counter = 0;
        for m in messages {
          match m {
            Message::Normal(content) => {
              recorded_frequencies
                .entry(content)
                .and_modify(|v| *v += 1)
                .or_insert(1);
            }
            Message::ControlEnd => {
              return Err("Unexpected end message!".to_string());
            }
          }
        }
      }
    } else {
      return Err(format!("Error when parsing: {}", res.err().unwrap()));
    }

    let sem_res = free_sem.try_post();
    if let Err(e) = sem_res {
      return Err(format!(
        "While posting a free buffer (idx {buff_idx}): {}",
        e
      ));
    }

    buff_idx += 1;
    buff_idx %= buff_num;
  }
}
