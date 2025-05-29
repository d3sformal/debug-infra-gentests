use std::marker::PhantomData;

use libc::{O_CREAT, O_EXCL, SEM_FAILED, c_int, mode_t, sem_open, sem_t};

use crate::log::Log;

use super::wrappers::{PERMS_PERMISSIVE, get_errno_string, to_cstr};

/// a semaphore for IPC
/// it is expected that it will be used only in single-threaded context (due to the use of errno)
pub enum Semaphore {
  Open {
    // SAFETY INVARIANT: sem is a valid pointer to an initialized semaphore obtained via correspodnging syscalls
    sem: *mut sem_t, // should mark !Send & !Sync
    cname: String,
    marker: PhantomData<sem_t>,
  },
  // this variant exists to disallow some interactions with semaphore after closing it
  Closed {
    // for our API, the sem potiner is not needed anymore @ this point (can change anytime ofc)
    cname: String,
  },
}

impl Semaphore {
  pub fn try_post(&mut self) -> Result<(), String> {
    match self {
      Semaphore::Open {
        sem,
        cname: _,
        marker: _pd,
      } =>
      // SAFETY: Self invariant
      {
        if unsafe { libc::sem_post(*sem) } == -1 {
          Err(format!("Failed to post semaphore: {}", get_errno_string()))
        } else {
          Ok(())
        }
      }
      Semaphore::Closed { cname: _ } => Err("Semaphore is closed".to_string()),
    }
  }

  pub fn try_wait(&mut self) -> Result<(), String> {
    match self {
      Semaphore::Open {
        sem,
        cname: _,
        marker: _pd,
      } =>
      // SAFETY: Self invariant
      {
        if unsafe { libc::sem_wait(*sem) } == -1 {
          Err(format!(
            "Failed to wait on semaphore: {}",
            get_errno_string()
          ))
        } else {
          Ok(())
        }
      }
      Semaphore::Closed { cname: _ } => Err("Semaphore is closed".to_string()),
    }
  }

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
              cname,
              marker: _pd,
            },
            format!("Failed to close semaphore: {}", get_errno_string()),
          ))
        } else {
          Ok(Self::Closed { cname })
        }
      }
      Semaphore::Closed { cname: _ } => Err((self, "Semaphore is closed".to_string())),
    }
  }

  fn cname(&self) -> &String {
    match self {
      Semaphore::Open {
        sem: _,
        cname,
        marker: _pd,
      } => cname,
      Semaphore::Closed { cname } => cname,
    }
  }

  pub fn try_destroy(self) -> Result<(), (Self, String)> {
    let cname = self.cname();

    // SAFETY: Self invariant
    if unsafe { libc::sem_unlink(to_cstr(cname).as_ptr()) } != 0 {
      Err((
        self,
        format!("Failed to unlink semaphore: {}", get_errno_string()),
      ))
    } else {
      Ok(())
    }
  }

  // opens a semaphore and returns a valid Self object
  pub fn try_open(
    name: &str,
    value: u32,
    flags: Option<c_int>,
    mode: Option<mode_t>,
  ) -> Result<Self, String> {
    let s_name = format!("{name}\x00");
    // SAFETY: line above
    let cstr_name = unsafe { to_cstr(&s_name) };
    // SAFETY: &CStr, syscall docs
    let result = unsafe {
      sem_open(
        cstr_name.as_ptr(),
        flags.unwrap_or(O_CREAT | O_EXCL),
        mode.unwrap_or(PERMS_PERMISSIVE),
        value,
      )
    };

    if result == SEM_FAILED {
      Err(format!(
        "Failed to initialize semaphore {}: {}",
        name,
        get_errno_string()
      ))
    } else {
      Log::get("try_open").info(format!("Opened semaphore {} with value {}", s_name, value));
      Ok(Self::Open {
        sem: result,
        cname: s_name,
        marker: PhantomData,
      })
    }
  }

  pub fn try_open_exclusive(name: &str, value: u32) -> Result<Self, String> {
    Self::try_open(name, value, (O_CREAT | O_EXCL).into(), None)
  }
}

pub struct FreeFullSemNames {
  pub free: String,
  pub full: String,
}

impl FreeFullSemNames {
  pub fn get(prefix: &str, category: &str, id: &str) -> Self {
    Self {
      free: format!("{prefix}-{category}-{id}-semfree"),
      full: format!("{prefix}-{category}-{id}-semfull"),
    }
  }
}
