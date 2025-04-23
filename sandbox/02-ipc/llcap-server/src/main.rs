use std::collections::HashMap;
use std::fs;

use args::Cli;
use bytes::Buf;
use clap::Parser;
use constants::Constants;
use log::Log;
use modmap::ExtModuleMap;
use zeromq::{PullSocket, Socket, SocketRecv, ZmqError};
mod args;
mod constants;
mod log;
mod modmap;

#[derive(Hash, PartialEq, Eq, Debug)]
struct FunctionCallInfo {
    function_id: u32,
    module_id: usize,
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
        None
    }
}

enum Message {
    Normal(FunctionCallInfo),
    ControlEnd,
}

async fn extract_message(socket: &mut PullSocket, modules: &ExtModuleMap) -> Option<Message> {
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

    let call_info = FunctionCallInfo::from_two_messages(shorter, longer, modules);

    call_info.and_then(|m| Message::Normal(m).into())
}

fn print_summary(freqs: &HashMap<FunctionCallInfo, u64>, mods: &ExtModuleMap) {
    let mut pairs = freqs.iter().collect::<Vec<(_, _)>>();
    pairs.sort_by(|a, b| b.1.cmp(a.1));
    for (idx, (fninfo, freq)) in pairs.iter().enumerate() {
        let modstr = mods.get_module_string_id(fninfo.module_id);
        let fn_name = mods.get_function_name(fninfo.module_id, fninfo.function_id);

        if modstr.and(fn_name).is_none() {
            eprintln!(
                "Warn: function id or module id confusion with fnid: {} moid: {}",
                fninfo.function_id, fninfo.module_id
            );
            continue;
        }

        println!(
            "{idx} - {freq} - {} (module {})",
            fn_name.unwrap(),
            modstr.unwrap()
        );
    }
    mods.print_summary();
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), ZmqError> {
    let cli = Cli::try_parse();
    if let Err(e) = cli {
        eprintln!("{}", e);
        return Err(ZmqError::NoMessage);
    }
    let cli = cli.unwrap();
    let lg = Log::get(cli.verbose);
    lg.info(&format!("Verbosity: {}", cli.verbose));

    let modules = ExtModuleMap::try_from(cli.modmap.clone());
    if modules.is_err() {
        lg.crit(&format!(
            "Could not parse module mapping from {}:\n{}",
            cli.modmap.to_string_lossy(),
            modules.unwrap_err()
        ));
        return Err(ZmqError::NoMessage);
    }

    let modules = modules.unwrap();
    let mut socket = PullSocket::new();
    let addr = &cli.socket;

    let mut recorded_frequencies: HashMap<FunctionCallInfo, u64> = HashMap::new();

    if socket.bind(addr).await.is_err() {
        if cli.socket == Constants::default_socket_address() {
            let _ = fs::remove_file(Constants::default_socket_path());
            socket.bind(addr).await.expect("Failed to connect");
        } else {
            return Err(ZmqError::Other("Cannot connect"));
        }
    }

    lg.info("Listening...");

    loop {
        if let Some(m) = extract_message(&mut socket, &modules).await {
            match m {
                Message::ControlEnd => break,
                Message::Normal(content) => recorded_frequencies
                    .entry(content)
                    .and_modify(|v| *v += 1)
                    .or_insert(1),
            };
        }
    }

    lg.info("Function Frequencies:");
    print_summary(&recorded_frequencies, &modules);
    return Ok(());
}
