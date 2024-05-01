use super::{Output, SingleInput};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct SpacingArgs {
    #[clap(flatten)]
    pub inputs: SingleInput,
    #[clap(flatten)]
    pub params: SpacingParams,
    #[clap(flatten)]
    pub output: Output,
}

#[derive(Parser, Debug)]
#[clap(next_help_heading = "Parameters")]
pub struct SpacingParams {
    #[clap(short = 's', long)]
    pub is_sorted: bool,
}
