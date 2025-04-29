use std::{collections::HashMap, ffi::{CStr, CString}, ptr};

use libc::{
  __errno_location, c_void, ftruncate, mmap, mode_t, sem_open, sem_t, shm_open, shm_unlink, MAP_FAILED, MAP_SHARED_VALIDATE, O_CREAT, O_EXCL, O_RDWR, PROT_READ, PROT_WRITE, SEM_FAILED, S_IRGRP, S_IROTH, S_IRUSR, S_IRWXG, S_IRWXO, S_IRWXU, S_IWGRP, S_IWOTH, S_IWUSR
};

use crate::{call_tracing::{FunctionCallInfo, Message}, log::Log, modmap::ExtModuleMap};

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

impl ShmSem {
  fn new(sem_ptr: *mut sem_t, cname: String) -> Self {
    assert!(cname.ends_with('\x00'));
    assert!(sem_ptr != SEM_FAILED);
    assert!(!sem_ptr.is_null());
    Self {
      sem: sem_ptr,
      cname,
    }
  }
  // user must ensure only one of the cloned values will ever be used
  pub fn bitwise_clone(&self) -> Self {
    Self {
      sem: self.sem,
      cname: self.cname.clone(),
    }
  }

  pub fn try_post(&mut self) -> Result<(), String> {
      if unsafe { libc::sem_post(self.sem) } == -1 {
        Err(format!(
          "Failed post semaphore: {}",
          get_errno_string()
        ))
      } else {
        Ok(())
      }
  }

  pub fn try_wait(&mut self) -> Result<(), String> {
    if unsafe { libc::sem_wait(self.sem)} == -1 {
      Err(format!(
        "Failed wait on semaphore: {}",
        get_errno_string()
      ))
    } else {
      Ok(())
    }
  }
}

fn get_errno_string() -> String {
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

// expects s to be null terminated
unsafe fn to_cstr(s: &String) -> &CStr {
  unsafe { CStr::from_bytes_with_nul_unchecked(s.as_bytes()) }
}

// TODO: refactor the bool
fn try_open_sem(name: &str, n_buffs: u32, exclusive: bool) -> Result<ShmSem, String> {
  let s_name = format!("{name}\x00");
  let cstr_name = unsafe { to_cstr(&s_name) };
  let result = unsafe {
    sem_open(
      cstr_name.as_ptr(),
      if exclusive { O_CREAT | O_EXCL } else { O_CREAT },
      PERMS_PERMISSIVE,
      n_buffs,
    )
  };
  if result == SEM_FAILED {
    Err(format!(
      "Failed to initialize semaphore {}: {}",
      name,
      get_errno_string()
    ))
  } else {
    Ok(ShmSem::new(result, s_name))
  }
}

pub fn clean_semaphores(prefix: &str) -> Result<(), String> {
  let free_name = format!("{prefix}semfree");
  let full_name = format!("{prefix}semfull");


  for name in &[free_name, full_name]
  {
    eprintln!("Cleanup {}", name);
    let res = try_open_sem(&name, 0, false);
  if let Ok(sem) = res {
    let _ = deinit_semaphore_single(sem).inspect_err(|e| eprintln!("Cleanup of opened {name}: {e}"));
  } else {
    eprintln!("Cleanup {}: {}", name, res.err().unwrap())
  }}

  let meta_tmp = format!("{prefix}shmmeta\x00");
  let buffs_tmp = format!("{prefix}shmbuffs\x00");

  for name in [unsafe { to_cstr(&meta_tmp) },unsafe { to_cstr(&buffs_tmp) }] {
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

  let free_sem = try_open_sem(&free_name, n_buffs, true)?;
  let full_sem = try_open_sem(&full_name, 0, true);

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

fn close_semaphore_single(sem: &mut ShmSem) -> Result<(), String> {
  if unsafe { libc::sem_close(sem.sem) } != 0 {
    Err(format!("Failed to close semaphore: {}", get_errno_string()))
  } else {
    Ok(())
  }
}

fn deinit_semaphore_single(mut sem: ShmSem) -> Result<(), String> {
  close_semaphore_single(&mut sem)?;

  if unsafe { libc::sem_unlink(to_cstr(&sem.cname).as_ptr()) } != 0 {
    Err(format!(
      "Failed to unlink semaphore: {}",
      get_errno_string()
    ))
  } else {
    Ok(())
  }
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
  let unmap_res = unsafe { libc::munmap(handle.mem, handle.len as usize) };
  if unmap_res != 0 {
    return Err(format!(
      "Failed to unmap memory @ {:?} of len {}: {}",
      handle.mem,
      handle.len,
      get_errno_string()
    ));
  }
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

  let fd = unsafe { shm_open(path.as_ptr(), O_CREAT | O_EXCL | O_RDWR, PERMS_PERMISSIVE) };
  if fd == -1 {
    return Err(format!(
      "Failed to open FD for shmem {}: {}",
      path.to_string_lossy(),
      get_errno_string()
    ));
  }

  if unsafe { ftruncate(fd, len as i64) } == -1 {
    return unlinking_handler(format!(
      "Failed to truncate FD for shmem {}, len: {}: {}",
      path.to_string_lossy(),
      len,
      get_errno_string()
    ));
  }

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
  let metacstr = unsafe { to_cstr(&meta_tmp) };
  let meta_mem_handle = try_mmap_with_name(metacstr, std::mem::size_of::<MetaDescriptor>() as u32)?;

  {
    let target_descriptor = MetaDescriptor {
      buff_count,
      buff_size: buff_len,
      total_len: buff_count * buff_len,
    };
    let meta_ptr = meta_mem_handle.mem as *mut MetaDescriptor;
    unsafe {
      *meta_ptr = target_descriptor;
    }
  }

  let buffs_tmp = format!("{prefix}shmbuffs\x00");
  let buffscstr = unsafe { to_cstr(&buffs_tmp) };

  let result = try_mmap_with_name(buffscstr, buff_count * buff_len);

  if let Ok(res) = result {
    Ok((meta_mem_handle, res))
  } else {
    match try_unmap_shm(meta_mem_handle) {
      Err(e) => Err(format!(
        "Failed to map shared memory, initialization failed: {e}"
      )),
      Ok(()) => Err(format!("Initialization failed and was successfully undone, original error: {}", result.err().unwrap())),
    }
  }
}

pub fn deinit_shmem(meta_mem: ShmHandle, buffers_mem: ShmHandle) -> Result<(), String> {
  try_unmap_shm(meta_mem)
    .map_err(|e| format!("When unmapping meta mem: {e}"))
    .and_then(|_| try_unmap_shm(buffers_mem))
    .map_err(|e| format!("When unmapping buffers mem: {e}"))
}

fn update_from_buffer(mut raw_buff: *const u8, max_size: usize, mods: &ExtModuleMap) -> Result<Vec<Message>, String> {
  if raw_buff.is_null() {
    return Err("Null buffer...".to_string());
  }

  // TODO: alignment assurances for EVERY ptr dereference
  let valid_size = unsafe {*(raw_buff as *const u32)};
  if valid_size == 0 {
    return Ok(vec![Message::ControlEnd]);
  }
  
  let mut result = vec![];
  
  let start = raw_buff.wrapping_byte_add(4);
  let max_size = start.wrapping_byte_add(valid_size as usize);
  raw_buff = start;
  const HASH_SIZE: usize =  64;
  const FNID_SIZE: usize = std::mem::size_of::<u32>();
  while raw_buff < max_size {
    if raw_buff.is_null() {
      return Err("Null pointer when iterating buffer...".to_string());
    }
    let module_id = if let Ok(s) = std::str::from_utf8(unsafe { std::slice::from_raw_parts(raw_buff,HASH_SIZE) }) {
      if let Some(id) = mods.get_module_id(s) {
        id
      } else {
        return Err("Capture invalid, id not found".to_string());
      }
    } else {
      return Err("Capture invalid, hash not a string".to_string());
    };
    
    raw_buff = raw_buff.wrapping_byte_add(HASH_SIZE);
    if raw_buff >= max_size || raw_buff.is_null() {
      return Err(format!("Unexpected end of data at {:?}, max: {:?}, start: {:?}, valid_size {:?}", raw_buff, max_size, start, valid_size));
    }
    
    let fn_id = unsafe { *(raw_buff as *const u32) };

    result.push(Message::Normal(FunctionCallInfo::new(fn_id, module_id as usize)));
    raw_buff = raw_buff.wrapping_byte_add(FNID_SIZE);
  }

  return Ok(result);
}


pub fn msg_handler(full_sem: &mut ShmSem, free_sem: &mut ShmSem, buffers: &ShmHandle, buff_size: usize, buff_num: usize, modules: &ExtModuleMap, recorded_frequencies: &mut HashMap<FunctionCallInfo, u64>,) -> Result<(), String> {
  let mut buff_idx = 0;
  loop {
    let sem_res = full_sem.try_wait();
    if let Err(e) =  sem_res {
      return Err(format!("While waiting for buffer: {}", e));
    }

    let buff_offset = buff_idx * buff_size;
    if buff_offset >= buffers.len as usize {
      return Err(format!("Calculated offset too large: {}, compared to the buffers len {}", buff_offset, buffers.len));
    }
    let buff_ptr = (buffers.mem as *const c_void).wrapping_byte_add(buff_offset);

    if buff_ptr.is_null() || buff_ptr < buffers.mem {
      return Err(format!("Buffer pointer is invalid: {:?}, size {buff_size}, num {buff_num}, mem: {:?}", buff_ptr, buffers));
    }
    

    let res: Result<Vec<Message>, String>= update_from_buffer(buff_ptr as *const u8, buff_size, modules);
    if let Ok(messages) = res {
      for m in messages {
        match m {
            Message::Normal(content) => {
              recorded_frequencies.entry(content)
              .and_modify(|v| *v += 1)
              .or_insert(1);
            }
            Message::ControlEnd => {
              println!("end");
              return Ok(());
            }
        }
      }
    } else {
      return Err(format!("Error when parsing:{}", res.err().unwrap()));
    }

    let sem_res = free_sem.try_post();
    if let Err(e) =  sem_res {
      return Err(format!("While posting a free buffer (idx {buff_idx}): {}", e));
    }

    buff_idx += 1;
    buff_idx %= buff_num;
  }
} 