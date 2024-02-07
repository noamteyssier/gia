mod closest;
mod commands;
mod complement;
mod inputs;
mod outputs;
pub use closest::ClosestArgs;
pub use commands::Command;
pub use complement::ComplementArgs;
pub use inputs::{DualInput, SingleInput};

use clap::Parser;

#[derive(Parser)]
#[clap(version)]
pub struct Cli {
    /// Subcommand to run
    #[clap(subcommand)]
    pub command: Command,

    /// Compression threads to use for output files if applicable
    #[clap(global = true, short = 'j', long, default_value = "1")]
    pub compression_threads: usize,

    /// Compression level to use for output files if applicable
    #[clap(global = true, long, default_value = "6")]
    pub compression_level: u32,
}
