use anyhow::{Result, anyhow, ensure};

use crate::log::Log;

#[derive(Debug)]
pub enum ReadProgress {
  /// reading of a value is done
  Done {
    /// result to be saved
    payload: Vec<u8>,
    /// nr of bytes consumed from input
    consumed_bytes: usize,
  },
  /// all bytes from input consumed, result not complete yet, send another buffer
  NotYet,
  /// buffer left untouched, you should call reset()
  Nop,
}

#[derive(Debug, Copy, Clone)]
pub enum ArgSizeTypeRef {
  Fixed(usize),
  Cstr,
  Custom,
}

impl TryFrom<u16> for ArgSizeTypeRef {
  type Error = anyhow::Error;

  fn try_from(id: u16) -> Result<Self, Self::Error> {
    match id {
      0..16 => Ok(Self::Fixed(id.into())),
      1026 => Ok(Self::Cstr),
      1027 => Ok(Self::Custom),
      _ => Err(anyhow!("Unsupported argument size type: {id}")),
    }
  }
}

// interface for argument readers
pub trait SizeTypeReader {
  /// resets the reader, acts as a no-op if reader is not finished
  ///
  /// returns true if reader was reset
  fn read_reset(&mut self) -> bool;
  /// consumes bytes from data
  /// may consume any number of bytes (up to length), refer to ReadProgress
  /// for return value information
  fn read(&mut self, data: &[u8]) -> Result<ReadProgress>;
  /// indicates that reader has finished reading, data is ready
  fn done(&self) -> bool;
}

pub struct FixedSizeTyReader {
  size: usize,
  done_read: bool,
  buffer: Vec<u8>,
}

impl FixedSizeTyReader {
  pub fn of_size(size: usize) -> Self {
    Self {
      buffer: Vec::with_capacity(size),
      size,
      done_read: false,
    }
  }
}

impl SizeTypeReader for FixedSizeTyReader {
  fn read(&mut self, data: &[u8]) -> Result<ReadProgress> {
    if self.size == 0 {
      // special case of the zero-sized reader
      self.done_read = true;
      return Ok(ReadProgress::Done {
        payload: Vec::with_capacity(0),
        consumed_bytes: 0,
      });
    }

    if self.done_read {
      return Ok(ReadProgress::Nop);
    }

    ensure!(
      self.size >= self.buffer.len(),
      "Invalid fixed reader condition - len!"
    );

    // what we wish to read
    let remaining = self.size - self.buffer.len();
    ensure!(
      remaining != 0,
      "Invalid fixed reader condition - remaining!"
    );

    let to_cpy = remaining.min(data.len());
    for item in data.iter().take(to_cpy) {
      self.buffer.push(*item);
    }

    if remaining == to_cpy {
      // we read everything we needed
      let mut buff = Vec::with_capacity(self.size);
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
    const FN: &str = "read_reset";
    if !self.done_read {
      Log::get(FN).warn("Reset on unfinished reader");
      return false;
    }
    if !self.buffer.is_empty() {
      Log::get(FN).warn("Reset without consuming the reader's buffer");
      return false;
    }

    self.done_read = false;
    true
  }

  fn done(&self) -> bool {
    self.done_read
  }
}

// reads a 0x00-terminated string
// (so far unused in the instrumentation)
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
    *self = Self::Start;
    true
  }

  fn read(&mut self, data: &[u8]) -> Result<ReadProgress> {
    // this "pair" thing is necessary to make borrow checker happy
    let (newstate, retval) = match self {
      CStringTypeReader::Start => {
        let mut output: Vec<u8> = vec![];
        if consume_until_zero_or_end(&mut output, data) {
          let len = output.len();
          (
            Some(CStringTypeReader::ReachedZero),
            Ok(ReadProgress::Done {
              payload: output,
              consumed_bytes: len,
            }),
          )
        } else {
          (
            Some(CStringTypeReader::Reading { payload: output }),
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
            Some(CStringTypeReader::ReachedZero),
            Ok(ReadProgress::Done {
              payload: new_vec,
              consumed_bytes: len,
            }),
          )
        } else {
          std::mem::swap(&mut new_vec, payload);
          (None, Ok(ReadProgress::NotYet))
        }
      }
      CStringTypeReader::ReachedZero => (None, Ok(ReadProgress::Nop)),
    };
    // I think this is not the best design (changing of self), but whatever
    if let Some(newstate) = newstate {
      *self = newstate;
    }
    retval
  }

  fn done(&self) -> bool {
    matches!(self, Self::ReachedZero)
  }
}

/// Returns start + number of bytes consumed
fn take_num_into_slice(n: usize, start: usize, out: &mut [u8; 8], inp: &[u8]) -> usize {
  let mut offs = start;
  for i in inp.iter().take(n) {
    out[offs] = *i;
    offs += 1;
  }

  offs
}

// the size of the mandatory "length" field of the LLSZ_CUSTOM types
const CUSTOM_TYPE_SIZE_SPEC_SIZE: usize = 8;
#[derive(Debug)]
pub enum CustomTypeReader {
  Start,
  // in the middle of reading the 8-byte size
  ReadingTgtSize {
    idx: u8,
    bytes: [u8; CUSTOM_TYPE_SIZE_SPEC_SIZE],
  },
  // finished reading the size, now reading the payload (of length target_size)
  Reading {
    target_size: u64,
    payload: Vec<u8>,
  },
  Finished,
}
impl CustomTypeReader {
  pub fn new() -> Self {
    Self::Start
  }
}

impl SizeTypeReader for CustomTypeReader {
  fn read_reset(&mut self) -> bool {
    if !self.done() {
      return false;
    }
    *self = Self::Start;
    true
  }

  fn read(&mut self, data: &[u8]) -> Result<ReadProgress> {
    let (newself, result) = match self {
      CustomTypeReader::Start => {
        let mut tgt_sz_buff = [0u8; 8];
        let idx = take_num_into_slice(8, 0, &mut tgt_sz_buff, data);
        if idx == tgt_sz_buff.len() && tgt_sz_buff.len() == data.len() {
          (
            Some(CustomTypeReader::Reading {
              target_size: u64::from_le_bytes(tgt_sz_buff),
              payload: vec![],
            }),
            ReadProgress::NotYet,
          )
        } else if idx == tgt_sz_buff.len() && tgt_sz_buff.len() < data.len() {
          let mut payload = vec![];
          perform_reading_stage(
            data,
            idx,
            u64::from_le_bytes(tgt_sz_buff),
            &mut payload,
            tgt_sz_buff.len(),
          )
        } else {
          (
            Some(CustomTypeReader::ReadingTgtSize {
              idx: idx as u8,
              bytes: tgt_sz_buff,
            }),
            ReadProgress::NotYet,
          )
        }
      }
      CustomTypeReader::ReadingTgtSize { idx, bytes } => {
        let uidx = *idx as usize;
        let offs = take_num_into_slice(8 - uidx, uidx, bytes, data);
        if offs == bytes.len() {
          (
            Some(CustomTypeReader::Reading {
              target_size: u64::from_le_bytes(*bytes),
              payload: vec![],
            }),
            ReadProgress::NotYet,
          )
        } else {
          *idx = offs as u8;
          (None, ReadProgress::NotYet)
        }
      }
      CustomTypeReader::Reading {
        target_size,
        payload,
      } => perform_reading_stage(data, 0, *target_size, payload, 0),
      CustomTypeReader::Finished => (None, ReadProgress::Nop),
    };
    if let Some(newself) = newself {
      *self = newself;
    }
    Ok(result)
  }

  fn done(&self) -> bool {
    matches!(self, CustomTypeReader::Finished)
  }
}

fn perform_reading_stage(
  data: &[u8],
  offset: usize,
  target_size: u64,
  payload: &mut Vec<u8>,
  previous_read: usize,
) -> (Option<CustomTypeReader>, ReadProgress) {
  let to_read = target_size as usize - payload.len();
  for b in data.iter().skip(offset).take(to_read) {
    payload.push(*b);
  }

  if to_read == 0 || to_read <= data.len() - offset {
    let mut exhg = vec![];
    std::mem::swap(&mut exhg, payload);
    (
      Some(CustomTypeReader::Finished),
      ReadProgress::Done {
        payload: exhg,
        consumed_bytes: to_read + previous_read,
      },
    )
  } else {
    let mut swp = vec![];
    std::mem::swap(&mut swp, payload);
    (
      Some(CustomTypeReader::Reading {
        target_size,
        payload: swp,
      }),
      ReadProgress::NotYet,
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // various tests checking (among other things) that the readers output
  // the correct state when presented with differently chunked data

  // fix_r => fixed-size reader
  #[test]
  fn fix_r_zero_done_on_init() {
    let zero_reader = FixedSizeTyReader::of_size(0);
    assert!(!zero_reader.done());
  }

  #[test]
  fn fix_r_zero_does_not_consume() {
    let data = [0u8, 0u8];
    let mut zero_reader = FixedSizeTyReader::of_size(0);
    assert!(
      matches!(zero_reader.read(&data), Ok(ReadProgress::Done { payload, consumed_bytes }) if payload.len() == 0 && consumed_bytes == 0 )
    );
  }

  #[test]
  fn fix_r_zero_does_not_consume_empty() {
    let data = [];
    let mut zero_reader = FixedSizeTyReader::of_size(0);
    assert!(
      matches!(zero_reader.read(&data), Ok(ReadProgress::Done { payload, consumed_bytes }) if payload.len() == 0 && consumed_bytes == 0 )
    );
  }

  #[test]
  fn fix_r_zero_done_after_read() {
    let data = [0u8, 0u8];
    let mut zero_reader = FixedSizeTyReader::of_size(0);
    let _ = zero_reader.read(&data);
    assert!(zero_reader.done());
  }

  #[test]
  fn fix_r_nonzero_done_on_init() {
    let reader = FixedSizeTyReader::of_size(2);
    assert!(!reader.done());
  }

  #[test]
  fn fix_r_nonzero_read_exact() {
    let data = [1u8, 2u8];
    let mut reader = FixedSizeTyReader::of_size(data.len());
    assert!(
      matches!(reader.read(&data), Ok(ReadProgress::Done { payload, consumed_bytes }) if payload.len() ==  data.len() && payload[0] == data[0] && payload[1] == data[1] && consumed_bytes == data.len() )
    );
  }

  #[test]
  fn fix_r_nonzero_done_after_read_exact() {
    let data = [1u8, 2u8];
    let mut reader = FixedSizeTyReader::of_size(data.len());
    let _ = reader.read(&data);
    assert!(reader.done());
  }

  #[test]
  fn fix_r_nonzero_read_more() {
    let data = [1u8, 2u8, 3u8];
    let sz = data.len() - 1;
    let mut reader = FixedSizeTyReader::of_size(sz);
    assert!(
      matches!(reader.read(&data), Ok(ReadProgress::Done { payload, consumed_bytes }) if payload.len() == sz && payload[0] == data[0] && payload[1] == data[1] && consumed_bytes == sz )
    );
  }

  #[test]
  fn fix_r_nonzero_done_after_read_more() {
    let data = [1u8, 2u8, 3u8];
    let sz = data.len() - 1;
    let mut reader = FixedSizeTyReader::of_size(sz);
    let _ = reader.read(&data);
    assert!(reader.done());
  }

  #[test]
  fn fix_r_nonzero_not_done_after_read_less() {
    let data = [1u8, 2u8];
    let sz = data.len() + 1;
    let mut reader = FixedSizeTyReader::of_size(sz);
    let _ = reader.read(&data);
    assert!(!reader.done());
  }

  #[test]
  fn fix_r_nonzero_read_less() {
    let data = [1u8, 2u8];
    let sz = data.len() + 1;
    let mut reader = FixedSizeTyReader::of_size(sz);
    matches!(reader.read(&data), Ok(ReadProgress::NotYet));
  }

  #[test]
  fn fix_r_nonzero_read_zero() {
    let data = [];
    let sz = 1;
    let mut reader = FixedSizeTyReader::of_size(sz);
    matches!(reader.read(&data), Ok(ReadProgress::NotYet));
  }

  #[test]
  fn fix_r_nonzero_read_less_and_exact() {
    let data = [2u8];
    let data2 = [111u8];
    let sz = data.len() + 1;
    let mut reader = FixedSizeTyReader::of_size(sz);
    let _ = reader.read(&data);
    let res = reader.read(&data2);
    assert!(
      matches!(res, Ok(ReadProgress::Done { payload, consumed_bytes }) if payload.len() == sz && consumed_bytes == 1 && payload[0] == data[0] && payload[1] == data2[0])
    );
  }

  #[test]
  fn fix_r_nonzero_done_after_read_less_and_exact() {
    let data = [2u8];
    let data2 = [111u8];
    let sz = data.len() + 1;
    let mut reader = FixedSizeTyReader::of_size(sz);
    let _ = reader.read(&data);
    let _ = reader.read(&data2);
    assert!(reader.done());
  }

  #[test]
  fn fix_r_nonzero_empty_reset() {
    let mut reader = FixedSizeTyReader::of_size(1);
    assert!(!reader.read_reset());
  }

  #[test]
  fn fix_r_nonzero_nonnempty_reset() {
    let data = [1u8, 2u8];
    let data_smaller = [1u8];
    let mut reader = FixedSizeTyReader::of_size(data.len());
    let _ = reader.read(&data_smaller);
    assert!(!reader.read_reset());
  }

  #[test]
  fn fix_r_nonzero_read_after_reset() {
    let data = [1u8, 2u8];
    let mut reader = FixedSizeTyReader::of_size(data.len());
    let _ = reader.read(&data);
    reader.read_reset();
    assert!(
      matches!(reader.read(&data), Ok(ReadProgress::Done { payload, consumed_bytes }) if payload.len() ==  data.len() && payload[0] == data[0] && payload[1] == data[1] && consumed_bytes == data.len() )
    );
  }

  #[test]
  fn fix_r_zero_empty_reset() {
    let mut reader = FixedSizeTyReader::of_size(0);
    assert!(!reader.read_reset());
  }

  #[test]
  fn fix_r_zero_reset_after_empty_read() {
    let mut reader = FixedSizeTyReader::of_size(0);
    let _ = reader.read(&[]);
    assert!(reader.read_reset());
  }

  #[test]
  fn fix_r_zero_reset_after_nonempty_read() {
    let mut reader = FixedSizeTyReader::of_size(0);
    let _ = reader.read(&[1, 2]);
    assert!(reader.read_reset());
  }

  #[test]
  fn cus_r_read_init_not_done() {
    let reader = CustomTypeReader::new();
    assert!(!reader.done());
  }
  #[test]
  fn cus_r_read_zero_size() {
    let mut reader = CustomTypeReader::new();
    let res = reader.read(&(0u8).to_le_bytes());
    assert!(matches!(res, Ok(ReadProgress::NotYet)));
  }

  fn make_packet(data: &[u8]) -> Vec<u8> {
    let mut res: Vec<u8> = vec![];
    res.append(&mut data.len().to_le_bytes().to_vec());
    res.append(&mut data.to_vec());
    res
  }

  #[test]
  fn cus_r_read_size_less() {
    let mut reader = CustomTypeReader::new();
    let num_data = 123098u32;
    let pkt = make_packet(&num_data.to_le_bytes());
    let (len, _) = pkt.split_at(8);
    let (len_less, _) = len.split_at(4);
    let res = reader.read(len_less);
    assert!(matches!(res, Ok(ReadProgress::NotYet)));
    assert!(matches!(
      reader,
      CustomTypeReader::ReadingTgtSize { idx: _, bytes: _ }
    ));
  }

  fn assert_reader_finished(
    res: Result<ReadProgress>,
    reader: CustomTypeReader,
    expected_parsed: u32,
    expected_consumed: usize,
  ) {
    assert!(
      matches!(res, Ok(ReadProgress::Done { payload, consumed_bytes }) if payload.len() == 4 && u32::from_le_bytes([payload[0], payload[1], payload[2], payload[3]]) == expected_parsed && consumed_bytes == expected_consumed)
    );
    assert!(matches!(reader, CustomTypeReader::Finished));
  }
  #[test]
  fn cus_r_read_size_exact_w_data_exact() {
    let mut reader = CustomTypeReader::new();
    let num_data = 123098u32;
    let pkt = make_packet(&num_data.to_le_bytes());
    let (len, data) = pkt.split_at(8);
    let res = reader.read(len);
    assert!(matches!(res, Ok(ReadProgress::NotYet)));

    let res = reader.read(data);
    assert_reader_finished(res, reader, num_data, data.len());
  }

  #[test]
  fn cus_r_read_size_less_exact_w_data_exact() {
    let mut reader = CustomTypeReader::new();
    let num_data = 123098u32;
    let pkt = make_packet(&num_data.to_le_bytes());
    let (len, data) = pkt.split_at(8);
    let (len_less, len_ex) = len.split_at(4);
    let res = reader.read(len_less);
    assert!(matches!(res, Ok(ReadProgress::NotYet)));
    assert!(matches!(
      reader,
      CustomTypeReader::ReadingTgtSize { idx: _, bytes: _ }
    ));

    let res = reader.read(len_ex);
    assert!(matches!(res, Ok(ReadProgress::NotYet)));
    assert!(matches!(
      reader,
      CustomTypeReader::Reading {
        target_size: _,
        payload: _
      }
    ));

    let res = reader.read(data);
    assert_reader_finished(res, reader, num_data, data.len());
  }

  #[test]
  fn cus_r_read_size_exact_w_data_less() {
    let mut reader = CustomTypeReader::new();
    let num_data = 123098u32;
    let pkt = make_packet(&num_data.to_le_bytes());
    let (len, data) = pkt.split_at(8);
    let res = reader.read(len);
    assert!(matches!(res, Ok(ReadProgress::NotYet)));

    let (data_less, _) = data.split_at(1);
    let res = reader.read(data_less);
    assert!(matches!(res, Ok(ReadProgress::NotYet)));
    assert!(matches!(
      reader,
      CustomTypeReader::Reading {
        target_size: _,
        payload: _
      }
    ));
  }

  #[test]
  fn cus_r_read_size_less_exact_w_data_less() {
    let mut reader = CustomTypeReader::new();
    let num_data = 123098u32;
    let pkt = make_packet(&num_data.to_le_bytes());
    let (len, data) = pkt.split_at(8);
    let (len_less, len_ex) = len.split_at(4);
    let res = reader.read(len_less);
    assert!(matches!(res, Ok(ReadProgress::NotYet)));
    assert!(matches!(
      reader,
      CustomTypeReader::ReadingTgtSize { idx: _, bytes: _ }
    ));
    let res = reader.read(len_ex);
    assert!(matches!(res, Ok(ReadProgress::NotYet)));
    assert!(matches!(
      reader,
      CustomTypeReader::Reading {
        target_size: _,
        payload: _
      }
    ));

    let (data_less, _) = data.split_at(1);
    let res = reader.read(data_less);
    assert!(matches!(res, Ok(ReadProgress::NotYet)));
    assert!(matches!(
      reader,
      CustomTypeReader::Reading {
        target_size: _,
        payload: _
      }
    ));
  }

  #[test]
  fn cus_r_read_size_exact_w_data_less_exact() {
    let mut reader = CustomTypeReader::new();
    let num_data = 123098u32;
    let pkt = make_packet(&num_data.to_le_bytes());
    let (len, data) = pkt.split_at(8);
    let _ = reader.read(len);

    let (data_less, data_exact) = data.split_at(1);
    let _ = reader.read(data_less);
    let res = reader.read(data_exact);
    assert_reader_finished(res, reader, num_data, data_exact.len());
  }

  #[test]
  fn cus_r_read_size_less_exact_w_data_less_exact() {
    let mut reader = CustomTypeReader::new();
    let num_data = 123098u32;
    let pkt = make_packet(&num_data.to_le_bytes());
    let (len, data) = pkt.split_at(8);
    let (len_less, len_ex) = len.split_at(4);
    let _ = reader.read(len_less);
    let _ = reader.read(len_ex);

    let (data_less, data_exact) = data.split_at(1);
    let _ = reader.read(data_less);
    let res = reader.read(data_exact);
    assert_reader_finished(res, reader, num_data, data_exact.len());
  }

  #[test]
  fn cus_r_read_payload_exact() {
    let mut reader = CustomTypeReader::new();
    let num_data = 123098u32;
    let pkt = make_packet(&num_data.to_le_bytes());
    let res = reader.read(&pkt);
    assert_reader_finished(res, reader, num_data, pkt.len());
  }

  #[test]
  fn cus_r_read_payload_exact_reset() {
    let mut reader = CustomTypeReader::new();
    let num_data = 123098u32;
    let pkt = make_packet(&num_data.to_le_bytes());
    let _ = reader.read(&pkt);
    assert!(reader.read_reset());
  }

  #[test]
  fn cus_r_read_empty_reset() {
    let mut reader = CustomTypeReader::new();
    assert!(!reader.read_reset());
  }

  #[test]
  fn cus_r_read_after_reset() {
    let mut reader = CustomTypeReader::new();
    let num_data = 123098u32;
    let pkt = make_packet(&num_data.to_le_bytes());
    let _ = reader.read(&pkt);
    reader.read_reset();
    let num_data = 123098u32;
    let pkt = make_packet(&num_data.to_le_bytes());
    let res = reader.read(&pkt);
    assert_reader_finished(res, reader, num_data, pkt.len());
  }
}
