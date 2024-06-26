use super::{Growth, Output, SingleInput};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct FlankArgs {
    #[clap(flatten)]
    pub input: SingleInput,

    #[clap(flatten)]
    pub growth: Growth,

    #[clap(flatten)]
    pub output: Output,
}
