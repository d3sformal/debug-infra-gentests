pub mod arg_capture;
pub mod call_tracing;
pub mod hooklib_commons;
pub mod mem_utils;
use std::ffi::CStr;

use hooklib_commons::{META_MEM_NAME, META_SEM_ACK, META_SEM_DATA, ShmMeta};
use mem_utils::read_w_alignment_chk;

use crate::libc_wrappers::fd::try_shm_unlink_fd;
use crate::libc_wrappers::sem::{FreeFullSemNames, Semaphore};
use crate::libc_wrappers::shared_memory::ShmemHandle;
use crate::libc_wrappers::wrappers::to_cstr;
use crate::log::Log;
use crate::modmap::{IntegralFnId, IntegralModId};
use libc::O_CREAT;
/// a handle to all shared memory infrastructure necessary for function tracing
pub struct TracingInfra<'a> {
  pub sem_free: Semaphore,
  pub sem_full: Semaphore,
  pub backing_buffer: ShmemHandle<'a>,
}

enum Either<T, S> {
  Left(T),
  Right(S),
}

pub fn cleanup(prefix: &str) -> Result<(), String> {
  let lg = Log::get("cleanup");
  let FreeFullSemNames { free, full } = FreeFullSemNames::get(prefix, "capture", "base");
  for name in &[
    free,
    full,
    String::from_utf8(META_SEM_DATA.split_last().unwrap().1.to_vec()).unwrap(),
    String::from_utf8(META_SEM_ACK.split_last().unwrap().1.to_vec()).unwrap(),
  ] {
    lg.info(format!("Cleanup {}", name));
    let res = Semaphore::try_open(name, 0, O_CREAT.into(), None);
    if let Ok(sem) = res {
      let _ = deinit_semaphore_single(sem)
        .inspect_err(|e| lg.info(format!("Cleanup of opened {name}: {e}")));
    } else {
      lg.info(format!("Cleanup {}: {}", name, res.err().unwrap()));
    }
  }

  let metadata_shm_name = String::from_utf8(META_MEM_NAME.to_vec()).map_err(|e| e.to_string())?;
  let buffs_shm_name = format!("{prefix}-capture-base-buffmem\x00");
  // SAFETY: line above
  for name in [unsafe { to_cstr(&metadata_shm_name) }, unsafe {
    to_cstr(&buffs_shm_name)
  }] {
    lg.info(format!("Cleanup {:?}", name));
    if let Err(e) = try_shm_unlink_fd(name) {
      lg.info(format!("Cleanup error: {:?}: {e}", name));
    }
  }
  Ok(())
}

fn init_semaphores(prefix: &str, n_buffs: u32) -> Result<(Semaphore, Semaphore), String> {
  let FreeFullSemNames {
    free: free_name,
    full: full_name,
  } = FreeFullSemNames::get(prefix, "capture", "base");

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

fn init_shmem(prefix: &str, buff_count: u32, buff_len: u32) -> Result<ShmemHandle, String> {
  let buffs_tmp = format!("{prefix}-capture-base-buffmem\x00");
  // SAFETY: line above
  let buffscstr = unsafe { to_cstr(&buffs_tmp) };

  ShmemHandle::try_mmap(buffscstr, buff_count * buff_len)
}

fn deinit_shmem(buffers_mem: ShmemHandle) -> Result<(), String> {
  buffers_mem
    .try_unmap()
    .map_err(|e| format!("When unmapping buffers mem: {e}"))
}

pub async fn init_tracing(
  resource_prefix: &str,
  buff_count: u32,
  buff_len: u32,
) -> Result<TracingInfra, String> {
  let lg = Log::get("init_tracing");
  let (sem_free, sem_full) = init_semaphores(resource_prefix, buff_count)?;
  lg.info("Initializing shmem");

  let backing_buffer = init_shmem(resource_prefix, buff_count, buff_len)?;

  Ok(TracingInfra {
    sem_free,
    sem_full,
    backing_buffer,
  })
}

pub fn send_call_tracing_metadata(
  chnl: &mut MetadataPublisher<'_>,
  buff_count: u32,
  buff_len: u32,
) -> Result<(), String> {
  send_metadata(
    chnl,
    ShmMeta {
      buff_count,
      buff_len,
      total_len: buff_count * buff_len,
      mode: 0,
      target_fnid: 0,
      target_modid: 0,
      forked: 0,
      arg_count: 0,
      test_count: 0,
    },
  )
}

pub fn send_arg_capture_metadata(
  chnl: &mut MetadataPublisher<'_>,
  buff_count: u32,
  buff_len: u32,
) -> Result<(), String> {
  send_metadata(
    chnl,
    ShmMeta {
      buff_count,
      buff_len,
      total_len: buff_count * buff_len,
      mode: 1,
      target_fnid: 0,
      target_modid: 0,
      forked: 0,
      arg_count: 0,
      test_count: 0,
    },
  )
}

pub fn send_test_metadata(
  chnl: &mut MetadataPublisher<'_>,
  buff_count: u32,
  buff_len: u32,
  module: IntegralModId,
  fn_id: IntegralFnId,
  arg_count: u32,
  test_count: u32,
) -> Result<(), String> {
  send_metadata(
    chnl,
    ShmMeta {
      buff_count,
      buff_len,
      total_len: buff_count * buff_len,
      mode: 2,
      target_fnid: *fn_id,
      target_modid: *module,
      forked: 0,
      arg_count,
      test_count,
    },
  )
}

fn send_metadata(
  chnl: &mut MetadataPublisher<'_>,
  target_descriptor: ShmMeta,
) -> Result<(), String> {
  Log::get("init_tracing").info("Waiting for a cooperating program");
  chnl.publish(target_descriptor)
}

pub fn deinit_tracing(infra: TracingInfra) -> Result<(), String> {
  let (semfree, semfull, buffers_shm) = (infra.sem_free, infra.sem_full, infra.backing_buffer);
  let shm_uninit = deinit_shmem(buffers_shm);
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

fn wait_for_free_buffer(infra: &mut TracingInfra) -> Result<(), String> {
  let sem_res = infra.sem_full.try_wait();
  if let Err(e) = sem_res {
    Err(format!("While waiting for buffer: {}", e))
  } else {
    Ok(())
  }
}

fn post_free_buffer(infra: &mut TracingInfra, dbg_buff_idx: usize) -> Result<(), String> {
  let sem_res = infra.sem_free.try_post();
  if let Err(e) = sem_res {
    return Err(format!(
      "While posting a free buffer (idx {dbg_buff_idx}): {}",
      e
    ));
  }
  Ok(())
}

// get the start of a buffer at an offset
fn get_buffer_start(infra: &TracingInfra, buff_offset: usize) -> Result<*mut u8, String> {
  let buffers = &infra.backing_buffer;
  if buff_offset >= buffers.len() as usize {
    return Err(format!(
      "Calculated offset too large: {}, compared to the buffers len {}",
      buff_offset,
      buffers.len()
    ));
  }
  let mem = buffers.ptr();
  let buff_ptr = mem.wrapping_byte_add(buff_offset) as *mut u8;

  if buff_ptr.is_null() || buff_ptr < mem as *mut u8 {
    return Err(format!(
      "Buffer pointer is invalid: {:?}, mem: {:?}",
      buff_ptr, buffers
    ));
  }
  Ok(buff_ptr)
}

// raw_buff arg: poitner validity must be ensured by protocol, target type is copied, no allocation over the same memory region
// Okay Left = ending (empty) message reached, no further processing needed
// Okay Right = [start, end) buffer's data bounds
fn buff_bounds_or_end(raw_buff: *const u8) -> Result<Either<(), (*const u8, *const u8)>, String> {
  // SAFETY: read_w_alignment_chk performs *const dereference & null/alignment check
  let valid_size: u32 = unsafe { read_w_alignment_chk(raw_buff) }?;

  if valid_size == 0 {
    return Ok(Either::Left(()));
  }

  let start = raw_buff.wrapping_byte_add(std::mem::size_of_val(&valid_size));
  let buff_end = start.wrapping_byte_add(valid_size as usize);
  Ok(Either::Right((start, buff_end)))
}

pub struct MetadataPublisher<'a> {
  shm: ShmemHandle<'a>,
  data_rdy_sem: Semaphore,
  data_ack_sem: Semaphore,
}

impl<'a> MetadataPublisher<'a> {
  pub fn new(mem_path: &CStr, data_sem_path: &str, ack_sem_path: &str) -> Result<Self, String> {
    let data = Semaphore::try_open_exclusive(data_sem_path, 0)?;
    let ack = Semaphore::try_open_exclusive(ack_sem_path, 1)?;
    let shm = ShmemHandle::try_mmap(mem_path, std::mem::size_of::<ShmemHandle>() as u32)?;
    Ok(Self {
      shm,
      data_rdy_sem: data,
      data_ack_sem: ack,
    })
  }

  pub fn publish(&mut self, meta: ShmMeta) -> Result<(), String> {
    self.data_ack_sem.try_wait()?;

    let mem = self.shm.ptr();
    unsafe {
      (mem as *mut ShmMeta).write(meta);
    };

    self.data_rdy_sem.try_post()
  }
}

// safe because we do not give access to shared memory handle
// and semaphores to the outside, furthermore, no suspension points
// are present in associated functions & named
// semaphores should be sharable between threads
unsafe impl Send for MetadataPublisher<'_> {}
