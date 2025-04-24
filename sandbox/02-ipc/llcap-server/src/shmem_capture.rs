use std::ffi::CStr;

use libc::{
  __errno_location, MAP_FAILED, MAP_SHARED_VALIDATE, O_CREAT, O_EXCL, O_RDWR, PROT_READ,
  PROT_WRITE, S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR, SEM_FAILED, c_void, ftruncate,
  mmap, mode_t, sem_open, sem_t, shm_open, shm_unlink,
};

#[repr(C)]
pub struct MetaDescriptor {
  pub buff_count: u32,
  pub buff_size: u32,
}

pub struct ShmSem {
  sem: *mut sem_t,
}

fn get_errno_string() -> String {
  let errno_str: &CStr = unsafe { CStr::from_ptr(libc::strerror(*__errno_location())) };
  let s = match errno_str.to_str() {
    Err(e) => {
      println!("Cannot convert errno string to utf8 string: {}", e);
      return "".to_owned();
    }
    Ok(s) => s.to_owned(),
  };
  s.to_string()
}

const PERMS_PERMISSIVE: mode_t = S_IROTH | S_IWOTH | S_IWGRP | S_IRGRP | S_IWUSR | S_IRUSR;

unsafe fn to_cstr(s: &String) -> &CStr {
  unsafe { CStr::from_bytes_with_nul_unchecked(s.as_bytes()) }
}

pub fn init_semaphores(prefix: &str, n_buffs: u32) -> Result<(ShmSem, ShmSem), String> {
  let free_cname = format!("{prefix}semfree\x00");
  let full_cname = format!("{prefix}semfull\x00");
  let free_sem_name = unsafe { to_cstr(&free_cname) };
  let full_sem_name = unsafe { to_cstr(&full_cname) };

  let free_sem = unsafe {
    sem_open(
      free_sem_name.as_ptr(),
      O_CREAT | O_EXCL,
      PERMS_PERMISSIVE,
      n_buffs,
    )
  };
  if free_sem == SEM_FAILED {
    let s = get_errno_string();
    return Err(format!("Failed to initialize FREE semaphore: {}", s));
  }
  let full_sem = unsafe {
    sem_open(
      full_sem_name.as_ptr(),
      O_CREAT | O_EXCL,
      PERMS_PERMISSIVE,
      0,
    )
  };
  if full_sem == SEM_FAILED {
    return Err(format!(
      "Failed to initialize FULL semaphore: {}",
      get_errno_string()
    ));
  }
  Ok((ShmSem { sem: free_sem }, ShmSem { sem: full_sem }))
}

pub struct ShmHandle {
  mem: *mut c_void,
  len: u32,
  fd: i32,
  name: String,
}

fn try_shm_unlink_fd(name: &CStr) -> Result<(), String> {
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

fn try_mmap_with_name(path: &CStr, len: u32) -> Result<ShmHandle, String> {
  let unlinking_handler = |error_string: String| {
    let unlink_res =
      try_shm_unlink_fd(path).map_err(|e| format!("{error_string}\n\tWith inner error: {e}"));

    match unlink_res {
      Ok(_) => Err(error_string),
      Err(s) => Err(s),
    }
  };

  let fd = unsafe { shm_open(path.as_ptr(), O_CREAT | O_EXCL | O_RDWR, PERMS_PERMISSIVE) };
  if fd == -1 {
    return Err(format!(
      "Failed to open FD for shmem {}: {}",
      path.to_string_lossy(),
      get_errno_string()
    ));
  }

  if unsafe { ftruncate(fd, len as i64) } == -1 {
    return unlinking_handler(format!(
      "Failed to truncate FD for shmem {}, len: {}: {}",
      path.to_string_lossy(),
      len,
      get_errno_string()
    ));
  }

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
  if mmap_res == MAP_FAILED {
    return unlinking_handler(format!(
      "Failed to mmap {}, len: {}: {}",
      path.to_string_lossy(),
      len,
      get_errno_string()
    ));
  }

  Ok(ShmHandle {
    fd,
    len,
    mem: mmap_res,
    name: path.to_string_lossy().to_string(),
  })
}

pub fn init_shmem(
  prefix: &str,
  buff_count: u32,
  buff_len: u32,
) -> Result<(ShmHandle, ShmHandle), String> {
  let meta_tmp = format!("{prefix}shmmeta\x00");
  let metacstr = unsafe { to_cstr(&meta_tmp) };
  let meta_mem_handle = try_mmap_with_name(metacstr, std::mem::size_of::<MetaDescriptor>() as u32)?;

  let buffs_tmp = format!("{prefix}shmbuffs\x00");
  let buffscstr = unsafe { to_cstr(&buffs_tmp) };

  let buffers_mem_handle = try_mmap_with_name(buffscstr, buff_count * buff_len).unwrap();

  Ok((meta_mem_handle, buffers_mem_handle))
}
