use crate::cli::bam::ConvertParams;
use crate::commands::bam::utils::{
    get_strand, parse_chr_name, parse_endpoints, parse_mapping_quality, parse_query_name,
};
use crate::io::build_writer;

use anyhow::Result;
use rust_htslib::bam::{HeaderView, Read, Reader as BamReader, Record};
use std::io::{stdout, Write};
use std::str::from_utf8;

fn format_print_record<W: Write>(
    record: &Record,
    header: &HeaderView,
    params: &ConvertParams,
    wtr: &mut csv::Writer<W>,
) -> Result<()> {
    let chr_name = parse_chr_name(record, header)?;
    let (start, end) = parse_endpoints(record)?;
    let qname = parse_query_name(record)?;
    let mapq = parse_mapping_quality(record);
    let strand = get_strand(record);
    //
    if params.bed.cigar {
        let cigar = record.cigar();
        let tuple = (
            from_utf8(chr_name)?,
            start,
            end,
            from_utf8(&qname)?,
            mapq,
            strand,
            format!("{}", cigar),
        );
        wtr.serialize(tuple)?;
    } else {
        let tuple = (
            from_utf8(chr_name)?,
            start,
            end,
            from_utf8(&qname)?,
            mapq,
            strand,
        );
        wtr.serialize(tuple)?;
    }
    Ok(())
}

pub fn convert_bed(mut bam: BamReader, params: ConvertParams) -> Result<()> {
    let header = bam.header().clone();
    let mut wtr = build_writer(stdout());
    let mut record = Record::new();
    while let Some(result) = bam.read(&mut record) {
        result?;
        format_print_record(&record, &header, &params, &mut wtr)?;
    }
    wtr.flush()?;
    Ok(())
}
