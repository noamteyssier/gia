mod bed;
pub use bed::convert_bed;

use crate::cli::bam::{BamConversionType, ConvertArgs, ConvertParams};
use crate::io::match_bam_input;

use anyhow::Result;
use noodles::bam::io::reader::Builder;
use noodles::bam::io::Reader;
use noodles::sam::Header;
use std::io::Read;

fn dispatch_conversion<R: Read>(
    bam: Reader<R>,
    header: Header,
    params: ConvertParams,
) -> Result<()> {
    match params.conv {
        BamConversionType::Bed => convert_bed(bam, header, params),
        _ => unimplemented!(),
    }
}

pub fn convert(args: ConvertArgs) -> Result<()> {
    let in_handle = match_bam_input(args.input.input)?;
    let mut bam = Builder.build_from_reader(in_handle);
    let header = bam.read_header()?;
    dispatch_conversion(bam, header, args.params)
}
