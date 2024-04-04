use super::{DualInput, Growth, Output};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct WindowArgs {
    #[clap(flatten)]
    pub inputs: DualInput,

    #[clap(flatten)]
    pub params: WindowParams,

    #[clap(flatten)]
    pub output: Output,
}

#[derive(Parser, Debug)]
#[clap(next_help_heading = "Parameters")]
pub struct WindowParams {
    /// Only report the intervals in the query that do not overlap with the target
    /// (i.e. the inverse of the intersection)
    #[clap(short = 'v', long)]
    pub inverse: bool,
    #[clap(flatten)]
    pub growth: Growth,
}
