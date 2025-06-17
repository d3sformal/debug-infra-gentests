use anyhow::{Result, ensure};

/// SAFETY: "reasonable" T (intended for primitive types)
/// performs a *const dereference
pub unsafe fn read_w_alignment_chk<T: Copy>(ptr: *const u8) -> Result<T> {
  aligned_to::<T>(ptr)?;
  ensure!(!ptr.is_null(), "Cannot read null ptr");
  // SAFETY: line above + Copy does not create aliased memory
  Ok(unsafe { *(ptr as *const T) })
}

/// returns the Err variant if supplied pointer is not aligned to the type T
fn aligned_to<T>(ptr: *const u8) -> Result<()> {
  ensure!(
    (ptr as *const T).is_aligned(),
    "Pointer {:?} unaligned to type of size {}",
    ptr,
    std::mem::size_of::<T>()
  );
  Ok(())
}

/// returns the Err variant if a read attempted with these parameters would be out of bounds
pub fn overread_check(
  raw_buff: *const u8,
  buff_end: *const u8,
  sz: usize,
  msg: &str,
) -> Result<()> {
  ensure!(
    ptr_add_nowrap(raw_buff, sz)? <= buff_end,
    "Would over-read {msg}... ptr: {:?} offset: {} end: {:?}",
    raw_buff,
    sz,
    buff_end
  );
  Ok(())
}

/// adds an offset to pointer, prohibiting overflow
pub fn ptr_add_nowrap(ptr: *const u8, sz: usize) -> Result<*const u8> {
  let mb_wrapped = ptr.wrapping_add(sz);
  ensure!(mb_wrapped >= ptr, "Wraparound for ptr {:?} len {}", ptr, sz);
  Ok(mb_wrapped)
}

/// adds an offset to pointer, prohibiting overflow
pub fn ptr_add_nowrap_mut(ptr: *mut u8, sz: usize) -> Result<*mut u8> {
  let mb_wrapped: *mut u8 = ptr.wrapping_add(sz);
  ensure!(
    mb_wrapped >= ptr,
    "Wraparound for mut ptr {:?} len {}",
    ptr,
    sz
  );
  Ok(mb_wrapped)
}

// bunch of sanity-check tests, nothing complex to see here
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn nowrap_ok_case() {
    let data: u32 = 12312;
    let ptr: *const u32 = std::ptr::from_ref(&data);
    assert!(ptr_add_nowrap(ptr as *const u8, 1).is_ok())
  }

  #[test]
  pub fn nowrap_simple() {
    let ptr: *const u8 = 0xffffffffffffffff as *const u8;
    assert!(ptr_add_nowrap(ptr, 1).is_err())
  }

  #[test]
  pub fn nowrap_larger() {
    let sz: usize = 4;
    let ptr: *const u8 = (0xffffffffffffffff - sz) as *const u8;
    assert!(ptr_add_nowrap(ptr, sz + 1).is_err())
  }

  #[test]
  pub fn aligned_to_ok_case() {
    let data: u32 = 12312;
    let ptr: *const u32 = std::ptr::from_ref(&data);
    assert!(aligned_to::<u32>(ptr as *const u8).is_ok())
  }

  #[test]
  pub fn aligned_to_simple() {
    let ptr: *const u8 = (0xffffffffffffffff - std::mem::size_of::<u32>() - 1) as *const u8;
    assert!(aligned_to::<u32>(ptr).is_err())
  }
}
