use crate::cli::{outputs::VcfOutput, MixedInputVcf, OverlapPredicates};

use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct FilterArgs {
    #[clap(flatten)]
    pub inputs: MixedInputVcf,

    #[clap(flatten)]
    pub params: FilterParams,

    #[clap(flatten)]
    pub output: VcfOutput,
}

#[derive(Parser, Debug, Clone)]
#[clap(next_help_heading = "Parameters")]
pub struct FilterParams {
    #[clap(flatten)]
    pub overlap_predicates: OverlapPredicates,

    #[clap(flatten)]
    pub output_predicates: OutputPredicates,
}

#[derive(Parser, Debug, Clone)]
#[clap(next_help_heading = "Output Predicates")]
pub struct OutputPredicates {
    /// Only return the records from a that DON'T overlap with b
    #[clap(short = 'v', long)]
    pub invert: bool,
}
