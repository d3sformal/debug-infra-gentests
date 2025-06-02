use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::constants::Constants;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
  /// Sets path to module map directory as produced by the instrumentation
  #[arg(short, long)]
  pub modmap: PathBuf,

  /// The stage to perform
  #[command(subcommand)]
  pub stage: Stage,

  /// File descriptor prefixes for resources
  #[arg(short = 'p', long, default_value = Constants::default_fd_prefix())]
  pub fd_prefix: String,

  /// Buffer count
  #[arg(short = 'c', long, default_value = Constants::default_buff_count_str())]
  pub buff_count: u32,

  /// Buffer size in bytes
  #[arg(short = 's', long, default_value = Constants::default_buff_size_bytes_str())]
  pub buff_size: u32,

  /// Enable verbose output, write up to 3x
  #[arg(short, long, action = clap::ArgAction::Count)]
  pub verbose: u8,

  /// Perform the full tool iteration starting from the selected stage
  #[arg(short, long)]
  pub full: bool,

  /// Produce all artifacts that can be exported (in default locations)
  /// This option overrides ALL paths specified as import or export paths for ANY stage  
  #[arg(short, long)]
  pub all_artifacts: bool,

  /// Perform a cleanup of all possibly leftover resources related to the given stage and exit
  #[arg(long)]
  pub cleanup: bool,
}

#[derive(Subcommand)]
pub enum Stage {
  /// Set up a function tracing server
  TraceCalls {
    /// produce an export file
    #[arg(short, long, default_value = Constants::default_trace_out_path())]
    out_file: Option<PathBuf>,

    /// imports an export file, export file will not be created
    #[arg(short, long)]
    import_path: Option<PathBuf>,
  },

  CaptureArgs {
    // path to the function selection file (generated in the call-tracing phase)
    #[arg(short, long, default_value = Constants::default_selected_functions_path())]
    selection_file: PathBuf,

    /// the directory where function argument traces are saved (or offloaded)
    #[arg(short, long, default_value = Constants::default_capture_out_path())]
    out_dir: PathBuf,

    /// capture memory limit in MEBIBYTES - offloading will be performed to the output directory
    #[arg(short = 'l', long, default_value = "0")]
    mem_limit: u32,

    /// Command to execute tested binary
    #[arg(trailing_var_arg(true))]
    command: Vec<String>,
  },

  Test {
    // path to the function selection file (generated in the call-tracing phase)
    #[arg(short, long, default_value = Constants::default_selected_functions_path())]
    selection_file: PathBuf,

    /// the directory where function argument traces have been saved (generated in the arg-capturing phase)
    #[arg(short, long, default_value = Constants::default_capture_out_path())]
    capture_dir: PathBuf,

    /// capture read memory limit in MEBIBYTES
    #[arg(short = 'l', long, default_value = "0")]
    mem_limit: u32,

    /// Redirects output to a file or a directory instead of the standard output (target directory must exist beforehand otherwise the path is treated as a file path)
    #[arg(short, long)]
    test_output: Option<PathBuf>,

    /// Command to execute tested binary
    #[arg(trailing_var_arg(true))]
    command: Vec<String>,
  },
}
