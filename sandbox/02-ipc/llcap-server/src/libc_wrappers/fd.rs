use std::ffi::CStr;

use libc::shm_unlink;

use super::wrappers::get_errno_string;

pub fn try_shm_unlink_fd(name: &CStr) -> Result<(), String> {
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
