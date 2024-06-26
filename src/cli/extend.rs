use clap::Parser;

use super::{Growth, Output, SingleInput};

#[derive(Parser, Debug)]
pub struct ExtendArgs {
    #[clap(flatten)]
    pub input: SingleInput,

    #[clap(flatten)]
    pub growth: Growth,

    #[clap(flatten)]
    pub output: Output,
}
