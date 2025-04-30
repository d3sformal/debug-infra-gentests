use std::{collections::HashMap, ffi::CStr};

use libc::{
  __errno_location, MAP_FAILED, MAP_SHARED_VALIDATE, O_CREAT, O_EXCL, O_RDWR, PROT_READ,
  PROT_WRITE, S_IRWXG, S_IRWXO, S_IRWXU, SEM_FAILED, c_int, c_void, ftruncate, mmap, mode_t,
  sem_open, sem_t, shm_open, shm_unlink,
};

use crate::{
  call_tracing::{FunctionCallInfo, Message, ModIdT},
  log::Log,
  modmap::ExtModuleMap,
};

#[repr(C)]
pub struct MetaDescriptor {
  pub buff_count: u32,
  pub buff_size: u32,
  pub total_len: u32,
}

pub struct ShmSem {
  sem: *mut sem_t,
  cname: String,
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

impl ShmSem {
  pub fn try_post(&mut self) -> Result<(), String> {
    // SAFETY: Self invariant
    if unsafe { libc::sem_post(self.sem) } == -1 {
      Err(format!("Failed post semaphore: {}", get_errno_string()))
    } else {
      Ok(())
    }
  }

  pub fn try_wait(&mut self) -> Result<(), String> {
    // SAFETY: Self invariant
    if unsafe { libc::sem_wait(self.sem) } == -1 {
      Err(format!("Failed wait on semaphore: {}", get_errno_string()))
    } else {
      Ok(())
    }
  }

  pub fn try_close(&mut self) -> Result<(), String> {
    // SAFETY: Self invariant
    if unsafe { libc::sem_close(self.sem) } != 0 {
      Err(format!("Failed to close semaphore: {}", get_errno_string()))
    } else {
      Ok(())
    }
  }

  pub fn try_destroy(self) -> Result<(), (Self, String)> {
    // SAFETY: Self invariant
    if unsafe { libc::sem_unlink(to_cstr(&self.cname).as_ptr()) } != 0 {
      Err((
        self,
        format!("Failed to unlink semaphore: {}", get_errno_string()),
      ))
    } else {
      Ok(())
    }
  }

  // opens a semaphore and returns a valid Self object
  fn try_open(
    name: &str,
    value: u32,
    flags: Option<c_int>,
    mode: Option<mode_t>,
  ) -> Result<Self, String> {
    let s_name = format!("{name}\x00");
    // SAFETY: line above
    let cstr_name = unsafe { to_cstr(&s_name) };
    // SAFETY: &CStr, syscall docs
    let result = unsafe {
      sem_open(
        cstr_name.as_ptr(),
        flags.unwrap_or(O_CREAT | O_EXCL),
        mode.unwrap_or(PERMS_PERMISSIVE),
        value,
      )
    };

    if result == SEM_FAILED {
      Err(format!(
        "Failed to initialize semaphore {}: {}",
        name,
        get_errno_string()
      ))
    } else {
      Ok(Self {
        sem: result,
        cname: s_name,
      })
    }
  }

  fn try_open_exclusive(name: &str, value: u32) -> Result<Self, String> {
    Self::try_open(name, value, (O_CREAT | O_EXCL).into(), None)
  }
}

fn get_errno_string() -> String {
  // SAFETY: entire app single thread for now
  let errno_str: &CStr = unsafe { CStr::from_ptr(libc::strerror(*__errno_location())) };
  let s = match errno_str.to_str() {
    Err(e) => {
      Log::get("get_errno_string").crit(&format!(
        "Cannot convert errno string to utf8 string: {}",
        e
      ));
      return "".to_owned();
    }
    Ok(s) => s.to_owned(),
  };
  s.to_string()
}

const PERMS_PERMISSIVE: mode_t = S_IRWXO | S_IRWXG | S_IRWXU;

/// expects s to be null terminated
unsafe fn to_cstr(s: &String) -> &CStr {
  unsafe { CStr::from_bytes_with_nul_unchecked(s.as_bytes()) }
}

pub fn clean_semaphores(prefix: &str) -> Result<(), String> {
  let free_name = format!("{prefix}semfree");
  let full_name = format!("{prefix}semfull");

  for name in &[free_name, full_name] {
    eprintln!("Cleanup {}", name);
    let res = ShmSem::try_open(name, 0, O_CREAT.into(), None);
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

pub fn init_semaphores(prefix: &str, n_buffs: u32) -> Result<(ShmSem, ShmSem), String> {
  let free_name = format!("{prefix}semfree");
  let full_name = format!("{prefix}semfull");

  let free_sem = ShmSem::try_open_exclusive(&free_name, n_buffs)?;
  let full_sem = ShmSem::try_open_exclusive(&full_name, 0);

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

fn deinit_semaphore_single(mut sem: ShmSem) -> Result<(), String> {
  sem.try_close()?;
  sem.try_destroy().map_err(|(_, s)| s)
}

pub fn deinit_semaphores(free_handle: ShmSem, full_handle: ShmSem) -> Result<(), String> {
  deinit_semaphore_single(free_handle)
    .map_err(|e| format!("When closing free semaphore: {e}"))
    .and_then(|_| deinit_semaphore_single(full_handle))
    .map_err(|e| format!("When closing full semaphore: {e}"))
}

#[derive(Debug)]
pub struct ShmHandle {
  pub mem: *mut c_void,
  len: u32,
  _fd: i32,
  /// null-char-terminated string
  cname: String,
}

impl ShmHandle {
  fn new(mem_ptr: *mut c_void, len: u32, fd: i32, name: String) -> Self {
    assert!(!mem_ptr.is_null());
    assert!(mem_ptr != MAP_FAILED);
    assert!(fd != -1);
    Self {
      mem: mem_ptr,
      len,
      _fd: fd,
      cname: format!("{}\x00", name),
    }
  }
}

fn try_shm_unlink_fd(name: &CStr) -> Result<(), String> {
  // SAFETY: &CStr type, syscall docs
  if unsafe { shm_unlink(name.as_ptr()) } == -1 {
    Err(format!(
      "Failed to unlink FD {}: {}",
      name.to_string_lossy(),
      get_errno_string()
    ))
  } else {
    Ok(())
  }
}

fn try_unmap_shm(handle: ShmHandle) -> Result<(), String> {
  // SAFETY: syscall docs, ShmHandle's invariant
  let unmap_res = unsafe { libc::munmap(handle.mem, handle.len as usize) };
  if unmap_res != 0 {
    return Err(format!(
      "Failed to unmap memory @ {:?} of len {}: {}",
      handle.mem,
      handle.len,
      get_errno_string()
    ));
  }
  // SAFETY: cname from type's invariant
  let cstr_name = unsafe { to_cstr(&handle.cname) };
  try_shm_unlink_fd(cstr_name)
}

fn try_mmap_with_name(path: &CStr, len: u32) -> Result<ShmHandle, String> {
  let unlinking_handler = |error_string: String| {
    let unlink_res =
      try_shm_unlink_fd(path).map_err(|e| format!("{error_string}\n\tWith inner error: {e}"));

    match unlink_res {
      Ok(_) => Err(error_string),
      Err(s) => Err(s),
    }
  };
  // SAFETY: &CStr type, syscall docs
  let fd = unsafe { shm_open(path.as_ptr(), O_CREAT | O_EXCL | O_RDWR, PERMS_PERMISSIVE) };
  if fd == -1 {
    return Err(format!(
      "Failed to open FD for shmem {}: {}",
      path.to_string_lossy(),
      get_errno_string()
    ));
  }
  // SAFETY: documentation of the syscall, fd obtained beforehand
  if unsafe { ftruncate(fd, len as i64) } == -1 {
    return unlinking_handler(format!(
      "Failed to truncate FD for shmem {}, len: {}: {}",
      path.to_string_lossy(),
      len,
      get_errno_string()
    ));
  }

  // SAFETY: documentation of the syscall, fd obtained beforehand, len passed fntruncate above
  let mmap_res = unsafe {
    mmap(
      std::ptr::null_mut(),
      len as usize,
      PROT_READ | PROT_WRITE,
      MAP_SHARED_VALIDATE,
      fd,
      0,
    )
  };
  if mmap_res == MAP_FAILED {
    return unlinking_handler(format!(
      "Failed to mmap {}, len: {}: {}",
      path.to_string_lossy(),
      len,
      get_errno_string()
    ));
  }

  Ok(ShmHandle::new(
    mmap_res,
    len,
    fd,
    path.to_string_lossy().to_string(),
  ))
}

pub fn init_shmem(
  prefix: &str,
  buff_count: u32,
  buff_len: u32,
) -> Result<(ShmHandle, ShmHandle), String> {
  let meta_tmp = format!("{prefix}shmmeta\x00");
  // SAFETY: line above
  let metacstr = unsafe { to_cstr(&meta_tmp) };
  let meta_mem_handle = try_mmap_with_name(metacstr, std::mem::size_of::<MetaDescriptor>() as u32)?;

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

  let result = try_mmap_with_name(buffscstr, buff_count * buff_len);

  if let Ok(res) = result {
    Ok((meta_mem_handle, res))
  } else {
    match try_unmap_shm(meta_mem_handle) {
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

pub fn deinit_shmem(meta_mem: ShmHandle, buffers_mem: ShmHandle) -> Result<(), String> {
  let resmeta = try_unmap_shm(meta_mem).map_err(|e| format!("When unmapping meta mem: {e}"));
  let resbuffs = try_unmap_shm(buffers_mem).map_err(|e| format!("When unmapping buffers mem: {e}"));

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

/// Safety: "reasonable" T (intended for primitive types)
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
  // SAFETY: read_w_alignment_chk performs *const dereference & null check
  // poitner validity ensured by protocol, target type is copied, no allocation over the same memory region
  let valid_size: u32 = unsafe { read_w_alignment_chk(raw_buff) }?;
  if valid_size == 0 {
    state.add_message(Message::ControlEnd);
    return Ok(state);
  }

  let start = raw_buff.wrapping_byte_add(4);
  let buff_end = start.wrapping_byte_add(valid_size as usize);
  raw_buff = start;
  const HASH_SIZE: usize = 64;
  const FNID_SIZE: usize = std::mem::size_of::<u32>();
  while raw_buff < buff_end {
    if raw_buff.is_null() {
      return Err("Null pointer when iterating buffer...".to_string());
    }

    let module_id = if let Some(id) = state.modid_wip {
      // module id from a previous buffer -> skip readnig for module id and instead read function id corresponding to the module id from a previous bufer
      state.modid_wip = None;
      id
    } else if let Ok(s) =
      // SAFETY: protocol ensures length, initialization & immutability of underlying data from the outside, null handled above
      // - alignment is assured by slice type (&[u8], alignment 1)
      // - slice does not span multiple allocated objects
      // - s is used only as read-only for its lifetime (via &str)
      std::str::from_utf8(unsafe { std::slice::from_raw_parts(raw_buff, HASH_SIZE) })
    {
      if let Some(id) = mods.get_module_id(s) {
        id
      } else {
        return Err("Capture invalid, id not found".to_string());
      }
    } else {
      return Err("Capture invalid, hash not a string".to_string());
    };

    raw_buff = raw_buff.wrapping_byte_add(HASH_SIZE);
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

pub fn msg_handler(
  full_sem: &mut ShmSem,
  free_sem: &mut ShmSem,
  buffers: &ShmHandle,
  buff_size: usize,
  buff_num: usize,
  modules: &ExtModuleMap,
  recorded_frequencies: &mut HashMap<FunctionCallInfo, u64>,
) -> Result<(), String> {
  let lg = Log::get("msghandler");
  let mut buff_idx = 0;
  let mut state = CallTraceMessageState::new(None, vec![]);
  loop {
    let sem_res = full_sem.try_wait();
    if let Err(e) = sem_res {
      return Err(format!("While waiting for buffer: {}", e));
    }
    lg.trace("Received buffer");

    let buff_offset = buff_idx * buff_size;
    if buff_offset >= buffers.len as usize {
      return Err(format!(
        "Calculated offset too large: {}, compared to the buffers len {}",
        buff_offset, buffers.len
      ));
    }
    let buff_ptr = (buffers.mem as *const c_void).wrapping_byte_add(buff_offset);

    if buff_ptr.is_null() || buff_ptr < buffers.mem {
      return Err(format!(
        "Buffer pointer is invalid: {:?}, size {buff_size}, num {buff_num}, mem: {:?}",
        buff_ptr, buffers
      ));
    }

    let res: Result<CallTraceMessageState, String> =
      update_from_buffer(buff_ptr as *const u8, buff_size, modules, state);
    if let Ok(mut st) = res {
      let messages = st.extract_messages();
      state = st;

      if let Some(mid) = state.modid_wip {
        lg.trace(&format!("State module id WIP: {mid}"));
      }

      for m in messages {
        match m {
          Message::Normal(content) => {
            recorded_frequencies
              .entry(content)
              .and_modify(|v| *v += 1)
              .or_insert(1);
          }
          Message::ControlEnd => {
            lg.trace("ControlEnd message");
            return Ok(());
          }
        }
      }
    } else {
      return Err(format!("Error when parsing:{}", res.err().unwrap()));
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
