use std::{
  path::PathBuf,
  sync::{Arc, atomic::AtomicBool},
  time::Duration,
};

use tokio::{
  io::{AsyncReadExt, BufReader},
  net::{UnixListener, UnixStream, unix::OwnedWriteHalf},
  time::timeout,
};

use crate::{
  log::Log,
  modmap::{ExtModuleMap, IntegralFnId, IntegralModId},
  shmem_capture::zmq_channels::consume_to_u32,
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
  termination_flag: Arc<AtomicBool>,
) -> Result<(), String> {
  let lg = Log::get("test_server_job");
  let path = test_server_socket(&prefix);
  lg.info(format!("Starting at {path}"));
  let listener = UnixListener::bind(path).map_err(|e| e.to_string())?;
  lg.info("Listening");
  while !termination_flag.load(std::sync::atomic::Ordering::Relaxed) {
    match timeout(Duration::from_millis(500), listener.accept()).await {
      Ok(Ok((client_stream, client_addr))) => {
        lg.trace(format!("Connected test {:?}", client_addr));
        std::mem::drop(tokio::spawn(single_test_job(
          client_stream,
          packet_dir.to_path_buf(),
          modules.clone(),
          mem_limit,
        )));
      }
      Ok(Err(e)) => Err(e.to_string())?,
      Err(_) => continue,
    }
  }
  lg.info("Finishing");
  Ok(())
}

#[derive(Debug, Clone, Copy)]
enum TestMessage {
  Start(IntegralModId, IntegralFnId),
  PacketRequest(u64),
  End(TestStatus),
}

#[derive(Debug, Clone, Copy)]
enum TestStatus {
  Timeout,
  Exit(u32),
  Signal(u32),
}

impl TryFrom<&[u8]> for TestStatus {
  type Error = String;

  fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
    const MSG_TAG_TIMEOUT: u16 = 15;
    const MSG_TAG_EXIT: u16 = 16;
    const MSG_TAG_SGNL: u16 = 17;
    let (tag, data) = value.split_at(2);
    if tag.starts_with(&MSG_TAG_TIMEOUT.to_le_bytes()) {
      Ok(Self::Timeout)
    } else if tag.starts_with(&MSG_TAG_EXIT.to_le_bytes()) {
      let sized: [u8; 4] = data
        .try_into()
        .map_err(|e: std::array::TryFromSliceError| e.to_string())?;
      Ok(Self::Exit(u32::from_le_bytes(sized)))
    } else if tag.starts_with(&MSG_TAG_SGNL.to_le_bytes()) {
      let sized: [u8; 4] = data
        .try_into()
        .map_err(|e: std::array::TryFromSliceError| e.to_string())?;
      Ok(Self::Signal(u32::from_le_bytes(sized)))
    } else {
      Err(format!("Invalid status format: {:?}", value))
    }
  }
}

impl TryFrom<&[u8]> for TestMessage {
  type Error = String;

  fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
    const MSG_TAG_START: u16 = 0;
    const MSG_TAG_PKTRQ: u16 = 1;
    const MSG_TAG_END: u16 = 2;
    let (tag, data) = value.split_at(2);
    if tag.starts_with(&MSG_TAG_START.to_le_bytes())
      && value.len() == MSG_TAG_START.to_le_bytes().len() + 4 + 4
    {
      Ok(Self::Start(
        IntegralModId(consume_to_u32(data, 0)?),
        IntegralFnId(consume_to_u32(data, 4)?),
      ))
    } else if tag.starts_with(&MSG_TAG_PKTRQ.to_le_bytes()) {
      let sized: [u8; 8] = data
        .try_into()
        .map_err(|e: std::array::TryFromSliceError| e.to_string())?;
      Ok(Self::PacketRequest(u64::from_le_bytes(sized)))
    } else if tag.starts_with(&MSG_TAG_END.to_le_bytes()) {
      Ok(Self::End(TestStatus::try_from(data)?))
    } else {
      Err(format!("Invalid msg format: {:?}", value))
    }
  }
}

#[derive(Debug)]
enum ClientState {
  Init,
  Started(IntegralModId, IntegralFnId),
  Ended(IntegralModId, IntegralFnId, TestStatus),
}

async fn single_test_job(
  stream: UnixStream,
  packet_dir: PathBuf,
  modules: Arc<ExtModuleMap>,
  mem_limit: usize,
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
    lg.info("Restart");
    let mut data = [0u8; 10];
    match timeout(Duration::from_millis(500), buff_stream.read(&mut data)).await {
      Ok(Ok(0)) => {
        lg.info("Client closed connection - ending");
        return;
      }
      Ok(Ok(n)) => {
        if n != 10 {
          lg.info(format!("Expected {} bytes, got {n} - ending", data.len()));
          return;
        }
      }
      Ok(Err(e)) => {
        lg.info(format!("Not readable {e} - ending"));
        return;
      }
      Err(_) => continue,
    }

    lg.info(format!("Read done: {:?}", data));
    let msg = match TestMessage::try_from(data.as_slice()) {
      Ok(msg) => msg,
      Err(e) => {
        lg.crit(e);
        return;
      }
    };

    let (new_state, response) = match handle_client_msg(state, msg, &mut packets) {
      Ok((s, r)) => (s, r),
      Err(e) => {
        lg.crit(e);
        return;
      }
    };
    state = new_state;
    lg = Log::get(&format!("single_test_job@{:?}", state));
    lg.info(format!("Response: {:?}", response));
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
) -> Result<(ClientState, Option<Vec<u8>>), String> {
  match state {
    ClientState::Init => match msg {
      TestMessage::Start(m, f) => Ok((ClientState::Started(m, f), None)),
      _ => Err(format!(
        "Invalid transition from Init state with msg {:?}",
        msg
      )),
    },
    ClientState::Started(mod_id, fn_id) => match msg {
      TestMessage::End(ts) => Ok((ClientState::Ended(mod_id, fn_id, ts), None)),
      TestMessage::PacketRequest(idx) => {
        let response = packets.get_packet(mod_id, fn_id, idx as usize);
        Ok((ClientState::Started(mod_id, fn_id), response))
      }
      _ => Err(format!(
        "Invalid transition from Started state with msg {:?}",
        msg
      )),
    },
    ClientState::Ended(m, f, st) => Err(format!(
      "Client state is Ended, no more messages were expected for mod/fun {} {}, status: {:?}, msg: {:?}",
      m.hex_string(),
      f.hex_string(),
      st,
      msg
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
