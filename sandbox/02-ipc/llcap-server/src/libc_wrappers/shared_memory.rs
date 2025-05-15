use std::{
  ffi::{CStr, c_void},
  marker::PhantomData,
};

use libc::{
  MAP_FAILED, MAP_SHARED_VALIDATE, O_CREAT, O_EXCL, O_RDWR, PROT_READ, PROT_WRITE, ftruncate, mmap,
  munmap, shm_open,
};

use super::{
  fd::try_shm_unlink_fd,
  wrappers::{PERMS_PERMISSIVE, get_errno_string, to_cstr},
};

#[derive(Debug)] // do not derive Clone or Copy!
pub struct ShmemHandle<'a> {
  pub mem: *mut c_void,
  len: u32,
  _fd: i32,
  /// null-char-terminated string
  cname: String,
  marker: PhantomData<&'a u8>,
}

impl ShmemHandle<'_> {
  fn new(mem_ptr: *mut c_void, len: u32, fd: i32, name: String) -> Self {
    assert!(!mem_ptr.is_null());
    assert!(mem_ptr != MAP_FAILED);
    assert!(fd != -1);
    Self {
      mem: mem_ptr,
      len,
      _fd: fd,
      cname: format!("{}\x00", name),
      marker: PhantomData,
    }
  }

  pub fn len(&self) -> u32 {
    self.len
  }

  pub fn try_mmap(path: &CStr, len: u32) -> Result<Self, String> {
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

    Ok(Self::new(
      mmap_res,
      len,
      fd,
      path.to_string_lossy().to_string(),
    ))
  }

  pub fn try_unmap(self) -> Result<(), String> {
    // SAFETY: syscall docs, ShmHandle's invariant
    let unmap_res = unsafe { munmap(self.mem, self.len as usize) };
    if unmap_res != 0 {
      return Err(format!(
        "Failed to unmap memory @ {:?} of len {}: {}",
        self.mem,
        self.len,
        get_errno_string()
      ));
    }
    // SAFETY: cname from type's invariant
    let cstr_name = unsafe { to_cstr(&self.cname) };
    try_shm_unlink_fd(cstr_name)
  }
}
