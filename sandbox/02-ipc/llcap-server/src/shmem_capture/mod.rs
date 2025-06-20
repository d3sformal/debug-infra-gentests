pub mod arg_capture;
pub mod call_tracing;
pub mod hooklib_commons;
pub mod mem_utils;
use anyhow::{Result, anyhow, bail, ensure};
use hooklib_commons::{META_MEM_NAME, META_SEM_ACK, META_SEM_DATA, ShmMeta};
use mem_utils::read_w_alignment_chk;
use std::ffi::CStr;

use crate::libc_wrappers::fd::try_shm_unlink_fd;
use crate::libc_wrappers::sem::{FreeFullSemNames, Semaphore};
use crate::libc_wrappers::shared_memory::ShmemHandle;
use crate::libc_wrappers::wrappers::to_cstr;
use crate::log::Log;
use crate::modmap::NumFunUid;
use crate::shmem_capture::mem_utils::ptr_add_nowrap;
use crate::stages::testing::test_server_socket;
use libc::O_CREAT;

/// a handle to all shared memory infrastructure necessary for function tracing (call tracing and argument capture)
pub struct TracingInfra {
  pub sem_free: Semaphore,
  pub sem_full: Semaphore,
  pub backing_buffer: ShmemHandle,
}

impl TracingInfra {
  /// blocks until a buffer has been filled by the instrumented applicaiton
  pub fn wait_for_full_buffer(&mut self) -> Result<()> {
    let sem_res = self.sem_full.try_wait();
    sem_res.map_err(|e| e.context("wait_for_full_buffer"))
  }

  /// signals to the application that "next" buffer is available for modification
  pub fn post_free_buffer(&mut self, dbg_buff_idx: usize) -> Result<()> {
    let sem_res = self.sem_free.try_post();
    sem_res.map_err(|e| e.context(format!("While posting a free buffer (idx {dbg_buff_idx}")))
  }

  pub fn deinit(self) -> Result<()> {
    let (semfree, semfull, buffers_shm) = (self.sem_free, self.sem_full, self.backing_buffer);
    let shm_uninit = deinit_shmem(buffers_shm);
    let sem_uninit = deinit_semaphores(semfree, semfull);

    let goodbye_errors = [shm_uninit, sem_uninit]
      .iter()
      .fold("".to_string(), |acc, v| {
        if let Err(e) = v {
          acc + &e.to_string()
        } else {
          acc
        }
      });
    ensure!(
      goodbye_errors.is_empty(),
      "Deinit failures: {}",
      goodbye_errors
    );
    Ok(())
  }

  fn init_semaphores(prefix: &str, n_buffs: u32) -> Result<(Semaphore, Semaphore)> {
    let FreeFullSemNames {
      free: free_name,
      full: full_name,
    } = FreeFullSemNames::new(prefix, "capture", "base");

    let free_sem = Semaphore::try_open_exclusive(&free_name, n_buffs)?;
    let full_sem = Semaphore::try_open_exclusive(&full_name, 0);

    if let Err(e) = full_sem {
      match deinit_semaphore_single(free_sem) {
        Ok(()) => Err(anyhow!(e)),
        Err(e2) => Err(anyhow!(
          "Failed cleanup after FULL semaphore init failure: {e2}, init failure: {e}"
        )),
      }
    } else {
      Ok((free_sem, full_sem.unwrap()))
    }
  }

  pub fn try_new(resource_prefix: &str, buff_count: u32, buff_len: u32) -> Result<Self> {
    let lg = Log::get("init_tracing");
    let (sem_free, sem_full) = Self::init_semaphores(resource_prefix, buff_count)?;
    lg.info("Initializing shmem");
    lg.warn(format!(
      "Cleanup arguments for finalizer: {} {}",
      sem_full.cname().trim_end_matches('\x00'),
      buff_count
    ));

    let backing_buffer = init_shmem(resource_prefix, buff_count, buff_len)?;

    Ok(Self {
      sem_free,
      sem_full,
      backing_buffer,
    })
  }

  /// returns Ok variant if base pointer + buff_offset are valid
  ///
  /// the contents of the Ok variant is the base pointer, NOT the offseted pointer!
  pub fn get_checked_base_ptr_mut(
    &mut self,
    buff_offset: usize,
  ) -> Result<std::cell::RefMut<'_, *mut u8>> {
    let buffers: &mut ShmemHandle = &mut self.backing_buffer;
    ensure!(
      buff_offset < buffers.len() as usize,
      "Offset too large: {}, compared to the (mut) buffers len {}",
      buff_offset,
      buffers.len()
    );
    let base_mem = buffers.borrow_ptr_mut()?;
    let test_value = ptr_add_nowrap(*base_mem, buff_offset)?;
    ensure!(
      !test_value.is_null() && test_value >= *base_mem,
      "Buffer mut pointer is invalid: {:?}, offset: {}",
      test_value,
      buff_offset
    );
    Ok(base_mem)
  }

  /// returns Ok variant if base pointer + buff_offset are valid
  ///
  /// the contents of the Ok variant is the base pointer, NOT the offseted pointer!
  pub fn get_checked_base_ptr(&self, buff_offset: usize) -> Result<std::cell::Ref<'_, *const u8>> {
    let buffers: &ShmemHandle = &self.backing_buffer;
    ensure!(
      buff_offset < buffers.len() as usize,
      "Offset too large: {}, compared to the buffers len {}",
      buff_offset,
      buffers.len()
    );
    let base_mem = buffers.borrow_ptr()?;
    let test_value = ptr_add_nowrap(*base_mem, buff_offset)?;
    ensure!(
      !test_value.is_null() && test_value >= *base_mem,
      "Buffer const pointer is invalid: {:?}, offset: {}",
      test_value,
      buff_offset
    );
    Ok(base_mem)
  }
}

// I just needd something that does not look like a Result and
// "acts" like Haskell's either
enum Either<T, S> {
  Left(T),
  Right(S),
}

fn cleanup_sems(prefix: &str) {
  let lg = Log::get("cleanup_sems");
  let FreeFullSemNames { free, full } = FreeFullSemNames::new(prefix, "capture", "base");
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
}

pub fn cleanup(prefix: &str) -> Result<()> {
  cleanup_sems(prefix);
  cleanup_shared_mem(prefix)
}

fn deinit_semaphore_single(sem: Semaphore) -> Result<()> {
  match sem.try_close() {
    Ok(sem) => sem,
    Err((_, err)) => bail!(err),
  }
  .try_destroy()
  .map_err(|(_, s)| anyhow!(s))
}

pub fn deinit_semaphores(free_handle: Semaphore, full_handle: Semaphore) -> Result<()> {
  deinit_semaphore_single(free_handle)
    .map_err(|e| e.context("When closing free semaphore"))
    .and_then(|_| deinit_semaphore_single(full_handle))
    .map_err(|e| e.context("When closing full semaphore"))
}

fn get_shmem_name(prefix: &str) -> String {
  format!("{prefix}-capture-base-buffmem\x00")
}

fn init_shmem(prefix: &str, buff_count: u32, buff_len: u32) -> Result<ShmemHandle> {
  let buffs_tmp: String = get_shmem_name(prefix); // keep type annotation for safety
  // SAFETY: line above
  let buffscstr = unsafe { to_cstr(&buffs_tmp) };

  ShmemHandle::try_mmap(buffscstr, buff_count * buff_len)
}

fn cleanup_shared_mem(prefix: &str) -> Result<()> {
  let lg = Log::get("cleanup_shared_mem");
  let metadata_shm_name = String::from_utf8(META_MEM_NAME.to_vec())?;
  let buffs_shm_name: String = get_shmem_name(prefix); // keep type annotation for safety
  // SAFETY: line above
  for name in unsafe { [to_cstr(&metadata_shm_name), to_cstr(&buffs_shm_name)] } {
    lg.info(format!("Cleanup {:?}", name));
    if let Err(e) = try_shm_unlink_fd(name) {
      lg.info(format!("Cleanup error: {:?}: {e}", name));
    }
  }
  let svr_sock_name = test_server_socket(prefix);
  lg.info(format!("Cleanup {:?}", svr_sock_name));
  let _ = std::fs::remove_file(svr_sock_name.clone())
    .inspect_err(|e| lg.info(format!("Cleanup error: {}: {}", svr_sock_name, e)));
  Ok(())
}

fn deinit_shmem(buffers_mem: ShmemHandle) -> Result<()> {
  buffers_mem
    .try_unmap()
    .map_err(|e| e.context("deinit_shmem"))
}

pub fn send_call_tracing_metadata(
  chnl: &mut MetadataPublisher,
  buff_count: u32,
  buff_len: u32,
) -> Result<()> {
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
      target_call_number: 0,
    },
  )
}

pub fn send_arg_capture_metadata(
  chnl: &mut MetadataPublisher,
  buff_count: u32,
  buff_len: u32,
) -> Result<()> {
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
      target_call_number: 0,
    },
  )
}

pub struct TestParams {
  pub arg_count: u32,
  pub test_count: u32,
  pub target_call_number: u32,
}

pub fn send_test_metadata(
  chnl: &mut MetadataPublisher,
  buff_count: u32,
  buff_len: u32,
  fn_uid: NumFunUid,
  params: TestParams,
) -> Result<()> {
  send_metadata(
    chnl,
    ShmMeta {
      buff_count,
      buff_len,
      total_len: buff_count * buff_len,
      mode: 2,
      target_fnid: *fn_uid.function_id,
      target_modid: *fn_uid.module_id,
      forked: 0,
      arg_count: params.arg_count,
      test_count: params.test_count,
      target_call_number: params.target_call_number,
    },
  )
}

fn send_metadata(meta_pub: &mut MetadataPublisher, target_descriptor: ShmMeta) -> Result<()> {
  Log::get("send_metadata").info("Waiting for a cooperating program");
  meta_pub.publish(target_descriptor)
}

/// safety: raw_buff must be the first byte of a buffer which is interpretable as a `*const u32`
/// and dereferencable as u32, other pointer guarantees must also hold
///
/// determines the range of the buffer's data
///  
/// Return value (Ok variant):
/// Left = ending message reached, no further processing needed
/// Right = [start, end) buffer's data bounds
unsafe fn buff_bounds_or_end(raw_buff: *const u8) -> Result<Either<(), (*const u8, *const u8)>> {
  ensure!(!raw_buff.is_null(), "Input is null");
  // SAFETY: read_w_alignment_chk performs *const dereference & null/alignment check, T is u32
  let valid_size: u32 = unsafe { read_w_alignment_chk(raw_buff) }?;

  if valid_size == 0 {
    return Ok(Either::Left(()));
  }

  let start = ptr_add_nowrap(raw_buff, std::mem::size_of_val(&valid_size))?;
  let buff_end = ptr_add_nowrap(start, valid_size as usize)?;
  ensure!(!buff_end.is_null(), "Buffer end is null");
  // validity of buff_end depends on the protocol
  Ok(Either::Right((start, buff_end)))
}

// do not derive clone/copy or define functions with similar semantics
pub struct MetadataPublisher {
  shm: ShmemHandle,
  data_rdy_sem: Semaphore,
  data_ack_sem: Semaphore,
}

impl MetadataPublisher {
  // metadata is published via shared memory and 2 semaphores:
  // a "data_rdy" semaphore that signals that the metadata is ready to be read
  // a "data_ack" semaphore that indicates that the data has been read and can be rewritten/discarded

  pub fn new(mem_path: &CStr, data_sem_path: &str, ack_sem_path: &str) -> Result<Self> {
    // initialize ready semaphore to zero as no data is ready
    let data = Semaphore::try_open_exclusive(data_sem_path, 0)?;
    // !! ack semaphore is initialized to ONE - this will be waited on in the first call of
    // Self::publish
    let ack = Semaphore::try_open_exclusive(ack_sem_path, 1)?;
    let shm = ShmemHandle::try_mmap(mem_path, std::mem::size_of::<ShmemHandle>() as u32)?;
    Ok(Self {
      shm,
      data_rdy_sem: data,
      data_ack_sem: ack,
    })
  }

  pub fn publish(&mut self, meta: ShmMeta) -> Result<()> {
    self.data_ack_sem.try_wait()?;

    {
      let mem = self.shm.borrow_ptr_mut()?;
      unsafe {
        // unaligned write just to be sure
        (*mem as *mut ShmMeta).write_unaligned(meta);
      }
    }

    self.data_rdy_sem.try_post()
  }

  pub fn deinit(self) -> Result<()> {
    self.shm.try_unmap()?;
    self.data_ack_sem.try_destroy().map_err(|e| anyhow!(e.1))?;
    self.data_rdy_sem.try_destroy().map_err(|e| anyhow!(e.1))?;
    Ok(())
  }
}

// SAFETY: we do not give access to shared memory handle
// and semaphores to the outside, furthermore, no suspension points
// are present in associated functions & named
// semaphores should be sharable between threads
unsafe impl Send for MetadataPublisher {}
// note: the type may never become Sync (deinit is not compatible) and
// publish was designed around synchronization of 2 processes, so
// multithreaded contention inside the function was not really considered
// (the type is Arc-Mutexed anyway)
