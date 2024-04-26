use crate::cli::SingleInputBam;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug, Clone)]
pub struct ConvertArgs {
    #[clap(flatten)]
    pub input: SingleInputBam,

    #[clap(flatten)]
    pub params: ConvertParams,
}

#[derive(Parser, Debug, Clone)]
#[clap(next_help_heading = "Parameters")]
pub struct ConvertParams {
    /// Number of threads to use when reading BAM file
    #[clap(short, long, default_value = "1")]
    pub threads: usize,

    #[clap(short, long, default_value = "bed")]
    pub conv: BamConversionType,

    #[clap(flatten)]
    pub bed: BedConversionParams,

    #[clap(flatten)]
    pub fastq: FastqConversionParams,
}

#[derive(Parser, Debug, Clone)]
#[clap(next_help_heading = "BED Conversion Options")]
pub struct BedConversionParams {
    #[clap(short = 'C', long)]
    /// Include CIGAR string in BED output
    pub cigar: bool,
}

#[derive(Parser, Debug, Clone)]
#[clap(next_help_heading = "FASTQ Conversion Options")]
pub struct FastqConversionParams {}

#[derive(Debug, Clone, ValueEnum, Default)]
pub enum BamConversionType {
    #[default]
    Bed,
    Fastq,
}
