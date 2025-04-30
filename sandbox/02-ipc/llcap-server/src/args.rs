use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::constants::Constants;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
  /// Sets path to module map directory as produced by the instrumentation
  #[arg(short, long, value_name = "FILE")]
  pub modmap: PathBuf,

  #[command(subcommand)]
  pub method: Type,

  /// Enable verbose output, write up to 3x
  #[arg(short, long, action = clap::ArgAction::Count)]
  pub verbose: u8,
}

#[derive(Subcommand)]
pub enum Type {
  /// Use (failed experimental) ZeroMQ capture
  ZeroMQ {
    /// Socket address to operate on
    #[arg(short, long, default_value = Constants::default_socket_address())]
    socket: String,
  },

  Shmem {
    /// File descriptor prefixes for resources
    #[arg(short, long, default_value = Constants::default_fd_prefix())]
    fd_prefix: String,

    /// Buffer count
    #[arg(short = 'c', long, default_value = Constants::default_buff_count_str())]
    buff_count: u32,

    /// Buffer size in bytes
    #[arg(short = 's', long, default_value = Constants::default_buff_size_bytes_str())]
    buff_size: u32,

    /// Perform semaphore cleanup
    #[arg(long)]
    cleanup: bool,
  },
}
