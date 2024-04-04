use crate::cli::bam::{ConvertParams, WrapCigar};
use crate::commands::bam::utils::{
    get_strand, parse_chr_name, parse_endpoints, parse_mapping_quality, parse_query_name,
};
use crate::io::build_writer;

use anyhow::Result;
use noodles::bam::io::Reader;
use noodles::bam::Record as BamRecord;
use noodles::sam::Header;
use std::io::{stdout, BufWriter, Read, Write};
use std::str::from_utf8;

fn format_print_record<W: Write>(
    record: &BamRecord,
    header: &Header,
    params: &ConvertParams,
    wtr: &mut csv::Writer<W>,
) -> Result<()> {
    let chr_name = parse_chr_name(record, header)?;
    let (start, end) = parse_endpoints(record)?;
    let qname = parse_query_name(record)?;
    let mapq = parse_mapping_quality(record);
    let strand = get_strand(record);

    if params.bed.cigar {
        let cigar: WrapCigar = record.cigar().into();
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

pub fn convert_bed<R: Read>(
    mut bam: Reader<R>,
    header: Header,
    params: ConvertParams,
) -> Result<()> {
    let mut wtr = build_writer(BufWriter::new(stdout()));
    for record in bam.records() {
        let record = record?;
        format_print_record(&record, &header, &params, &mut wtr)?;
    }
    wtr.flush()?;
    Ok(())
}
