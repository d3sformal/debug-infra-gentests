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

impl Log {
  pub fn get(verbosity: u8) -> &'static Self {
    match LogLevel::from(verbosity) {
      LogLevel::Critical => &LOG_CRIT,
      LogLevel::Warn => &LOG_WARN,
      LogLevel::Info => &LOG_INFO,
      LogLevel::Trace => &LOG_TRACE,
    }
  }

  fn log_level_preamble(lvl: LogLevel) -> &'static str {
    match lvl {
      LogLevel::Critical => "C |",
      LogLevel::Warn => "W |",
      LogLevel::Info => "I |",
      LogLevel::Trace => "T |",
    }
  }

  pub fn crit(&self, msg: &str) {
    self.log(LogLevel::Critical, msg);
  }
  pub fn warn(&self, msg: &str) {
    self.log(LogLevel::Warn, msg);
  }
  pub fn info(&self, msg: &str) {
    self.log(LogLevel::Info, msg);
  }
  pub fn trace(&self, msg: &str) {
    self.log(LogLevel::Trace, msg);
  }

  fn log(&self, lvl: LogLevel, msg: &str) {
    if u8::from(lvl) > u8::from(self.level) {
      return;
    }
    eprintln!("{} {}", Log::log_level_preamble(lvl), msg);
  }
}
