use super::{DualInput, Output, OverlapPredicates};
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
pub struct JoinArgs {
    #[clap(flatten)]
    pub inputs: DualInput,

    #[clap(flatten)]
    pub params: JoinParams,

    #[clap(flatten)]
    pub output: Output,
}

#[derive(Parser, Debug)]
#[clap(next_help_heading = "Parameters")]
pub struct JoinParams {
    #[clap(short = 'H', long, default_value = "inner")]
    pub how: JoinMethod,

    /// Assert the inputs are pre-sorted
    #[clap(short = 'S', long)]
    pub sorted: bool,

    #[clap(flatten)]
    pub overlap_predicates: OverlapPredicates,
}

#[derive(Parser, Debug, ValueEnum, Clone, Copy)]
pub enum JoinMethod {
    Left,
    Right,
    Inner,
    Outer,
}
