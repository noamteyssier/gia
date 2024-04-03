use super::{Output, SingleInput};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct SegmentArgs {
    #[clap(flatten)]
    pub input: SingleInput,
    #[clap(flatten)]
    pub output: Output,
    #[clap(flatten)]
    pub params: SegmentParams,
}

#[derive(Parser, Debug)]
pub struct SegmentParams {
    /// Assume input is sorted (default=false)
    #[clap(short, long)]
    pub sorted: bool,
}
