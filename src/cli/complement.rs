use clap::Parser;

use super::{outputs::Output, SingleInput};

#[derive(Parser, Debug)]
pub struct ComplementArgs {
    #[clap(flatten)]
    pub input: SingleInput,

    #[clap(flatten)]
    pub params: ComplementParams,

    #[clap(flatten)]
    pub output: Output,
}

#[derive(Parser, Debug, Clone)]
#[clap(next_help_heading = "Parameters")]
pub struct ComplementParams {
    /// Assume input is sorted (default=false)
    #[clap(short, long)]
    pub sorted: bool,

    /// Stream the input file instead of loading it into memory
    ///
    /// Note that this requires the input file to be sorted
    /// and will result in undefined behavior if it is not.
    #[clap(short = 'S', long)]
    pub stream: bool,
}
