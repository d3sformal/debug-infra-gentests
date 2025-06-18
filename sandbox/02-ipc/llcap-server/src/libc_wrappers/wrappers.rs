use std::ffi::CStr;

use libc::{S_IRWXU, mode_t};

pub const PERMS_PERMISSIVE: mode_t = S_IRWXU;

/// expects s to be null terminated
pub unsafe fn to_cstr(s: &String) -> &CStr {
  unsafe { CStr::from_bytes_with_nul_unchecked(s.as_bytes()) }
}
