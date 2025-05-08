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
