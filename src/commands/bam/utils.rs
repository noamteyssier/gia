use anyhow::{bail, Result};
use noodles::bam::Record as BamRecord;
use noodles::sam::alignment::Record;
use noodles::sam::Header;

pub fn parse_chr_name<'a>(record: &BamRecord, header: &'a Header) -> Result<&'a [u8]> {
    if let Some(chr) = record.reference_sequence(header) {
        let (chr_name, _map) = chr?;
        Ok(chr_name.as_ref())
    } else {
        bail!("Record is missing chr name");
    }
}

pub fn parse_endpoints(record: &BamRecord) -> Result<(usize, usize)> {
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

const FIRST_SEGMENT: &[u8] = &[b'/', b'1'];
const LAST_SEGMENT: &[u8] = &[b'/', b'2'];
pub fn parse_query_name(record: &BamRecord) -> Result<Vec<u8>> {
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

pub fn parse_mapping_quality(record: &BamRecord) -> u8 {
    record.mapping_quality().map(|x| x.get()).unwrap_or(255)
}

pub fn get_strand(record: &BamRecord) -> char {
    if record.flags().is_reverse_complemented() {
        '-'
    } else {
        '+'
    }
}
