use std::collections::HashMap;
use std::fs;

use args::Cli;
use call_tracing::{extract_message, FunctionCallInfo, Message};
use clap::Parser;
use constants::Constants;
use log::Log;
use modmap::ExtModuleMap;
use zeromq::{PullSocket, Socket, ZmqError};
mod args;
mod constants;
mod log;
mod modmap;
mod call_tracing;

pub fn print_summary(freqs: &HashMap<FunctionCallInfo, u64>, mods: &ExtModuleMap) {
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
    println!("Total traced calls: {}", freqs.values().sum::<u64>());
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
            lg.trace("Got message");
            match m {
                Message::ControlEnd => {
                    lg.trace("Stopping on EndMessage");
                    break;
                },
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
    print_summary(&recorded_frequencies, &modules);
    return Ok(());
}
