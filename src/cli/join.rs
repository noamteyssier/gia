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
    /// Return all records in the left input even if no match is found in right
    Left,
    /// Return all records in the right input even if no match is found in left
    Right,
    /// Return only records that have a match in both inputs
    Inner,
}
