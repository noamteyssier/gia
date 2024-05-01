use super::{Output, SingleInput};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct SegmentArgs {
    #[clap(flatten)]
    pub input: SingleInput,
    #[clap(flatten)]
    pub params: SegmentParams,
    #[clap(flatten)]
    pub output: Output,
}

#[derive(Parser, Debug)]
#[clap(next_help_heading = "Parameters")]
pub struct SegmentParams {
    /// Assume input is sorted (default=false)
    #[clap(short, long)]
    pub sorted: bool,
}
