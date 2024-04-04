use crate::cli::bam::{ConvertParams, WrapCigar};
use crate::io::build_writer;

use anyhow::{bail, Result};
use noodles::bam::io::Reader;
use noodles::bam::Record as BamRecord;
use noodles::sam::alignment::Record;
use noodles::sam::Header;
use std::io::{stdout, BufWriter, Read, Write};
use std::str::from_utf8;

const FIRST_SEGMENT: &[u8] = &[b'/', b'1'];
const LAST_SEGMENT: &[u8] = &[b'/', b'2'];

fn parse_chr_name<'a>(record: &BamRecord, header: &'a Header) -> Result<&'a [u8]> {
    if let Some(chr) = record.reference_sequence(header) {
        let (chr_name, _map) = chr?;
        Ok(chr_name.as_ref())
    } else {
        bail!("Record is missing chr name");
    }
}

fn parse_endpoints(record: &BamRecord) -> Result<(usize, usize)> {
    let start = if let Some(start) = record.alignment_start() {
        // Adjust to 0-based
        start?.get() - 1
    } else {
        bail!("Record is missing start");
    };
    let end = if let Some(end) = record.alignment_end() {
        end?.get()
    } else {
        bail!("Record is missing end");
    };
    Ok((start, end))
}

fn parse_query_name(record: &BamRecord) -> Result<Vec<u8>> {
    if let Some(name) = record.name() {
        if record.flags().is_segmented() {
            if record.flags().is_first_segment() {
                Ok([name.as_bytes(), FIRST_SEGMENT].concat())
            } else {
                Ok([name.as_bytes(), LAST_SEGMENT].concat())
            }
        } else {
            Ok(name.as_bytes().to_vec())
        }
    } else {
        bail!("Record is missing query name");
    }
}

fn parse_mapping_quality(record: &BamRecord) -> u8 {
    record.mapping_quality().map(|x| x.get()).unwrap_or(255)
}

fn get_strand(record: &BamRecord) -> char {
    if record.flags().is_reverse_complemented() {
        '-'
    } else {
        '+'
    }
}

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
