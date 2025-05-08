pub mod call_tracing;
pub mod mem_utils;

use mem_utils::aligned_to;

use crate::libc_wrappers::fd::try_shm_unlink_fd;
use crate::libc_wrappers::sem::Semaphore;
use crate::libc_wrappers::shared_memory::ShmemHandle;
use crate::libc_wrappers::wrappers::to_cstr;
use crate::log::Log;
use libc::O_CREAT;

#[repr(C)]
struct MetaDescriptor {
  pub buff_count: u32,
  pub buff_size: u32,
  pub total_len: u32,
}

/// a handle to all shared memory infrastructure necessary for function tracing
pub struct TracingInfra {
  pub sem_free: Semaphore,
  pub sem_full: Semaphore,
  pub meta_buffer: ShmemHandle,
  pub backing_buffer: ShmemHandle,
}

pub fn cleanup_shmem(prefix: &str) -> Result<(), String> {
  let lg = Log::get("cleanup");
  let free_name = format!("{prefix}semfree");
  let full_name = format!("{prefix}semfull");

  for name in &[free_name, full_name] {
    lg.info(format!("Cleanup {}", name));
    let res = Semaphore::try_open(name, 0, O_CREAT.into(), None);
    if let Ok(sem) = res {
      let _ = deinit_semaphore_single(sem)
        .inspect_err(|e| lg.info(format!("Cleanup of opened {name}: {e}")));
    } else {
      lg.info(format!("Cleanup {}: {}", name, res.err().unwrap()));
    }
  }

  let meta_tmp = format!("{prefix}shmmeta\x00");
  let buffs_tmp = format!("{prefix}shmbuffs\x00");
  // SAFETY: line above
  for name in [unsafe { to_cstr(&meta_tmp) }, unsafe {
    to_cstr(&buffs_tmp)
  }] {
    lg.info(format!("Cleanup {:?}", name));
    if let Err(e) = try_shm_unlink_fd(name) {
      lg.info(format!("Cleanup error: {:?}: {e}", name));
    }
  }

  Ok(())
}

fn init_semaphores(prefix: &str, n_buffs: u32) -> Result<(Semaphore, Semaphore), String> {
  let free_name = format!("{prefix}semfree");
  let full_name = format!("{prefix}semfull");

  let free_sem = Semaphore::try_open_exclusive(&free_name, n_buffs)?;
  let full_sem = Semaphore::try_open_exclusive(&full_name, 0);

  if let Err(e) = full_sem {
    match deinit_semaphore_single(free_sem) {
      Ok(()) => Err(e),
      Err(e2) => Err(format!(
        "Failed cleanup after FULL semaphore init failure: {e2}, init failure: {e}"
      )),
    }
  } else {
    Ok((free_sem, full_sem.unwrap()))
  }
}

fn deinit_semaphore_single(sem: Semaphore) -> Result<(), String> {
  match sem.try_close() {
    Ok(sem) => sem,
    Err((_, err)) => return Err(err),
  }
  .try_destroy()
  .map_err(|(_, s)| s)
}

pub fn deinit_semaphores(free_handle: Semaphore, full_handle: Semaphore) -> Result<(), String> {
  deinit_semaphore_single(free_handle)
    .map_err(|e| format!("When closing free semaphore: {e}"))
    .and_then(|_| deinit_semaphore_single(full_handle))
    .map_err(|e| format!("When closing full semaphore: {e}"))
}

pub fn init_shmem(
  prefix: &str,
  buff_count: u32,
  buff_len: u32,
) -> Result<(ShmemHandle, ShmemHandle), String> {
  let meta_tmp = format!("{prefix}shmmeta\x00");
  // SAFETY: line above
  let metacstr = unsafe { to_cstr(&meta_tmp) };
  let meta_mem_handle =
    ShmemHandle::try_mmap(metacstr, std::mem::size_of::<MetaDescriptor>() as u32)?;

  {
    let target_descriptor = MetaDescriptor {
      buff_count,
      buff_size: buff_len,
      total_len: buff_count * buff_len,
    };
    aligned_to::<MetaDescriptor>(meta_mem_handle.mem as *const u8)?;
    // SAFETY: line above
    unsafe {
      *(meta_mem_handle.mem as *mut MetaDescriptor) = target_descriptor;
    }
  }

  let buffs_tmp = format!("{prefix}shmbuffs\x00");
  // SAFETY: line above
  let buffscstr = unsafe { to_cstr(&buffs_tmp) };

  let result = ShmemHandle::try_mmap(buffscstr, buff_count * buff_len);

  if let Ok(res) = result {
    Ok((meta_mem_handle, res))
  } else {
    match meta_mem_handle.try_unmap() {
      Err(e) => Err(format!(
        "Failed to map shared memory, initialization failed: {e}"
      )),
      Ok(()) => Err(format!(
        "Initialization failed and was successfully undone, original error: {}",
        result.err().unwrap()
      )),
    }
  }
}

fn deinit_shmem(meta_mem: ShmemHandle, buffers_mem: ShmemHandle) -> Result<(), String> {
  let resmeta = meta_mem
    .try_unmap()
    .map_err(|e| format!("When unmapping meta mem: {e}"));
  let resbuffs = buffers_mem
    .try_unmap()
    .map_err(|e| format!("When unmapping buffers mem: {e}"));

  if resmeta.is_err() || resbuffs.is_err() {
    let err_str = |res: Result<(), String>| {
      if let Err(e) = res { e } else { "".to_owned() }
    };
    let meta_err = err_str(resmeta);
    let buffs_err = err_str(resbuffs);

    Err("Shmem deinit errors: ".to_string() + &meta_err + " " + &buffs_err)
  } else {
    Ok(())
  }
}

pub fn init_tracing(
  resource_prefix: &str,
  buffer_count: u32,
  buffer_size: u32,
) -> Result<TracingInfra, String> {
  let lg = Log::get("init_tracing");
  let (sem_free, sem_full) = init_semaphores(resource_prefix, buffer_count)?;
  lg.info("Initializing shmem");
  let (meta_buffer, backing_buffer) = match init_shmem(resource_prefix, buffer_count, buffer_size) {
    Err(e) => {
      match deinit_semaphores(sem_free, sem_full) {
        // both arms MUST return Err! or the unreachable! macro below panics!
        Ok(()) => Err(e),
        Err(e2) => Err(format!(
          "Failed to clean up semaphores when mmap failed: {e2}, map failure: {e}"
        )),
      }?; // <-- unreachable does not panic if both arms return an Err

      unreachable!(
        "The above uses ? operator on an ensured Err variant (returned in both match arms)"
      );
    }
    Ok(a) => Ok::<(ShmemHandle, ShmemHandle), String>(a),
  }?;

  Ok(TracingInfra {
    sem_free,
    sem_full,
    meta_buffer,
    backing_buffer,
  })
}

pub fn deinit_tracing(infra: TracingInfra) -> Result<(), String> {
  let (semfree, semfull, meta_shm, buffers_shm) = (
    infra.sem_free,
    infra.sem_full,
    infra.meta_buffer,
    infra.backing_buffer,
  );
  let shm_uninit = deinit_shmem(meta_shm, buffers_shm);
  let sem_uninit = deinit_semaphores(semfree, semfull);

  let goodbye_errors = [shm_uninit, sem_uninit]
    .iter()
    .fold("".to_string(), |acc, v| {
      if v.is_err() {
        acc + v.as_ref().unwrap_err()
      } else {
        acc
      }
    });

  if !goodbye_errors.is_empty() {
    Err(format!("Failed deinit! {goodbye_errors}"))
  } else {
    Ok(())
  }
}
