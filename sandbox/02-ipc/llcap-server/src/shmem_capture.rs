use std::ffi::CStr;

use libc::{
  __errno_location, MAP_FAILED, MAP_SHARED_VALIDATE, O_CREAT, O_EXCL, O_RDWR, PROT_READ,
  PROT_WRITE, S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, SEM_FAILED, c_void, ftruncate,
  mmap, mode_t, sem_open, sem_t, shm_open, shm_unlink,
};

use crate::log::Log;

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

const PERMS_PERMISSIVE: mode_t = S_IROTH | S_IWOTH | S_IWGRP | S_IRGRP | S_IWUSR | S_IRUSR;

// expects s to be null terminated
unsafe fn to_cstr(s: &String) -> &CStr {
  unsafe { CStr::from_bytes_with_nul_unchecked(s.as_bytes()) }
}

fn try_open_sem(name: &str, n_buffs: u32) -> Result<ShmSem, String> {
  let s_name = format!("{name}\x00");
  let cstr_name = unsafe { to_cstr(&s_name) };
  let result = unsafe {
    sem_open(
      cstr_name.as_ptr(),
      O_CREAT | O_EXCL,
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

pub fn init_semaphores(prefix: &str, n_buffs: u32) -> Result<(ShmSem, ShmSem), String> {
  let free_name = format!("{prefix}semfree");
  let full_name = format!("{prefix}semfull");

  let free_sem = try_open_sem(&free_name, n_buffs)?;
  let full_sem = try_open_sem(&full_name, n_buffs);

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
      Ok(()) => Err("Initialization failed and was successfully undone.".to_string()),
    }
  }
}

pub fn deinit_shmem(meta_mem: ShmHandle, buffers_mem: ShmHandle) -> Result<(), String> {
  try_unmap_shm(meta_mem)
    .map_err(|e| format!("When unmapping meta mem: {e}"))
    .and_then(|_| try_unmap_shm(buffers_mem))
    .map_err(|e| format!("When unmapping buffers mem: {e}"))
}
