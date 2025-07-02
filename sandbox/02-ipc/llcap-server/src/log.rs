use std::sync::atomic::AtomicU8;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum LogLevel {
  Critical,
  Warn,
  Info,
  Trace,
}

impl From<u8> for LogLevel {
  fn from(value: u8) -> Self {
    match value {
      0 => LogLevel::Critical,
      1 => LogLevel::Warn,
      2 => LogLevel::Info,
      _ => LogLevel::Trace,
    }
  }
}

impl From<LogLevel> for u8 {
  fn from(value: LogLevel) -> Self {
    match value {
      LogLevel::Critical => 0,
      LogLevel::Warn => 1,
      LogLevel::Info => 2,
      LogLevel::Trace => 3,
    }
  }
}

#[derive(Clone, Copy)]
pub struct Log {
  level: LogLevel,
}

// bad, wrong, terrible but I don't think extensive customizable logging is needed
static LOG_CRIT: Log = Log {
  level: LogLevel::Critical,
};
static LOG_WARN: Log = Log {
  level: LogLevel::Warn,
};
static LOG_INFO: Log = Log {
  level: LogLevel::Info,
};
static LOG_TRACE: Log = Log {
  level: LogLevel::Trace,
};

static LOG_LEVEL: AtomicU8 = AtomicU8::new(0);

pub struct Logger {
  name: String,
  inner_log: &'static Log,
}

impl Logger {
  pub fn new(name: &str) -> Self {
    let log = match LogLevel::from(LOG_LEVEL.load(std::sync::atomic::Ordering::Relaxed)) {
      LogLevel::Critical => &LOG_CRIT,
      LogLevel::Warn => &LOG_WARN,
      LogLevel::Info => &LOG_INFO,
      LogLevel::Trace => &LOG_TRACE,
    };
    Self {
      name: name.to_string(),
      inner_log: log,
    }
  }

  fn formatted(&self, msg: &str) -> String {
    format!("[{}] {}", self.name, msg)
  }

  pub fn crit<T: AsRef<str>>(&self, msg: T) {
    self
      .inner_log
      .log(LogLevel::Critical, &self.formatted(msg.as_ref()));
  }
  pub fn warn<T: AsRef<str>>(&self, msg: T) {
    self
      .inner_log
      .log(LogLevel::Warn, &self.formatted(msg.as_ref()));
  }
  pub fn info<T: AsRef<str>>(&self, msg: T) {
    self
      .inner_log
      .log(LogLevel::Info, &self.formatted(msg.as_ref()));
  }
  pub fn trace<T: AsRef<str>>(&self, msg: T) {
    self
      .inner_log
      .log(LogLevel::Trace, &self.formatted(msg.as_ref()));
  }

  // an unconditional log
  pub fn progress<T: AsRef<str>>(&self, msg: T) {
    self.inner_log.log_progress(&self.formatted(msg.as_ref()))
  }
}

impl Log {
  pub fn set_verbosity(verbosity: u8) -> u8 {
    LOG_LEVEL.swap(verbosity, std::sync::atomic::Ordering::Relaxed)
  }

  pub fn get(name: &str) -> Logger {
    Logger::new(name)
  }

  fn log_level_preamble(lvl: LogLevel) -> &'static str {
    match lvl {
      LogLevel::Critical => "C |",
      LogLevel::Warn => "W |",
      LogLevel::Info => "I |",
      LogLevel::Trace => "T |",
    }
  }

  fn log(&self, lvl: LogLevel, msg: &str) {
    if u8::from(lvl) > u8::from(self.level) {
      return;
    }
    eprintln!("{} {}", Log::log_level_preamble(lvl), msg);
  }

  fn log_progress(&self, msg: &str) {
    println!("P | {msg}");
  }
}
