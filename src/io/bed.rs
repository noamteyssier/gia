use anyhow::Result;
use bedrs::{Container, GenomicInterval, GenomicIntervalSet};
use csv::Writer;
use std::io::{Read, Write};

pub fn read_set<R: Read>(reader: R) -> Result<GenomicIntervalSet<usize>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .comment(Some(b'#'))
        .from_reader(reader);
    let set = reader
        .deserialize()
        .map(|record| {
            let record: GenomicInterval<usize> = record?;
            Ok(record)
        })
        .collect::<Result<GenomicIntervalSet<usize>>>()?;
    Ok(set)
}

pub fn write_set<W: Write>(set: &GenomicIntervalSet<usize>, writer: W) -> Result<()> {
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_writer(writer);
    write_internal(set.records(), &mut wtr)?;
    wtr.flush()?;
    Ok(())
}

pub fn write_records<W: Write>(records: &[GenomicInterval<usize>], writer: W) -> Result<()> {
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_writer(writer);
    write_internal(records, &mut wtr)?;
    wtr.flush()?;
    Ok(())
}

fn write_internal<W: Write>(records: &[GenomicInterval<usize>], wtr: &mut Writer<W>) -> Result<()> {
    for interval in records.iter() {
        wtr.serialize(interval)?;
    }
    Ok(())
}
