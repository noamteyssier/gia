use super::{DualInput, Output, OverlapPredicates};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct CoverageArgs {
    #[clap(flatten)]
    pub inputs: DualInput,

    #[clap(flatten)]
    pub params: CoverageParams,

    #[clap(flatten)]
    pub output: Output,
}

#[derive(Parser, Debug)]
#[clap(next_help_heading = "Parameters")]
pub struct CoverageParams {
    /// Assert that the intervals are presorted in BOTH files (unexpected behavior if they are
    /// not)
    #[clap(short = 'S', long)]
    pub sorted: bool,

    /// Number of threads to use for parallel processing of intervals
    #[clap(short = '@', long)]
    pub threads: Option<usize>,

    /// Batch size used when writing to the output in parallel
    #[clap(short = 'C', long, default_value = "5000", requires("threads"))]
    pub chunk_size: usize,

    #[clap(flatten)]
    pub overlap_predicates: OverlapPredicates,
}
