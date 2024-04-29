use super::{Output, SingleInput};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct GetFastaArgs {
    #[clap(flatten)]
    pub input: SingleInput,

    /// FASTA file to extract sequences from (assumes <fasta>.fai exists)
    ///
    /// If the file ends with .gz, it will be treated as a BGZIP compressed file
    /// and decompressed on-the-fly. It will expect a corresponding .fai index
    /// and a gzip index file.
    #[clap(short, long)]
    pub fasta: String,

    /// Reverse complement the sequence if the strand is negative
    /// Default is to ignore strand information
    #[clap(short, long)]
    pub stranded: bool,

    #[clap(flatten)]
    pub output: Output,
}
