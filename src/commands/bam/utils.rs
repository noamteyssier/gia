use anyhow::{bail, Result};
use rust_htslib::bam::{HeaderView, Record};

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
        if record.is_reverse() {
            Ok([name, LAST_SEGMENT].concat())
        } else {
            Ok([name, FIRST_SEGMENT].concat())
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
