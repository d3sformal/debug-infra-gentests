use std::ffi::CStr;

use libc::{__errno_location, S_IRWXU, mode_t};

use crate::log::Log;

pub const PERMS_PERMISSIVE: mode_t = S_IRWXU;

const INVALID_ERRNO: &CStr = c"Errno invalid";

pub fn get_errno_string() -> String {
  // SAFETY: errno location should be thread-local, null retval from libc::strerror is handled
  let errno_str = unsafe {
    let c_errno = libc::strerror(*__errno_location());
    if c_errno.is_null() {
      INVALID_ERRNO
    } else {
      CStr::from_ptr(c_errno)
    }
  };
  let s = match errno_str.to_str() {
    Err(e) => {
      Log::get("get_errno_string")
        .crit(format!("Cannot convert errno string to utf8 string: {}", e));
      return "".to_owned();
    }
    Ok(s) => s.to_owned(),
  };
  s.to_string()
}

/// expects s to be null terminated
pub unsafe fn to_cstr(s: &String) -> &CStr {
  unsafe { CStr::from_bytes_with_nul_unchecked(s.as_bytes()) }
}
