use std::ffi::CStr;

use libc::shm_unlink;

use super::wrappers::get_errno_string;
use anyhow::{Result, ensure};

pub fn try_shm_unlink_fd(name: &CStr) -> Result<()> {
  ensure!(
    // SAFETY: &CStr type, syscall docs
    unsafe { shm_unlink(name.as_ptr()) } != -1,
    "Failed to unlink FD {}: {}",
    name.to_string_lossy(),
    get_errno_string()
  );
  Ok(())
}
