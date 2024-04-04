use crate::cli::{BamOutput, MixedInput, OverlapPredicates};

use clap::Parser;

#[derive(Parser, Debug)]
pub struct FilterArgs {
    #[clap(flatten)]
    pub inputs: MixedInput,

    #[clap(flatten)]
    pub params: FilterParams,

    #[clap(flatten)]
    pub output: BamOutput,
}

#[derive(Parser, Debug)]
#[clap(next_help_heading = "Parameters")]
pub struct FilterParams {
    #[clap(flatten)]
    pub overlap_predicates: OverlapPredicates,
}
