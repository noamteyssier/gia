use anyhow::{bail, Result};
use bedrs::{Strand, StrandedBed3};
use rust_htslib::bam::{HeaderView, Record};

use crate::types::SplitTranslater;

pub fn parse_chr_name<'a>(record: &Record, header: &'a HeaderView) -> Result<&'a [u8]> {
    let tid = record.tid();
    if tid < 0 {
        bail!("Record is missing chr name");
    }
    let chr_name = header.tid2name(tid as u32);
    Ok(chr_name)
}

pub fn parse_endpoints(record: &Record) -> Result<(usize, usize)> {
    let start = record.pos() as usize;
    let end = record.cigar().end_pos() as usize;
    Ok((start, end))
}

const FIRST_SEGMENT: &[u8] = &[b'/', b'1'];
const LAST_SEGMENT: &[u8] = &[b'/', b'2'];
pub fn parse_query_name(record: &Record) -> Result<Vec<u8>> {
    let name = record.qname();
    if record.is_paired() {
        if record.is_first_in_template() {
            Ok([name, FIRST_SEGMENT].concat())
        } else {
            Ok([name, LAST_SEGMENT].concat())
        }
    } else {
        Ok(name.to_vec())
    }
}

pub fn parse_mapping_quality(record: &Record) -> u8 {
    record.mapq()
}

pub fn get_strand(record: &Record) -> char {
    if record.is_reverse() {
        '-'
    } else {
        '+'
    }
}

pub fn parse_strand(record: &Record) -> Strand {
    match get_strand(record) {
        '+' => Strand::Forward,
        '-' => Strand::Reverse,
        _ => Strand::Unknown,
    }
}

pub fn get_stranded_bed3(
    record: &Record,
    header: &HeaderView,
    translater: &SplitTranslater,
) -> Result<Option<StrandedBed3<usize>>> {
    let chr_bytes = parse_chr_name(record, header)?;
    let chr_name = std::str::from_utf8(chr_bytes)?;
    let chr_idx = if let Some(idx) = translater.get_chr_idx(chr_name) {
        idx
    } else {
        return Ok(None);
    };
    let (start, end) = parse_endpoints(record)?;
    let strand = parse_strand(record);
    Ok(Some(StrandedBed3::new(chr_idx, start, end, strand)))
}
