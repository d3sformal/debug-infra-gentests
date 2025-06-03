use anyhow::{Result, ensure};

/// SAFETY: "reasonable" T (intended for primitive types)
/// performs a *const dereference
pub unsafe fn read_w_alignment_chk<T: Copy>(ptr: *const u8) -> Result<T> {
  aligned_to::<T>(ptr)?;
  ensure!(!ptr.is_null(), "Cannot read null ptr");
  // SAFETY: line above + Copy does not create aliased memory
  Ok(unsafe { *(ptr as *const T) })
}

pub fn aligned_to<T>(ptr: *const u8) -> Result<()> {
  ensure!(
    (ptr as *const T).is_aligned(),
    "Pointer {:?} unaligned to type of size {}",
    ptr,
    std::mem::size_of::<T>()
  );
  Ok(())
}

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

pub fn ptr_add_nowrap(ptr: *const u8, sz: usize) -> Result<*const u8> {
  let mb_wrapped = ptr.wrapping_add(sz);
  ensure!(mb_wrapped >= ptr, "Wraparound for ptr {:?} len {}", ptr, sz);
  Ok(mb_wrapped)
}

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
