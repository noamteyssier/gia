use super::{DualInput, Output, OverlapPredicates};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct CoverageArgs {
    #[clap(flatten)]
    pub inputs: DualInput,

    #[clap(flatten)]
    pub output: Output,

    #[clap(flatten)]
    pub overlap_predicates: OverlapPredicates,

    /// Assert that the intervals are presorted in BOTH files (unexpected behavior if they are
    /// not)
    #[clap(short, long)]
    pub sorted: bool,
}
