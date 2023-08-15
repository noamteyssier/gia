use super::NameIndex;
use anyhow::Result;
use bedrs::{Container, Coordinates, GenomicInterval, GenomicIntervalSet};
use csv::Writer;
use dashmap::DashMap;
use std::io::Write;

pub fn build_writer<W: Write>(writer: W) -> csv::Writer<W> {
    csv::WriterBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_writer(writer)
}

pub fn write_set_with<W: Write>(
    set: &GenomicIntervalSet<usize>,
    writer: W,
    name_index: Option<&NameIndex>,
) -> Result<()> {
    if let Some(name_index) = name_index {
        write_named_set(set, writer, name_index)?;
    } else {
        write_set(set, writer)?;
    }
    Ok(())
}

pub fn write_set<W: Write>(set: &GenomicIntervalSet<usize>, writer: W) -> Result<()> {
    let mut wtr = build_writer(writer);
    write_internal(set.records(), &mut wtr)?;
    wtr.flush()?;
    Ok(())
}

pub fn write_named_set<W: Write>(
    set: &GenomicIntervalSet<usize>,
    writer: W,
    name_index: &NameIndex,
) -> Result<()> {
    let mut wtr = build_writer(writer);
    write_internal_named(set.records(), &mut wtr, name_index)?;
    wtr.flush()?;
    Ok(())
}

pub fn write_records_with<W: Write>(
    records: &[GenomicInterval<usize>],
    writer: W,
    name_index: Option<&NameIndex>,
) -> Result<()> {
    if let Some(name_index) = name_index {
        write_named_records(records, writer, name_index)?;
    } else {
        write_records(records, writer)?;
    }
    Ok(())
}

pub fn write_records<W: Write>(records: &[GenomicInterval<usize>], writer: W) -> Result<()> {
    let mut wtr = build_writer(writer);
    write_internal(records, &mut wtr)?;
    wtr.flush()?;
    Ok(())
}

pub fn write_named_records<W: Write>(
    records: &[GenomicInterval<usize>],
    writer: W,
    name_index: &NameIndex,
) -> Result<()> {
    let mut wtr = build_writer(writer);
    write_internal_named(records, &mut wtr, name_index)?;
    wtr.flush()?;
    Ok(())
}

pub fn write_records_iter_with<W, I>(
    records: I,
    writer: W,
    name_index: Option<&NameIndex>,
) -> Result<()>
where
    W: Write,
    I: Iterator<Item = GenomicInterval<usize>>,
{
    if let Some(name_index) = name_index {
        write_named_records_iter(records, writer, name_index)?;
    } else {
        write_records_iter(records, writer)?;
    }
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

pub fn write_named_records_iter<W: Write, I: Iterator<Item = GenomicInterval<usize>>>(
    records: I,
    writer: W,
    name_map: &NameIndex,
) -> Result<()> {
    let mut wtr = build_writer(writer);
    for record in records {
        let chr = name_map.get(&record.chr()).unwrap();
        let named_interval = (chr, record.start(), record.end());
        wtr.serialize(named_interval)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn write_named_records_iter_dashmap<W: Write, I: Iterator<Item = GenomicInterval<usize>>>(
    records: I,
    writer: W,
    name_map: &DashMap<usize, String>,
) -> Result<()> {
    let mut wtr = build_writer(writer);
    for record in records {
        let chr = name_map.get(&record.chr()).unwrap();
        let named_interval = (chr, record.start(), record.end());
        wtr.serialize(named_interval)?;
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

fn write_internal_named<W: Write>(
    records: &[GenomicInterval<usize>],
    wtr: &mut Writer<W>,
    name_map: &NameIndex,
) -> Result<()> {
    for interval in records.iter() {
        let chr = name_map.get(&interval.chr()).unwrap();
        let named_interval = (chr, interval.start(), interval.end());
        wtr.serialize(named_interval)?;
    }
    Ok(())
}
