use super::{outputs::Output, overlap_predicates::WrapStrandedness, DualInput};
use clap::Parser;

/// Finds the closest interval in a secondary BED file for all intervals in a primary BED file
#[derive(Parser, Debug)]
#[clap(next_help_heading = "Global options")]
pub struct ClosestArgs {
    #[clap(flatten)]
    pub inputs: DualInput,

    #[clap(flatten)]
    pub params: ClosestParams,

    #[clap(flatten)]
    pub output: Output,
}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(next_help_heading = "Parameters")]
pub struct ClosestParams {
    /// Report only the closest upstream interval
    #[clap(short = 'u', long, conflicts_with = "downstream")]
    pub upstream: bool,

    /// Report only the closest downstream interval
    #[clap(short = 'd', long, conflicts_with = "upstream")]
    pub downstream: bool,

    /// Strand-specificity of closest intervals
    #[clap(short, long, default_value = "i")]
    pub strandedness: WrapStrandedness,

    /// Specify that the input files are already presorted
    #[clap(short = 'S', long)]
    pub sorted: bool,
}
