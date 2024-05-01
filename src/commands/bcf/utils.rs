use anyhow::{bail, Result};
use rust_htslib::bcf::{header::HeaderView, Record};

pub fn parse_chr_name<'a>(record: &Record, header: &'a HeaderView) -> Result<&'a [u8]> {
    if let Some(rid) = record.rid() {
        let chr_name = header.rid2name(rid)?;
        Ok(chr_name)
    } else {
        bail!("Record is missing chr name");
    }
}

pub fn parse_endpoints(record: &Record) -> Result<(usize, usize)> {
    let start = record.pos() as usize;
    let end = record.end() as usize;
    Ok((start, end))
}
