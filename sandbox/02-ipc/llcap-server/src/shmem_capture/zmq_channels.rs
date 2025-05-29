use zeromq::{PullSocket, PushSocket, RepSocket, Socket, SocketRecv, SocketSend, ZmqMessage};

use crate::{
  log::Log,
  modmap::{IntegralFnId, IntegralModId},
  stages::arg_capture::PacketReader,
};

use super::hooklib_commons::ShmMeta;

pub struct MainChannelNames {
  pub meta: String,
  pub ack: String,
}

impl MainChannelNames {
  pub fn get_metadata(prefix: &str) -> String {
    format!("{prefix}-zmqmain-meta")
  }
}

pub struct ZmqMetadataServer {
  rep_sock: RepSocket,
}

impl ZmqMetadataServer {
  pub async fn new(data_chnl_name: &str) -> Result<Self, String> {
    let mut rep_sock = RepSocket::new();

    let _ = rep_sock
      .bind(&format!("ipc:///tmp{data_chnl_name}"))
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

pub struct ZmqArgumentPacketServer {
  rep_sock: RepSocket,
  reader: PacketReader,
}

impl ZmqArgumentPacketServer {
  pub async fn new(socket_name: &str, reader: PacketReader) -> Result<Self, String> {
    let mut rep_sock = RepSocket::new();

    let _ = rep_sock
      .bind(&format!("ipc:///tmp{socket_name}"))
      .await
      .map_err(|e| format!("rep_sock failed to bind {socket_name}: {e}"))?;

    Ok(Self { rep_sock, reader })
  }

  pub async fn process_message(&mut self) -> Result<bool, String> {
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
    const END: u16 = 2;
    if id == PACKET || req_payload.len() != 10 {
      let le_bytes = [
        *req_payload.get(2).unwrap(),
        *req_payload.get(3).unwrap(),
        *req_payload.get(4).unwrap(),
        *req_payload.get(5).unwrap(),
      ];
      let mod_id = IntegralModId(u32::from_le_bytes(le_bytes));
      let le_bytes = [
        *req_payload.get(6).unwrap(),
        *req_payload.get(7).unwrap(),
        *req_payload.get(8).unwrap(),
        *req_payload.get(9).unwrap(),
      ];
      let fn_id = IntegralFnId(u32::from_le_bytes(le_bytes));

      let res = self.reader.read_next_packet(mod_id, fn_id)?;
      if let Some(res) = res {
        let msg = ZmqMessage::from(res);
        self
          .rep_sock
          .send(msg)
          .await
          .map_err(|e| format!("Failed to push packet: {e}"))?;
        Ok(true)
      } else {
        Err("Invalid state, client expects more packets than available".to_string())
      }
    } else if id == END && req_payload.len() == 2 {
      Ok(false)
    } else {
      Err(format!("Unknown request ID: {} {:?}", id, req_payload))
    }
  }
}
