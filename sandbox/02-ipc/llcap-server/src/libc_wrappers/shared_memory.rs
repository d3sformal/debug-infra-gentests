use std::{
  ffi::{c_void, CStr}, io::Error, marker::PhantomData
};

use anyhow::{Result, anyhow, ensure};
use libc::{
  MAP_FAILED, MAP_SHARED_VALIDATE, O_CREAT, O_EXCL, O_RDWR, PROT_READ, PROT_WRITE, ftruncate, mmap,
  munmap, shm_open,
};

use super::{
  fd::try_shm_unlink_fd,
  wrappers::{PERMS_PERMISSIVE, to_cstr},
};

#[derive(Debug)] // do not derive Clone or Copy!
pub struct ShmemHandle<'a> {
  mem: *mut c_void, // declare !Send, !Sync as we expose this pointer in assoc. fn ptr()
  len: u32,
  _fd: i32,
  /// null-char-terminated string
  cname: String,
  marker: PhantomData<&'a [u8]>,
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

  pub fn ptr(&mut self) -> *mut c_void {
    self.mem
  }

  pub fn try_mmap(path: &CStr, len: u32) -> Result<Self> {
    let unlinking_handler = |error_string: String| {
      let unlink_res = try_shm_unlink_fd(path).map_err(|e| e.context(error_string.clone()));

      match unlink_res {
        Ok(_) => anyhow!(error_string),
        Err(s) => anyhow!(s),
      }
    };
    // SAFETY: &CStr type, syscall docs
    let fd = unsafe { shm_open(path.as_ptr(), O_CREAT | O_EXCL | O_RDWR, PERMS_PERMISSIVE) };
    ensure!(
      fd != -1,
      "Failed to open FD for shmem {}: {}",
      path.to_string_lossy(),
      Error::last_os_error()
    );

    // SAFETY: documentation of the syscall, fd obtained beforehand
    let truncation = unsafe { ftruncate(fd, len as i64) };
    ensure!(
      truncation != -1,
      unlinking_handler(format!(
        "Failed to truncate FD for shmem {}, len: {}: {}",
        path.to_string_lossy(),
        len,
        Error::last_os_error()
      ))
    );

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
    ensure!(
      mmap_res != MAP_FAILED,
      unlinking_handler(format!(
        "Failed to mmap {}, len: {}: {}",
        path.to_string_lossy(),
        len,
        Error::last_os_error()
      ))
    );

    Ok(Self::new(
      mmap_res,
      len,
      fd,
      path.to_string_lossy().to_string(),
    ))
  }

  pub fn try_unmap(self) -> Result<()> {
    // SAFETY: syscall docs, ShmHandle's invariant
    let unmap_res = unsafe { munmap(self.mem, self.len as usize) };
    ensure!(
      unmap_res == 0,
      format!(
        "Failed to unmap memory @ {:?} of len {}: {}",
        self.mem,
        self.len,
        Error::last_os_error()
      )
    );
    // SAFETY: cname from type's invariant
    let cstr_name = unsafe { to_cstr(&self.cname) };
    try_shm_unlink_fd(cstr_name)
  }
}
