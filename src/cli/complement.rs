use clap::Parser;

use super::{outputs::Output, SingleInput};

#[derive(Parser, Debug)]
pub struct ComplementArgs {
    #[clap(flatten)]
    pub input: SingleInput,

    #[clap(flatten)]
    pub output: Output,

    /// Stream the input file instead of loading it into memory
    ///
    /// Note that this requires the input file to be sorted
    /// and will result in undefined behavior if it is not.
    #[clap(short = 'S', long)]
    pub stream: bool,
}
