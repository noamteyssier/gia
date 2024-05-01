use super::{DualInput, Output, OverlapPredicates};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct SubtractArgs {
    #[clap(flatten)]
    pub inputs: DualInput,
    #[clap(flatten)]
    pub params: SubtractParams,
    #[clap(flatten)]
    pub output: Output,
}

#[derive(Parser, Debug, Clone, Copy)]
#[clap(next_help_heading = "Parameters")]
pub struct SubtractParams {
    /// Keep the query records unmerged (i.e. report all subtractions)
    ///
    /// By default, the query records are merged to remove overlapping
    /// regions.
    #[clap(short, long)]
    pub unmerged: bool,
    #[clap(flatten)]
    pub overlap_predicates: OverlapPredicates,
}
