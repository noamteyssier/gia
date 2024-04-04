use super::{Output, SingleInput};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct GetFastaArgs {
    #[clap(flatten)]
    pub input: SingleInput,

    /// FASTA file to extract sequences from (assumes <fasta>.fai exists)
    #[clap(short, long)]
    pub fasta: String,

    #[clap(flatten)]
    pub output: Output,
}
