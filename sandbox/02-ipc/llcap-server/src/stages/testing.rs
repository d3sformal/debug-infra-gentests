use std::{
  fs::{self},
  mem,
  path::PathBuf,
  sync::{Arc, Mutex},
  time::Duration,
};

use tokio::{
  io::{AsyncReadExt, BufReader},
  net::{UnixListener, UnixStream, unix::OwnedWriteHalf},
  sync::oneshot::{Receiver, Sender},
  time::timeout,
};

use crate::{
  log::Log,
  modmap::{ExtModuleMap, IntegralFnId, IntegralModId},
  shmem_capture::hooklib_commons::{
    TAG_EXIT, TAG_FATAL, TAG_PKT, TAG_SGNL, TAG_START, TAG_TEST_END, TAG_TEST_FINISH, TAG_TIMEOUT,
  },
  stages::arg_capture::PacketReader,
};

use super::arg_capture::PacketProvider;

pub fn test_server_socket(prefix: &str) -> String {
  format!("/tmp{prefix}-test-server")
}

pub async fn test_server_job(
  prefix: String,
  packet_dir: PathBuf,
  modules: Arc<ExtModuleMap>,
  mem_limit: usize,
  mut end_rx: Receiver<()>,
  ready_tx: Sender<()>,
  results: Arc<Mutex<TestResults>>,
) -> Result<(), String> {
  let lg = Log::get("test_server_job");
  let path = test_server_socket(&prefix);
  lg.info(format!("Starting at {path}"));
  let listener = UnixListener::bind(path.clone()).map_err(|e| e.to_string())?;
  ready_tx.send(()).map_err(|_| "Receiver dropped")?;
  lg.info("Listening");

  let mut handles = vec![];

  while end_rx.try_recv().is_err() {
    match timeout(Duration::from_millis(100), listener.accept()).await {
      Ok(Ok((client_stream, client_addr))) => {
        lg.trace(format!("Connected test {:?}", client_addr));
        let results = results.clone();
        handles.push(tokio::spawn(single_test_job(
          client_stream,
          packet_dir.to_path_buf(),
          modules.clone(),
          mem_limit,
          results,
        )));
      }
      Ok(Err(e)) => Err(e.to_string())?,
      Err(_) => continue,
    }
  }
  lg.info("Finishing");
  for handle in handles {
    lg.trace("Finishing handle");
    handle.await.map_err(|e| e.to_string())?;
  }
  mem::drop(listener);
  fs::remove_file(path).map_err(|e| e.to_string())?;
  Ok(())
}

#[derive(Debug, Clone, Copy)]
enum TestMessage {
  Start(IntegralModId, IntegralFnId),
  PacketRequest(u64),
  EndTest(u64, TestStatus),
  End,
}

#[derive(Debug, Clone, Copy)]
pub enum TestStatus {
  Timeout,
  #[allow(dead_code)]
  Exit(i32),
  #[allow(dead_code)]
  Signal(i32),
  Fatal,
}

pub type TestResults = Vec<(IntegralModId, IntegralFnId, u64, TestStatus)>;

impl TryFrom<&[u8]> for TestStatus {
  type Error = String;

  fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
    let (tag, data) = value.split_at(2);
    if tag.starts_with(&TAG_TIMEOUT.to_le_bytes()) {
      Ok(Self::Timeout)
    } else if tag.starts_with(&TAG_EXIT.to_le_bytes()) {
      let sized: [u8; 4] = data
        .try_into()
        .map_err(|e: std::array::TryFromSliceError| e.to_string())?;
      Ok(Self::Exit(i32::from_le_bytes(sized)))
    } else if tag.starts_with(&TAG_SGNL.to_le_bytes()) {
      let sized: [u8; 4] = data
        .try_into()
        .map_err(|e: std::array::TryFromSliceError| e.to_string())?;
      Ok(Self::Signal(i32::from_le_bytes(sized)))
    } else if tag.starts_with(&TAG_FATAL.to_le_bytes()) {
      Ok(Self::Fatal)
    } else {
      Err(format!("Invalid status format: {:?}", value))
    }
  }
}

pub fn consume_to_u32(bytes: &[u8], start: usize) -> Result<u32, String> {
  if bytes.len() < start + 4 {
    return Err("Not enough bytes".to_string());
  }
  let le_bytes = [
    *bytes.get(start).unwrap(),
    *bytes.get(start + 1).unwrap(),
    *bytes.get(start + 2).unwrap(),
    *bytes.get(start + 3).unwrap(),
  ];
  let num = u32::from_le_bytes(le_bytes);
  Ok(num)
}

impl TryFrom<&[u8]> for TestMessage {
  type Error = String;

  fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
    let (tag, data) = value.split_at(2);
    if tag.starts_with(&TAG_START.to_le_bytes()) {
      Ok(Self::Start(
        IntegralModId(consume_to_u32(data, 0)?),
        IntegralFnId(consume_to_u32(data, 4)?),
      ))
    } else if tag.starts_with(&TAG_PKT.to_le_bytes()) {
      let sized: [u8; 8] = data
        .split_at(8)
        .0
        .try_into()
        .map_err(|e: std::array::TryFromSliceError| e.to_string() + " pktrq")?;
      Ok(Self::PacketRequest(u64::from_le_bytes(sized)))
    } else if tag.starts_with(&TAG_TEST_END.to_le_bytes()) {
      let (s1, s2) = data.split_at(8);
      let sized: [u8; 8] = s1
        .try_into()
        .map_err(|e: std::array::TryFromSliceError| e.to_string() + " msgend")?;
      Ok(Self::EndTest(
        u64::from_le_bytes(sized),
        TestStatus::try_from(s2)?,
      ))
    } else if tag.starts_with(&TAG_TEST_FINISH.to_le_bytes()) {
      Ok(Self::End)
    } else {
      Err(format!("Invalid msg format: {:?}", value))
    }
  }
}

#[derive(Debug)]
enum ClientState {
  Init,
  Started(IntegralModId, IntegralFnId),
  Ended,
}

async fn single_test_job(
  stream: UnixStream,
  packet_dir: PathBuf,
  modules: Arc<ExtModuleMap>,
  mem_limit: usize,
  results: Arc<Mutex<TestResults>>,
) {
  let mut state = ClientState::Init;
  let mut lg = Log::get(&format!("single_test_job@{:?}", state));
  let mut packets = match PacketReader::new(&packet_dir, &modules, mem_limit) {
    Ok(p) => p,
    Err(e) => {
      lg.crit(e.to_string());
      return;
    }
  };
  let (read, mut write) = stream.into_split();
  let mut buff_stream = BufReader::new(read);

  loop {
    lg.trace("Restart");
    let mut data = [0u8; 16];
    match timeout(Duration::from_millis(100), buff_stream.read(&mut data)).await {
      Ok(Ok(0)) => {
        lg.trace("Client closed connection - ending");
        return;
      }
      Ok(Ok(n)) => {
        if n != data.len() {
          lg.warn(format!("Expected {} bytes, got {n} - ending", data.len()));
          return;
        }
      }
      Ok(Err(e)) => {
        lg.warn(format!("Not readable {e} - ending"));
        return;
      }
      Err(_) => continue,
    }

    lg.trace(format!("Read done: {:?}", data));
    let msg = match TestMessage::try_from(data.as_slice()) {
      Ok(msg) => msg,
      Err(e) => {
        lg.crit(e);
        return;
      }
    };

    let (new_state, response) = match handle_client_msg(state, msg, &mut packets, results.clone()) {
      Ok((s, r)) => (s, r),
      Err(e) => {
        lg.crit(e);
        return;
      }
    };
    state = new_state;
    lg = Log::get(&format!("single_test_job@{:?}", state));
    if matches!(state, ClientState::Ended) {
      lg.trace("Client ended");
      return;
    }

    lg.trace(format!("Response: {:?}", response));
    if response.is_none() {
      continue;
    }
    match send_response_to_client(
      &mut write,
      &(response.as_ref().unwrap().len() as u32).to_le_bytes(),
    )
    .await
    {
      Ok(_) => (),
      Err(e) => {
        lg.crit(e);
        return;
      }
    }

    match send_response_to_client(&mut write, response.unwrap().as_slice()).await {
      Ok(_) => (),
      Err(e) => {
        lg.crit(e);
        return;
      }
    }
  }
}

fn handle_client_msg(
  state: ClientState,
  msg: TestMessage,
  packets: &mut dyn PacketProvider,
  results: Arc<Mutex<TestResults>>,
) -> Result<(ClientState, Option<Vec<u8>>), String> {
  match state {
    ClientState::Init => match msg {
      TestMessage::Start(m, f) => Ok((ClientState::Started(m, f), None)),
      TestMessage::End => Ok((ClientState::Ended, None)),
      _ => Err(format!(
        "Invalid transition from Init state with msg {:?}",
        msg
      )),
    },
    ClientState::Started(mod_id, fn_id) => match msg {
      TestMessage::EndTest(test_index, status) => {
        Log::get("handle_client_msg").info(format!(
          "test {} {} ended: {:?}, idx: {}",
          mod_id.hex_string(),
          fn_id.hex_string(),
          status,
          test_index
        ));
        results
          .lock()
          .unwrap()
          .push((mod_id, fn_id, test_index, status));
        Ok((state, None))
      }
      TestMessage::PacketRequest(idx) => {
        let response = packets.get_packet(mod_id, fn_id, idx as usize);
        Ok((ClientState::Started(mod_id, fn_id), response))
      }
      TestMessage::End => Ok((ClientState::Ended, None)),
      _ => Err(format!(
        "Invalid transition from Started state with msg {:?}",
        msg
      )),
    },
    ClientState::Ended => Err(format!(
      "Client state is Ended, no more messages were expected msg: {:?}, from state: {:?}",
      msg, state
    )),
  }
}

async fn send_response_to_client(stream: &mut OwnedWriteHalf, data: &[u8]) -> Result<(), String> {
  let mut idx = 0;
  loop {
    stream.writable().await.map_err(|e| e.to_string())?;

    match stream.try_write(data.split_at(idx).1) {
      Ok(n) => {
        if n == data.len() {
          return Ok(());
        }
        idx += n;
        continue;
      }
      Err(ref e) if e.kind() == tokio::io::ErrorKind::WouldBlock => {
        continue;
      }
      Err(e) => {
        return Err(e.to_string());
      }
    }
  }
}
