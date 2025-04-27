use std::collections::HashMap;
use std::fs;

use crate::call_tracing::{FunctionCallInfo, Message, extract_message};
use crate::constants::Constants;
use crate::log::Log;
use crate::modmap::ExtModuleMap;
use zeromq::{PullSocket, Socket};

pub async fn zmq_call_trace(
  addr: &str,
  modules: &ExtModuleMap,
  recorded_frequencies: &mut HashMap<FunctionCallInfo, u64>,
) {
  let lg = Log::get("zmq_call_trace");
  let mut socket = PullSocket::new();

  if socket.bind(addr).await.is_err() {
    if addr == Constants::default_socket_address() {
      let _ = fs::remove_file(Constants::default_socket_path());
      socket.bind(addr).await.expect("Failed to connect");
    } else {
      lg.crit("Cannot connect");
      return;
    }
  }
  lg.info("Listening...");

  loop {
    if let Some(m) = extract_message(&mut socket, modules).await {
      lg.trace("Got message");
      match m {
        Message::ControlEnd => {
          lg.trace("Stopping on EndMessage");
          break;
        }
        Message::Normal(content) => recorded_frequencies
          .entry(content)
          .and_modify(|v| *v += 1)
          .or_insert(1),
      };
    } else {
      lg.crit("Could not extract message!");
    }
  }

  lg.info("Function Frequencies:");
}
