use super::{Output, SingleInput};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct ClusterArgs {
    #[clap(flatten)]
    pub input: SingleInput,

    #[clap(flatten)]
    pub output: Output,

    #[clap(flatten)]
    pub params: ClusterParams,
}

#[derive(Parser, Debug)]
pub struct ClusterParams {
    /// Assume input is sorted (default=false)
    #[clap(short, long)]
    pub sorted: bool,
}
