use super::{Output, SingleInput};
use bedrs::Strand;
use clap::{Parser, ValueEnum};

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
    /// Only merge intervals that share strandedness (will ignore intervals that have unknown
    /// strand)
    #[clap(short = 'r', long, conflicts_with("specific"))]
    pub stranded: bool,

    /// Only merge intervals that belong to a specific strand (will ignore all intervals that do
    /// not share the specified strand)
    #[clap(short = 'R', long, conflicts_with("stranded"))]
    pub specific: Option<StrandEnum>,

    /// Demote all merged intervals into BED3 format if they are not already in that format
    #[clap(short, long)]
    pub demote: bool,

    /// Assume input is sorted (default=false)
    #[clap(short, long)]
    pub sorted: bool,

    /// Stream the input file instead of loading it into memory
    ///
    /// Note that this requires the input file to be sorted
    /// and will result in undefined behavior if it is not.
    ///
    /// Currently does not support non-integer chromosome names.
    #[clap(short = 'S', long, conflicts_with_all(&["stranded", "specific"]))]
    pub stream: bool,
}

#[derive(Debug, Clone, Parser, ValueEnum)]
pub enum StrandEnum {
    #[clap(name = "+")]
    Plus,
    #[clap(name = "-")]
    Minus,
}
impl From<StrandEnum> for Strand {
    fn from(value: StrandEnum) -> Self {
        match value {
            StrandEnum::Plus => Strand::Forward,
            StrandEnum::Minus => Strand::Reverse,
        }
    }
}
