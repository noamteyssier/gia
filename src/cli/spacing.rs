use super::{Output, SingleInput};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct SpacingArgs {
    #[clap(flatten)]
    pub inputs: SingleInput,
    #[clap(flatten)]
    pub output: Output,
    #[clap(flatten)]
    pub params: SpacingParams,
}

#[derive(Parser, Debug)]
pub struct SpacingParams {
    #[clap(short = 's', long)]
    pub is_sorted: bool,
}
