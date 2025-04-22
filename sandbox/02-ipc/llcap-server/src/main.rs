use std::collections::HashMap;
use std::fs;

use bytes::Buf;
use zeromq::{PullSocket, Socket, SocketRecv, ZmqError};

#[derive(Debug)]
struct ModuleRegistry {
    module_map: HashMap<String, u32>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        ModuleRegistry {
            module_map: HashMap::new(),
        }
    }

    pub fn get_module_id(&mut self, key: String) -> u32 {
        let len = self.module_map.len();
        *self.module_map.entry(key).or_insert(len as u32)
    }
}

#[derive(Hash, PartialEq, Eq, Debug)]
struct FunctionCallInfo {
    function_id: u32,
    module_index: u32,
}

impl FunctionCallInfo {
    pub fn from_two_messages(
        id_bytes: &bytes::Bytes,
        module_bytes: &bytes::Bytes,
        registry: &mut ModuleRegistry,
    ) -> Option<Self> {
        let fn_id: Result<u32, bytes::TryGetError> = id_bytes.clone().try_get_u32_le();
        let sha256hash = String::from_utf8(module_bytes.to_vec());
        if let (Ok(fn_id), Ok(sha256hash)) = (fn_id, sha256hash) {
            return Some(FunctionCallInfo {
                function_id: fn_id,
                module_index: registry.get_module_id(sha256hash),
            });
        }
        None
    }
}

enum Message {
    Normal(FunctionCallInfo),
    ControlEnd,
}

async fn extract_message(
    socket: &mut PullSocket,
    registry: &mut ModuleRegistry,
) -> Option<Message> {
    let message = socket.recv().await;
    if message.is_err() {
        let _ = message.inspect_err(|e| println!("Error receiving: {}", e));
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
        println!("huh {:?} {:?}", shorter, longer);
    }

    let call_info = FunctionCallInfo::from_two_messages(shorter, longer, registry);
    
    call_info.and_then(|m| Message::Normal(m).into())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), ZmqError> {
    let mut socket = PullSocket::new();
    let socket_path = "/tmp/zmq-socket";
    let addr = &format!("ipc://{}", socket_path);

    let mut registry = ModuleRegistry::new();
    let mut recorded_frequencies: HashMap<FunctionCallInfo, u64> = HashMap::new();

    if socket.bind(addr).await.is_err() {
        let _ = fs::remove_file(socket_path);
        socket.bind(addr).await.expect("Failed to connect");
    }

    loop {
        if let Some(m) = extract_message(&mut socket, &mut registry).await {
            match m {
                Message::ControlEnd => break,
                Message::Normal(content) => recorded_frequencies
                    .entry(content)
                    .and_modify(|v| *v += 1)
                    .or_insert(1),
            };
        }
    }

    println!("{:?} {:?}", registry, recorded_frequencies);

    return Ok(());
}
