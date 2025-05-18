pub enum ReadProgress {
  Done {
    // reading of a value is done
    payload: Vec<u8>,      // result to be saved
    consumed_bytes: usize, // nr of bytes consumed from input
  },
  NotYet, // all bytes from input consumed, result not complete yet, send another buffer
  Nop,    // buffer left untouched, you should call reset()
}

pub enum WriteProgress {
  Done {
    // writing of a value is done
    consumed_bytes: usize, // nr of bytes consumed from input
  },
  NotYet, // all bytes from input consumed, write not complete yet, send another buffer
  Nop,
}

#[derive(Debug, Copy, Clone)]
pub enum ArgSizeTypeRef {
  Fixed(usize),
  Cstr,
  Custom,
}

impl TryFrom<u16> for ArgSizeTypeRef {
  type Error = String;

  fn try_from(id: u16) -> Result<Self, Self::Error> {
    match id {
      0..16 => Ok(Self::Fixed(id.into())),
      1026 => Ok(Self::Cstr),
      1027 => Ok(Self::Custom),
      _ => Err(format!("Unsupported argument size type: {id}")),
    }
  }
}

pub trait SizeTypeReader {
  fn read_reset(&mut self) -> bool;
  fn read(&mut self, data: &[u8]) -> Result<ReadProgress, String>;
  fn done(&self) -> bool;
}

pub trait SizeTypeWriter {
  fn write_reset(&mut self) -> bool;
  fn write(
    &mut self,
    data: &[u8],
    writer: impl Fn(u8) -> Result<(), String>,
  ) -> Result<WriteProgress, String>;
}

pub struct FixedSizeTyData {
  required_size: usize,
}

impl FixedSizeTyData {
  pub fn of_size(size: usize) -> Self {
    Self {
      required_size: size,
    }
  }
}

pub struct FixedSizeTyReader {
  data: FixedSizeTyData,
  done_read: bool,
  buffer: Vec<u8>,
}

pub struct FixedSizeTyWriter {
  data: FixedSizeTyData,
  progress_write: usize,
  done_write: bool,
}

impl FixedSizeTyReader {
  pub fn new(data: FixedSizeTyData) -> Self {
    Self {
      buffer: Vec::with_capacity(data.required_size),
      data,
      done_read: false,
    }
  }
}

impl SizeTypeReader for FixedSizeTyReader {
  fn read(&mut self, data: &[u8]) -> Result<ReadProgress, String> {
    if self.data.required_size == 0 {
      self.done_read = true;
      return Ok(ReadProgress::Done {
        payload: Vec::with_capacity(0),
        consumed_bytes: 0,
      });
    }

    if self.done_read {
      return Ok(ReadProgress::Nop);
    }

    if self.data.required_size < self.buffer.len() {
      return Err("Invalid condition - len!".to_string());
    }
    let remaining = self.data.required_size - self.buffer.len();
    if remaining == 0 {
      return Err("Invalid condition - rem!".to_string());
    }

    let to_cpy = remaining.min(data.len());
    for item in data.iter().take(to_cpy) {
      self.buffer.push(*item);
    }

    if remaining == to_cpy {
      let mut buff = Vec::with_capacity(self.data.required_size);
      std::mem::swap(&mut buff, &mut self.buffer);
      self.done_read = true;
      Ok(ReadProgress::Done {
        payload: buff,
        consumed_bytes: to_cpy,
      })
    } else {
      Ok(ReadProgress::NotYet)
    }
  }

  fn read_reset(&mut self) -> bool {
    if !self.done_read {
      return false;
    }
    if !self.buffer.is_empty() {
      return false;
    }

    self.done_read = false;
    true
  }

  fn done(&self) -> bool {
    self.done_read
  }
}

impl FixedSizeTyWriter {
  pub fn new(data: FixedSizeTyData) -> Self {
    Self {
      data,
      progress_write: 0,
      done_write: false,
    }
  }
}

impl SizeTypeWriter for FixedSizeTyWriter {
  fn write_reset(&mut self) -> bool {
    if !self.done_write {
      return false;
    }
    self.done_write = false;
    self.progress_write = 0;

    true
  }

  fn write(
    &mut self,
    data: &[u8],
    writer: impl Fn(u8) -> Result<(), String>,
  ) -> Result<WriteProgress, String> {
    if self.done_write {
      return Ok(WriteProgress::Nop);
    }

    if self.data.required_size < self.progress_write {
      return Err("Invalid condition - len!".to_string());
    }
    let remaining = self.data.required_size - self.progress_write;
    if remaining == 0 {
      return Err("Invalid condition - rem!".to_string());
    }

    let to_cpy = remaining.min(data.len());
    for item in data.iter().take(to_cpy) {
      writer(*item)?;
    }
    self.progress_write += to_cpy;

    if remaining == to_cpy {
      self.done_write = true;
      Ok(WriteProgress::Done {
        consumed_bytes: to_cpy,
      })
    } else {
      Ok(WriteProgress::NotYet)
    }
  }
}

pub enum CStringTypeReader {
  Start,
  Reading { payload: Vec<u8> },
  ReachedZero,
}

impl CStringTypeReader {
  pub fn new() -> Self {
    CStringTypeReader::Start
  }
}

// true if zero byte reached
fn consume_until_zero_or_end(out: &mut Vec<u8>, inp: &[u8]) -> bool {
  for i in inp {
    out.push(*i);
    if *i == 0 {
      return true;
    }
  }

  false
}

impl SizeTypeReader for CStringTypeReader {
  fn read_reset(&mut self) -> bool {
    if !self.done() {
      return false;
    }
    *self = CStringTypeReader::Start;
    true
  }

  fn read(&mut self, data: &[u8]) -> Result<ReadProgress, String> {
    // this "pair" thing is necessary to make borrow checker happy
    let (target, retval) = match self {
      CStringTypeReader::Start => {
        let mut output: Vec<u8> = vec![];
        if consume_until_zero_or_end(&mut output, data) {
          let len = output.len();
          (
            CStringTypeReader::ReachedZero,
            Ok(ReadProgress::Done {
              payload: output,
              consumed_bytes: len,
            }),
          )
        } else {
          (
            CStringTypeReader::Reading { payload: output },
            Ok(ReadProgress::NotYet),
          )
        }
      }
      CStringTypeReader::Reading { payload } => {
        let mut new_vec = vec![];
        if consume_until_zero_or_end(payload, data) {
          std::mem::swap(&mut new_vec, payload);
          let len = payload.len();
          (
            CStringTypeReader::ReachedZero,
            Ok(ReadProgress::Done {
              payload: new_vec,
              consumed_bytes: len,
            }),
          )
        } else {
          std::mem::swap(&mut new_vec, payload);
          (
            CStringTypeReader::Reading { payload: new_vec },
            Ok(ReadProgress::NotYet),
          )
        }
      }
      CStringTypeReader::ReachedZero => (CStringTypeReader::ReachedZero, Ok(ReadProgress::Nop)),
    };
    // I think this is not the best design (changing of self), but whatever
    *self = target;
    retval
  }

  fn done(&self) -> bool {
    matches!(self, Self::ReachedZero)
  }
}
