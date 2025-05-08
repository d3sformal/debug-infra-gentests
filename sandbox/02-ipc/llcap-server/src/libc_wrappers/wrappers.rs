use std::ffi::CStr;

use libc::{__errno_location, S_IRWXG, S_IRWXO, S_IRWXU, mode_t};

use crate::log::Log;

// todo: change to user only
pub const PERMS_PERMISSIVE: mode_t = S_IRWXO | S_IRWXG | S_IRWXU;

pub fn get_errno_string() -> String {
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

/// expects s to be null terminated
pub unsafe fn to_cstr(s: &String) -> &CStr {
  unsafe { CStr::from_bytes_with_nul_unchecked(s.as_bytes()) }
}
