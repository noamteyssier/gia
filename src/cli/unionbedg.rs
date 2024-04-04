use super::{MultiInput, Output};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct UnionBedGraphArgs {
    #[clap(flatten)]
    pub inputs: MultiInput,
    #[clap(flatten)]
    pub params: UnionBedGraphParams,
    #[clap(flatten)]
    pub output: Output,
}

#[derive(Parser, Debug)]
#[clap(next_help_heading = "Parameters")]
pub struct UnionBedGraphParams {
    /// Assume *ALL* input is sorted (default=false)
    #[clap(short, long)]
    pub sorted: bool,
}
