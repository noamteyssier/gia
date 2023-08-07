use anyhow::Result;
use bedrs::{Container, GenomicInterval, GenomicIntervalSet};
use csv::Writer;
use hashbrown::HashMap;
use std::io::{Read, Write};

pub fn build_writer<W: Write>(writer: W) -> csv::Writer<W> {
    csv::WriterBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_writer(writer)
}

pub fn build_reader<R: Read>(reader: R) -> csv::Reader<R> {
    csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .comment(Some(b'#'))
        .from_reader(reader)
}

pub fn read_set<R: Read>(reader: R) -> Result<GenomicIntervalSet<usize>> {
    let mut reader = build_reader(reader);
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
    let mut wtr = build_writer(writer);
    write_internal(set.records(), &mut wtr)?;
    wtr.flush()?;
    Ok(())
}

pub fn write_records<W: Write>(records: &[GenomicInterval<usize>], writer: W) -> Result<()> {
    let mut wtr = build_writer(writer);
    write_internal(records, &mut wtr)?;
    wtr.flush()?;
    Ok(())
}

pub fn write_records_iter<W: Write, I: Iterator<Item = GenomicInterval<usize>>>(
    records: I,
    writer: W,
) -> Result<()> {
    let mut wtr = build_writer(writer);
    for record in records {
        wtr.serialize(record)?;
    }
    wtr.flush()?;
    Ok(())
}

fn write_internal<W: Write>(records: &[GenomicInterval<usize>], wtr: &mut Writer<W>) -> Result<()> {
    for interval in records.iter() {
        wtr.serialize(interval)?;
    }
    Ok(())
}

pub fn read_name_map<R: Read>(reader: R) -> Result<HashMap<usize, String>> {
    let mut reader = build_reader(reader);
    let map = reader
        .deserialize()
        .map(|record| {
            let record: (usize, String) = record?;
            Ok(record)
        })
        .collect::<Result<HashMap<usize, String>>>()?;
    Ok(map)
}

pub fn read_genome<R: Read>(reader: R) -> Result<HashMap<usize, usize>> {
    let mut reader = build_reader(reader);
    let map = reader
        .deserialize()
        .map(|record| {
            let record: (usize, usize) = record?;
            Ok(record)
        })
        .collect::<Result<HashMap<usize, usize>>>()?;
    Ok(map)
}
