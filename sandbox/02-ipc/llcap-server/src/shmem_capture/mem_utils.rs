/// SAFETY: "reasonable" T (intended for primitive types)
/// performs a *const dereference
pub unsafe fn read_w_alignment_chk<T: Copy>(ptr: *const u8) -> Result<T, String> {
  aligned_to::<T>(ptr)?;
  if ptr.is_null() {
    return Err("Null ptr encountered".to_owned());
  }
  // SAFETY: line above + Copy does not create aliased memory
  Ok(unsafe { *(ptr as *const T) })
}

pub fn aligned_to<T>(ptr: *const u8) -> Result<(), String> {
  if !(ptr as *const T).is_aligned() {
    Err(format!(
      "Pointer {:?} unaligned to type of size {}",
      ptr,
      std::mem::size_of::<T>()
    ))
  } else {
    Ok(())
  }
}

pub fn overread_check(
  raw_buff: *const u8,
  buff_end: *const u8,
  sz: usize,
  msg: &str,
) -> Result<(), String> {
  if ptr_add_nowrap(raw_buff, sz)? > buff_end {
    Err(format!(
      "Would over-read {msg}... ptr: {:?} offset: {} end: {:?}",
      raw_buff, sz, buff_end
    ))
  } else {
    Ok(())
  }
}

pub fn ptr_add_nowrap(ptr: *const u8, sz: usize) -> Result<*const u8, String> {
  let mb_wrapped = ptr.wrapping_add(sz);
  if mb_wrapped < ptr {
    Err(format!("Wraparound for ptr {:?} len {}", ptr, sz))
  } else {
    Ok(mb_wrapped)
  }
}

pub fn ptr_add_nowrap_mut(ptr: *mut u8, sz: usize) -> Result<*mut u8, String> {
  let mb_wrapped: *mut u8 = ptr.wrapping_add(sz);
  if mb_wrapped < ptr {
    Err(format!("Wraparound for mut ptr {:?} len {}", ptr, sz))
  } else {
    Ok(mb_wrapped)
  }
}
