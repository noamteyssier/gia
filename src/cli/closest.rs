use clap::Parser;

use super::{outputs::Output, DualInput};

/// Finds the closest interval in a secondary BED file for all intervals in a primary BED file
#[derive(Parser, Debug)]
pub struct ClosestArgs {
    #[clap(flatten)]
    pub inputs: DualInput,

    #[clap(flatten)]
    pub output: Output,

    #[clap(flatten)]
    pub params: ClosestParams,
}

#[derive(Parser, Debug, Clone, Copy)]
pub struct ClosestParams {
    /// Report only the closest upstream interval
    #[clap(short = 'u', long, conflicts_with = "downstream")]
    pub upstream: bool,

    /// Report only the closest downstream interval
    #[clap(short = 'd', long, conflicts_with = "upstream")]
    pub downstream: bool,

    /// Specify that the input files are already presorted
    #[clap(short, long)]
    pub sorted: bool,
}
