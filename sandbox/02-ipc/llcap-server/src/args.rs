use clap::Parser;
use std::path::PathBuf;

use crate::constants::Constants;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Sockte address to operate on
    #[arg(short, long, default_value = Constants::default_socket_address())]
    pub socket: String,

    /// Sets path to module map directory as produced by the instrumentation
    #[arg(short, long, value_name = "FILE")]
    pub modmap: PathBuf,

    /// Enable verbose output, write up to 3x
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}
