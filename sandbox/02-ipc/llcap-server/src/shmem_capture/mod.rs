pub mod arg_capture;
pub mod call_tracing;
pub mod hooklib_commons;
pub mod mem_utils;
use anyhow::{Result, anyhow, bail, ensure};
use hooklib_commons::{META_MEM_NAME, META_SEM_ACK, META_SEM_DATA, ShmMeta};
use std::ffi::CStr;
use std::ops::ControlFlow;
use std::slice;

use crate::libc_wrappers::fd::try_shm_unlink_fd;
use crate::libc_wrappers::sem::{FreeFullSemNames, Semaphore};
use crate::libc_wrappers::shared_memory::ShmemHandle;
use crate::libc_wrappers::wrappers::to_cstr;
use crate::log::Log;
use crate::modmap::{ExtModuleMap, NumFunUid};
use crate::shmem_capture::mem_utils::ptr_add_nowrap;
use crate::stages::testing::test_server_socket;
use libc::O_CREAT;

/// a handle to all shared memory infrastructure necessary for function tracing (call tracing and argument capture)
pub struct TracingInfra {
  pub sem_free: Semaphore,
  pub sem_full: Semaphore,
  pub backing_buffer: ShmemHandle,
  logical_buffer_size: usize,
  logical_buffer_count: usize,
  current_index: usize,
  got_buff_flag: bool,
}

/// a uniquely-typed wrapper around a start pointer to a logical buffer
/// which points to a valid size field
pub struct BufferStartPtr<'a> {
  ptr: ReadOnlyBufferPtr<'a>,
}

impl<'a> BufferStartPtr<'a> {
  pub fn new(slice: &'a [u8]) -> Result<Self> {
    ensure!(
      slice.len() > std::mem::size_of::<u32>(),
      "No space for length field"
    );
    Ok(Self {
      ptr: ReadOnlyBufferPtr::new(slice),
    })
  }

  /// read the size field and return a buffer that supports reading of data
  pub fn shift_init_data(mut self) -> Result<ReadOnlyBufferPtr<'a>> {
    let valid_size: u32 = self.ptr.unaligned_shift_num_read()?;

    Ok(self.ptr.constrain(valid_size as usize))
  }
}

/// a uniquely-typed wrapper around a logical buffer's inner data
pub struct ReadOnlyBufferPtr<'a> {
  slice: &'a [u8],
}

impl<'a> ReadOnlyBufferPtr<'a> {
  fn len(&self) -> usize {
    self.slice.len()
  }

  pub fn empty(&self) -> bool {
    self.len() == 0
  }

  pub fn constrain(self, max_size: usize) -> Self {
    Self::new(self.slice.split_at(max_size).0)
  }

  fn new(slice: &'a [u8]) -> Self {
    Self { slice }
  }

  pub fn shift(&mut self, offset: usize) {
    self.slice = self.slice.split_at(offset).1;
  }

  // splits the underlying slice at offset, returning the first part
  // if successful (and shifting the underlying slice to the second part)
  fn shift_split(&mut self, size: usize) -> Result<&[u8]> {
    let (res, rest) = self.slice.split_at(size);
    ensure!(res.len() == size, "Buff does not contain enough bytes");
    self.slice = rest;
    Ok(res)
  }

  // performs an unaligned read of bytes into a numerical value
  pub fn unaligned_shift_num_read<S: num_traits::FromBytes>(&mut self) -> Result<S>
  where
    for<'x> &'x <S as num_traits::FromBytes>::Bytes: TryFrom<&'x [u8]>,
    for<'y> <&'y <S as num_traits::FromBytes>::Bytes as TryFrom<&'y [u8]>>::Error:
      Into<anyhow::Error>,
  {
    Ok(S::from_le_bytes(
      self
        .shift_split(std::mem::size_of::<S>())?
        .try_into()
        .map_err(|e: <&<S as num_traits::FromBytes>::Bytes as TryFrom<&[u8]>>::Error| anyhow!(e))?,
    ))
  }

  // caller must ensure that the slice's pointer is never exposed or (worse) used as ptr to
  // mutable
  pub fn as_slice(&self) -> &[u8] {
    self.slice
  }
}

pub struct BorrowedReadBuffer<'a> {
  _borrow_handle: std::cell::Ref<'a, *const u8>,
  pub buffer: ReadOnlyBufferPtr<'a>,
}

impl TracingInfra {
  /// blocks until a buffer has been filled by the instrumented applicaiton
  pub fn wait_for_full_buffer(&mut self) -> Result<BorrowedReadBuffer<'_>> {
    let sem_res = self.sem_full.try_wait();
    sem_res.map_err(|e| e.context("wait_for_full_buffer"))?;
    self.got_buff_flag = true;
    self.get_checked_base_ptr()
  }

  pub fn finish_buffer(&mut self) -> Result<()> {
    {
      let buffer = self.get_checked_base_ptr_mut()?;

      // SAFETY: RefMut succeeded, construction
      unsafe {
        *(*buffer as *mut u32) = 0;
      };
    }
    let sem_res = self.sem_free.try_post();
    sem_res.map_err(|e| {
      e.context(format!(
        "While posting a free buffer (idx {}",
        self.current_index
      ))
    })?;
    self.got_buff_flag = false;
    self.current_index += 1;
    self.current_index %= self.logical_buffer_count;
    Ok(())
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
      logical_buffer_size: buff_len as usize,
      logical_buffer_count: buff_count as usize,
      current_index: 0,
      got_buff_flag: false,
    })
  }

  fn buffer_offset(&self, idx: usize) -> Result<usize> {
    ensure!(
      idx < self.logical_buffer_count,
      "Invalid buffer index {}",
      idx
    );
    Ok(idx * self.logical_buffer_size)
  }

  fn buffer_count(&self) -> usize {
    self.logical_buffer_count
  }

  /// returns a buffer pointing tho the base of a logical buffer (incl. length field)
  fn get_checked_base_ptr_mut(&mut self) -> Result<std::cell::RefMut<'_, *mut u8>> {
    let buff_offset = self.buffer_offset(self.current_index)?;
    ensure!(
      buff_offset < self.backing_buffer.len() as usize,
      "Offset too large: {}, compared to the (mut) buffers len {}",
      buff_offset,
      self.backing_buffer.len()
    );
    let base_mem = self.backing_buffer.borrow_ptr_mut()?;
    let test_value = ptr_add_nowrap(*base_mem, buff_offset)?;
    ensure!(
      !test_value.is_null() && test_value >= *base_mem,
      "Buffer mut pointer is invalid: {:?}, offset: {}",
      test_value,
      buff_offset
    );
    Ok(base_mem)
  }

  /// returns Ok variant if base pointer + buff_offset are valid offset to a logical buffer
  fn get_checked_base_ptr(&self) -> Result<BorrowedReadBuffer<'_>> {
    let buffers: &ShmemHandle = &self.backing_buffer;
    let buff_offset = self.buffer_offset(self.current_index)?;
    ensure!(
      buff_offset < buffers.len() as usize,
      "Offset too large: {}, compared to the buffers len {}",
      buff_offset,
      buffers.len()
    );
    let base_mem = buffers.borrow_ptr()?;
    {
      // arithmetic operations on pointers that check we can do what follows after this block
      // the pointers here carry no/incorrect provenance and are thus unreferencable
      // even by the slice::from_raw_parts

      let test_value = ptr_add_nowrap(*base_mem, buff_offset)?;
      ensure!(
        !test_value.is_null() && test_value >= *base_mem,
        "Buffer const pointer is invalid: {:?}, offset: {}",
        test_value,
        buff_offset
      );
      let test_value_end = ptr_add_nowrap(test_value, self.logical_buffer_size)?;
      ensure!(
        !test_value_end.is_null()
          && (test_value_end as usize) - (test_value as usize)
            <= self.backing_buffer.len() as usize,
        "Buffer const pointer is invalid: {:?}, {:?}, offset: {}, backing size: {}",
        test_value_end,
        test_value,
        buff_offset,
        self.backing_buffer.len()
      );
    }
    // base_mem should carry no provenance as it originates from mmpaed memory

    // by the following, I hope to introduce provenance to the pointer (slices) that follow

    // this slice only says we can fit all previous buffer and the target buffer
    // SAFETY: via refcell borrow checking and checks done above
    let up_to_target_buff: &[u8] =
      unsafe { slice::from_raw_parts(*base_mem, buff_offset + self.logical_buffer_size) };
    let target_buffer = up_to_target_buff.split_at(buff_offset).1;

    let buffer = BufferStartPtr::new(target_buffer)?.shift_init_data()?;

    Ok(BorrowedReadBuffer {
      _borrow_handle: base_mem,
      buffer,
    })
  }
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

trait CaptureLoop {
  type State: Default;

  fn update_from_buffer<'b>(
    &mut self,
    state: Self::State,
    buffer: BorrowedReadBuffer<'b>,
    modules: &ExtModuleMap,
  ) -> Result<Self::State>;

  fn process_state(&mut self, state: Self::State) -> ControlFlow<(), Self::State>;
}

fn run_capture<S: Default, C: CaptureLoop<State = S>>(
  mut capture: C,
  infra: &mut TracingInfra,
  modules: &ExtModuleMap,
) -> Result<C> {
  let lg = Log::get("run_capture");
  let mut state = S::default();
  loop {
    let base_ptr: BorrowedReadBuffer<'_> = infra.wait_for_full_buffer()?;
    lg.trace("Received buffer");
    let st = capture.update_from_buffer(state, base_ptr, modules)?;
    infra.finish_buffer()?;

    if let ControlFlow::Continue(st) = capture.process_state(st) {
      state = st;
    } else {
      return Ok(capture);
    }
  }
}
