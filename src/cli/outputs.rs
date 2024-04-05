use crate::io::{match_bam_output, match_output};
use anyhow::Result;
use clap::{Parser, ValueEnum};
use rust_htslib::bam::{Format, HeaderView, Writer as BamWriter};
use std::io::Write;

#[derive(Parser, Debug, Clone)]
#[clap(next_help_heading = "Output Options")]
pub struct Output {
    /// Output BED file to write to (default=stdout)
    #[clap(short, long)]
    pub output: Option<String>,

    /// Compression threads to use for output files if applicable
    #[clap(global = true, short = 'j', long, default_value = "1")]
    pub compression_threads: usize,

    /// Compression level to use for output files if applicable
    #[clap(global = true, long, default_value = "6")]
    pub compression_level: u32,
}
impl Output {
    pub fn get_writer(&self) -> Result<Box<dyn Write>> {
        match_output(
            self.output.clone(),
            self.compression_threads,
            self.compression_level,
        )
    }
}

#[derive(Parser, Debug, Clone)]
#[clap(next_help_heading = "BAM Output Options")]
pub struct BamOutput {
    /// Output BAM file to write to (default=stdout)
    #[clap(short, long)]
    pub output: Option<String>,

    /// Output Format to write to (default=BAM)
    #[clap(short = 'O', long, default_value = "bam")]
    pub format: WrapHtsFormat,

    /// Threads to use when writing BAM files
    #[clap(short = 't', long, default_value = "1")]
    pub threads: usize,
}
impl BamOutput {
    pub fn get_writer(&self, header: &HeaderView) -> Result<BamWriter> {
        match_bam_output(
            self.output.clone(),
            header,
            self.format.into(),
            self.threads,
        )
    }
}

#[derive(Parser, Debug, Clone, ValueEnum, Copy)]
pub enum WrapHtsFormat {
    Bam,
    Sam,
    Cram,
}
impl From<WrapHtsFormat> for Format {
    fn from(format: WrapHtsFormat) -> Self {
        match format {
            WrapHtsFormat::Bam => Format::Bam,
            WrapHtsFormat::Sam => Format::Sam,
            WrapHtsFormat::Cram => Format::Cram,
        }
    }
}
