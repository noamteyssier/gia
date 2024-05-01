use super::{Output, SingleInput};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct ClusterArgs {
    #[clap(flatten)]
    pub input: SingleInput,

    #[clap(flatten)]
    pub params: ClusterParams,

    #[clap(flatten)]
    pub output: Output,
}

#[derive(Parser, Debug)]
#[clap(next_help_heading = "Parameters")]
pub struct ClusterParams {
    /// Assume input is sorted (default=false)
    #[clap(short, long)]
    pub sorted: bool,
}
