use super::{DualInput, Output, OverlapPredicates};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct SubtractArgs {
    #[clap(flatten)]
    pub inputs: DualInput,
    #[clap(flatten)]
    pub output: Output,
    #[clap(flatten)]
    pub params: SubtractParams,
}

#[derive(Parser, Debug, Clone, Copy)]
pub struct SubtractParams {
    #[clap(flatten)]
    pub overlap_predicates: OverlapPredicates,

    /// Keep the query records unmerged (i.e. report all subtractions)
    ///
    /// By default, the query records are merged to remove overlapping
    /// regions.
    #[clap(short, long)]
    pub unmerged: bool,
}
