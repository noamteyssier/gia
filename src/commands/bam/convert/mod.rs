mod bed;
pub use bed::convert_bed;

use crate::cli::bam::{BamConversionType, ConvertArgs, ConvertParams};
use crate::io::match_bam_input;

use anyhow::{bail, Result};
use rust_htslib::bam::Reader as BamReader;

fn dispatch_conversion(bam: BamReader, params: ConvertParams) -> Result<()> {
    match params.conv {
        BamConversionType::Bed => convert_bed(bam, params),
        _ => bail!(
            "FASTQ conversion is not implemented yet - but checkout samtools fastq for a solution"
        ),
    }
}

pub fn convert(args: ConvertArgs) -> Result<()> {
    let bam = match_bam_input(args.input.input)?;
    dispatch_conversion(bam, args.params)
}
