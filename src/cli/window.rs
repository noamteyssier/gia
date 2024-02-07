use super::{DualInput, Growth, Output};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct WindowArgs {
    #[clap(flatten)]
    pub inputs: DualInput,

    #[clap(flatten)]
    pub output: Output,

    #[clap(flatten)]
    pub params: WindowParams,
}

#[derive(Parser, Debug)]
pub struct WindowParams {
    #[clap(flatten)]
    pub growth: Growth,

    /// Only report the intervals in the query that do not overlap with the target
    /// (i.e. the inverse of the intersection)
    #[clap(short = 'v', long)]
    pub inverse: bool,
}
