mod closest;
mod commands;
mod complement;
mod coverage;
mod extend;
mod flank;
mod get_fasta;
mod growth;
mod inputs;
mod intersect;
mod merge;
mod outputs;
mod overlap_predicates;
pub use closest::ClosestArgs;
pub use commands::Command;
pub use complement::ComplementArgs;
pub use coverage::CoverageArgs;
pub use extend::ExtendArgs;
pub use flank::FlankArgs;
pub use get_fasta::GetFastaArgs;
pub use growth::Growth;
pub use inputs::{DualInput, SingleInput};
pub use intersect::{IntersectArgs, OutputMethod, OutputPredicates};
pub use merge::MergeArgs;
pub use outputs::Output;
pub use overlap_predicates::OverlapPredicates;

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
