use zeromq::{RepSocket, Socket, SocketRecv, SocketSend, ZmqMessage};

use crate::{
  log::Log,
  modmap::{IntegralFnId, IntegralModId},
};

use super::hooklib_commons::ShmMeta;

pub fn zmq_metadata_chnl_name(prefix: &str) -> String {
  format!("{prefix}-zmqmain-meta")
}

pub fn zmq_packet_chnl_name(prefix: &str) -> String {
  format!("{}-zmqpackets", prefix)
}

pub fn to_zmq_absolute_path(channel_name: &str) -> String {
  format!("/tmp{channel_name}")
}

pub struct ZmqMetadataServer {
  rep_sock: RepSocket,
}

impl ZmqMetadataServer {
  pub async fn new(data_chnl_name: &str) -> Result<Self, String> {
    let mut rep_sock = RepSocket::new();

    let _ = rep_sock
      .bind(&format!("ipc://{}", to_zmq_absolute_path(data_chnl_name)))
      .await
      .map_err(|e| format!("rep_sock failed to bind {data_chnl_name}: {e}"))?;

    Ok(Self { rep_sock })
  }

  pub async fn send_metadata(&mut self, data: ShmMeta) -> Result<(), String> {
    let lg = Log::get("send meta");
    lg.trace("Recv");
    self
      .rep_sock
      .recv()
      .await
      .map_err(|e| format!("Failed to recv request: {e}"))?;

    let mut buff = [0u8; std::mem::size_of::<ShmMeta>()];
    assert_eq!(std::mem::size_of_val(&buff), std::mem::size_of_val(&data));
    buff = unsafe { std::mem::transmute_copy(&data) };
    let msg = ZmqMessage::from(buff.to_vec());

    lg.trace("Send");
    self
      .rep_sock
      .send(msg)
      .await
      .map_err(|e| format!("Failed to push metadata: {e}"))?;
    Ok(())
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

pub struct ZmqArgumentPacketServer {
  rep_sock: RepSocket,
}

impl ZmqArgumentPacketServer {
  pub async fn new(socket_name: &str) -> Result<Self, String> {
    let mut rep_sock = RepSocket::new();

    let _ = rep_sock
      .bind(&format!("ipc://{}", to_zmq_absolute_path(socket_name)))
      .await
      .map_err(|e| format!("rep_sock failed to bind {socket_name}: {e}"))?;

    Ok(Self { rep_sock })
  }

  pub async fn process_msg(&mut self, data: Option<Vec<u8>>) -> Result<bool, String> {
    let req = self
      .rep_sock
      .recv()
      .await
      .map_err(|e| format!("Failed to recv request: {e}"))?;
    let req_payload = req.get(0);
    if req_payload.is_none() || req_payload.unwrap().len() < 2 {
      return Err("Invalid length of request ID".to_string());
    }
    let req_payload = req_payload.unwrap();

    let le_bytes = [*req_payload.first().unwrap(), *req_payload.get(1).unwrap()];
    let id = u16::from_le_bytes(le_bytes);

    const PACKET: u16 = 1;
    if id == PACKET {
      let mod_id = IntegralModId(consume_to_u32(req_payload, 2)?);
      let fn_id = IntegralFnId(consume_to_u32(req_payload, 6)?);
      Log::get("expect_send_packet").info(format!(
        "Got {} {}",
        mod_id.hex_string(),
        fn_id.hex_string()
      ));
      let msg = ZmqMessage::from(data.unwrap());
      self
        .rep_sock
        .send(msg)
        .await
        .map_err(|e| format!("Failed to push packet: {e}"))?;
      Ok(true)
    } else {
      Err(format!("Unknown request ID: {} {:?}", id, req_payload))
    }
  }
}
