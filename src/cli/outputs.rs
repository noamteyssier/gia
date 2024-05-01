use crate::io::{match_bam_output, match_bcf_output, match_output};
use anyhow::Result;
use clap::{Parser, ValueEnum};
use rust_htslib::{
    bam::{Format as SamFormat, HeaderView as BamHeaderView, Writer as BamWriter},
    bcf::{header::HeaderView as VcfHeaderView, Format as VcfFormat, Writer as VcfWriter},
};
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
    pub format: WrapSamFormat,

    /// Threads to use when writing BAM files
    #[clap(short = 't', long, default_value = "1")]
    pub threads: usize,
}
impl BamOutput {
    pub fn get_writer(&self, header: &BamHeaderView) -> Result<BamWriter> {
        match_bam_output(
            self.output.clone(),
            header,
            self.format.into(),
            self.threads,
        )
    }
}

#[derive(Parser, Debug, Clone)]
#[clap(next_help_heading = "BAM Output Options")]
pub struct VcfOutput {
    /// Output BCF file to write to (default=stdout)
    #[clap(short, long)]
    pub output: Option<String>,

    /// Output Format to write to
    ///
    /// v/z: VCF (uncompressed/compressed)
    ///
    /// u/b: BCF (uncompressed/compressed)
    #[clap(short = 'O', long, default_value = "b")]
    pub format: WrapVcfFormat,

    /// Threads to use when writing BCF/VCF files
    #[clap(short = 't', long, default_value = "1")]
    pub threads: usize,
}
impl VcfOutput {
    pub fn get_writer(&self, header: &VcfHeaderView) -> Result<VcfWriter> {
        match_bcf_output(
            self.output.clone(),
            header,
            self.format.into(),
            self.format.into(),
            self.threads,
        )
    }
}

#[derive(Parser, Debug, Clone, ValueEnum, Copy)]
pub enum WrapSamFormat {
    Bam,
    Sam,
    Cram,
}
impl From<WrapSamFormat> for SamFormat {
    fn from(format: WrapSamFormat) -> Self {
        match format {
            WrapSamFormat::Bam => SamFormat::Bam,
            WrapSamFormat::Sam => SamFormat::Sam,
            WrapSamFormat::Cram => SamFormat::Cram,
        }
    }
}

#[derive(Parser, Debug, Clone, ValueEnum, Copy)]
pub enum WrapVcfFormat {
    #[clap(name = "z")]
    VcfCompressed,
    #[clap(name = "v")]
    VcfUncompressed,
    #[clap(name = "b")]
    BcfCompressed,
    #[clap(name = "u")]
    BcfUncompressed,
}
impl From<WrapVcfFormat> for VcfFormat {
    fn from(format: WrapVcfFormat) -> Self {
        match format {
            WrapVcfFormat::VcfCompressed | WrapVcfFormat::VcfUncompressed => VcfFormat::Vcf,
            WrapVcfFormat::BcfCompressed | WrapVcfFormat::BcfUncompressed => VcfFormat::Bcf,
        }
    }
}
impl From<WrapVcfFormat> for bool {
    fn from(format: WrapVcfFormat) -> Self {
        match format {
            WrapVcfFormat::VcfCompressed | WrapVcfFormat::BcfCompressed => true,
            WrapVcfFormat::VcfUncompressed | WrapVcfFormat::BcfUncompressed => false,
        }
    }
}
