use super::{Output, SingleInput};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct MergeArgs {
    #[clap(flatten)]
    pub input: SingleInput,

    #[clap(flatten)]
    pub params: MergeParams,

    #[clap(flatten)]
    pub output: Output,
}

#[derive(Parser, Debug)]
#[clap(next_help_heading = "Parameters")]
pub struct MergeParams {
    /// Assume input is sorted (default=false)
    #[clap(short, long)]
    pub sorted: bool,

    /// Stream the input file instead of loading it into memory
    ///
    /// Note that this requires the input file to be sorted
    /// and will result in undefined behavior if it is not.
    ///
    /// Currently does not support non-integer chromosome names.
    #[clap(short = 'S', long)]
    pub stream: bool,
}
