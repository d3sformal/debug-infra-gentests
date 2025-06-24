use std::{io::Error, marker::PhantomData};

use libc::{O_CREAT, O_EXCL, SEM_FAILED, c_int, mode_t, sem_open, sem_t};

use crate::log::Log;
use anyhow::{Result, bail, ensure};

use super::wrappers::{PERMS_PERMISSIVE, to_cstr};

/// a semaphore for IPC
pub enum Semaphore {
  Open {
    /// SAFETY INVARIANT: sem is a valid pointer to an initialized semaphore obtained via
    ///the correspodnging syscalls
    sem: *mut sem_t, // marks the type !Send & !Sync
    cname: String,
    marker: PhantomData<sem_t>,
  },
  /// this variant exists to disallow some interactions with semaphore after closing it
  Closed {
    // for our API, the sem potiner is not needed anymore @ this point (can change anytime ofc)
    cname: String,
  },
}

impl Semaphore {
  pub fn try_post(&mut self) -> Result<()> {
    match self {
      Semaphore::Open {
        sem,
        cname: _,
        marker: _pd,
      } => {
        // SAFETY: Self invariant
        ensure!(
          unsafe { libc::sem_post(*sem) } != -1,
          "Failed to post semaphore: {}",
          Error::last_os_error()
        );
        Ok(())
      }
      Semaphore::Closed { cname } => bail!("Post on closed semaphore {cname}"),
    }
  }

  pub fn try_wait(&mut self) -> Result<()> {
    match self {
      Semaphore::Open {
        sem,
        cname: _,
        marker: _pd,
      } =>
      // SAFETY: Self invariant
      {
        ensure!(
          unsafe { libc::sem_wait(*sem) } != -1,
          "Failed to wait on semaphore: {}",
          Error::last_os_error()
        );
        Ok(())
      }
      Semaphore::Closed { cname } => bail!("Wait on closed semaphore {cname}"),
    }
  }

  /// Consumes self. The error variant contains the un-closed `self` along with an error message
  pub fn try_close(self) -> Result<Semaphore, (Semaphore, String)> {
    match self {
      Semaphore::Open {
        sem,
        cname,
        marker: _pd,
      } => {
        // SAFETY: Self invariant
        if unsafe { libc::sem_close(sem) } != 0 {
          Err((
            Semaphore::Open {
              sem,
              cname: cname.clone(),
              marker: _pd,
            },
            format!(
              "Failed to close semaphore {}: {}",
              cname,
              Error::last_os_error()
            ),
          ))
        } else {
          Ok(Self::Closed { cname })
        }
      }
      Semaphore::Closed { cname } => Err((
        Semaphore::Closed {
          cname: cname.clone(),
        },
        format!("Close on closed semaphore {}", cname),
      )),
    }
  }

  pub fn cname(&self) -> &String {
    match self {
      Semaphore::Open {
        sem: _,
        cname,
        marker: _pd,
      } => cname,
      Semaphore::Closed { cname } => cname,
    }
  }

  /// Consumes self. The error variant contains the un-destroyed `self` along with an error message
  pub fn try_destroy(self) -> Result<(), (Self, String)> {
    let cname = self.cname();

    // SAFETY: Self invariant
    if unsafe { libc::sem_unlink(to_cstr(cname).as_ptr()) } != 0 {
      Err((
        self,
        format!("Failed to unlink semaphore: {}", Error::last_os_error()),
      ))
    } else {
      Ok(())
    }
  }

  /// opens a semaphore with the specified parameters
  pub fn try_open(
    name: &str,
    value: u32,
    flags: Option<c_int>,
    mode: Option<mode_t>,
  ) -> Result<Self> {
    let s_name = format!("{name}\x00");
    // SAFETY: line above
    let cstr_name = unsafe { to_cstr(&s_name) };
    // SAFETY: &CStr, syscall docs
    let result = unsafe {
      sem_open(
        cstr_name.as_ptr(),
        // the default here is kept exclusive - it's better to crash & specify "a connection to an existing semaphore" than possible bugs later
        flags.unwrap_or(O_CREAT | O_EXCL),
        mode.unwrap_or(PERMS_PERMISSIVE),
        value,
      )
    };

    ensure!(
      result != SEM_FAILED,
      "Failed to initialize semaphore {}: {}",
      name,
      Error::last_os_error()
    );

    Log::get("try_open").trace(format!("Opened semaphore {} with value {}", s_name, value));
    Ok(Self::Open {
      sem: result,
      cname: s_name,
      marker: PhantomData,
    })
  }

  pub fn try_open_exclusive(name: &str, value: u32) -> Result<Self> {
    Self::try_open(name, value, (O_CREAT | O_EXCL).into(), None)
  }
}

/// a helper struct for the creation of semaphore pairs for
/// shared memory management
pub struct FreeFullSemNames {
  pub free: String,
  pub full: String,
}

impl FreeFullSemNames {
  pub fn new(prefix: &str, category: &str, id: &str) -> Self {
    Self {
      free: format!("{prefix}-{category}-{id}-semfree"),
      full: format!("{prefix}-{category}-{id}-semfull"),
    }
  }
}
