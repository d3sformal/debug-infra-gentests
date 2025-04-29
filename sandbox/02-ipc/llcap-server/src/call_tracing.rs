use bytes::Buf;
use zeromq::{PullSocket, SocketRecv};

use crate::{log::Log, modmap::ExtModuleMap};

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct FunctionCallInfo {
  pub function_id: u32,
  pub module_id: usize,
}

impl FunctionCallInfo {
  pub fn from_two_messages(
    id_bytes: &bytes::Bytes,
    module_bytes: &bytes::Bytes,
    modules: &ExtModuleMap,
  ) -> Option<Self> {
    let fn_id: Result<u32, bytes::TryGetError> = id_bytes.clone().try_get_u32_le();
    let sha256hash = String::from_utf8(module_bytes.to_vec());
    if let (Ok(fn_id), Ok(sha256hash)) = (fn_id, sha256hash) {
      return modules
        .get_module_id(&sha256hash)
        .map(|id| FunctionCallInfo {
          function_id: fn_id,
          module_id: id,
        });
    }
    Log::get("from_two_messages").crit("Invalid data received!");
    None
  }

  pub fn new(fn_id: u32, mod_id: usize) -> Self {
    Self {
      function_id: fn_id,
      module_id: mod_id
    }
  }
}

pub enum Message {
  Normal(FunctionCallInfo),
  ControlEnd,
}

pub async fn extract_message(socket: &mut PullSocket, modules: &ExtModuleMap) -> Option<Message> {
  let lg = Log::get("extract_message");
  let message = socket.recv().await;
  if message.is_err() {
    let _ = message.inspect_err(|e| lg.crit(&format!("Error receiving: {}", e)));
    return None;
  }
  let message = message.unwrap();
  if message.is_empty() {
    return None;
  }

  if message.len() == 1 {
    return Some(Message::ControlEnd);
  }

  let msg1 = message.get(0).unwrap();
  let msg2 = message.get(1).unwrap();

  let (shorter, longer) = if msg1.len() > msg2.len() {
    (msg2, msg1)
  } else {
    (msg1, msg2)
  };
  if shorter.len() != 4 || longer.len() != 64 {
    lg.warn(&format!("Invalid sizes of {:?} and {:?}", shorter, longer));
  }

  let call_info = FunctionCallInfo::from_two_messages(shorter, longer, modules);

  call_info.and_then(|m| Message::Normal(m).into())
}
