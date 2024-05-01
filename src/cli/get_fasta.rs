use super::{Output, SingleInput};
use clap::Parser;

#[derive(Parser, Debug, Clone)]
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

    #[clap(flatten)]
    pub params: GetFastaParams,

    #[clap(flatten)]
    pub output: Output,
}

#[derive(Parser, Debug, Copy, Clone)]
pub struct GetFastaParams {
    /// Reverse complement the sequence if the strand is negative
    /// Default is to ignore strand information
    #[clap(short, long)]
    pub stranded: bool,

    /// The FASTA is RNA instead of DNA and reverse complement is handled accordingly.
    #[clap(short, long, requires("stranded"))]
    pub rna: bool,
}
