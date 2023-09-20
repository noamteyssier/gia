use crate::types::{IntervalPair, StreamTranslater, Translate, Translater};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, Container, Coordinates, GenomicInterval, GenomicIntervalSet};
use csv::Writer;
use serde::Serialize;
use std::io::Write;

pub fn build_writer<W: Write>(writer: W) -> csv::Writer<W> {
    csv::WriterBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_writer(writer)
}

pub fn write_set_with<W: Write, T: Translate>(
    set: &GenomicIntervalSet<usize>,
    writer: W,
    translater: Option<&T>,
) -> Result<()> {
    if let Some(translater) = translater {
        write_named_set(set, writer, translater)?;
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

pub fn write_named_set<W: Write, T: Translate>(
    set: &GenomicIntervalSet<usize>,
    writer: W,
    translater: &T,
) -> Result<()> {
    let mut wtr = build_writer(writer);
    write_internal_named(set.records(), &mut wtr, translater)?;
    wtr.flush()?;
    Ok(())
}

fn write_internal<W: Write>(records: &[GenomicInterval<usize>], wtr: &mut Writer<W>) -> Result<()> {
    for interval in records.iter() {
        wtr.serialize(interval)?;
    }
    Ok(())
}

fn write_internal_named<W: Write, T: Translate>(
    records: &[GenomicInterval<usize>],
    wtr: &mut Writer<W>,
    translater: &T,
) -> Result<()> {
    for interval in records.iter() {
        let chr = translater.get_name(*interval.chr()).unwrap();
        let named_interval = (chr, interval.start(), interval.end());
        wtr.serialize(named_interval)?;
    }
    Ok(())
}

pub fn write_records_iter<W, R, I>(records: I, writer: W) -> Result<()>
where
    W: Write,
    R: Serialize,
    I: Iterator<Item = R>,
{
    let mut wtr = build_writer(writer);
    for record in records {
        wtr.serialize(record)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn write_pairs_iter_with<W, I, It>(
    records: It,
    writer: W,
    translater: Option<&Translater>,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Serialize,
    W: Write,
    It: Iterator<Item = IntervalPair<I, usize, usize>>,
{
    if let Some(translater) = translater {
        write_named_pairs_iter(records, writer, translater)?;
    } else {
        write_pairs_iter(records, writer)?;
    }
    Ok(())
}

pub fn write_pairs_iter<W, I, It>(records: It, writer: W) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Serialize,
    W: Write,
    It: Iterator<Item = IntervalPair<I, usize, usize>>,
{
    let mut wtr = build_writer(writer);
    for record in records {
        let pair = (
            record.iv_a.chr(),
            record.iv_a.start(),
            record.iv_a.end(),
            record.iv_b.as_ref().map(|iv| iv.chr()),
            record.iv_b.as_ref().map(|iv| iv.start()),
            record.iv_b.as_ref().map(|iv| iv.end()),
        );
        wtr.serialize(pair)?;
    }
    Ok(())
}

pub fn write_named_pairs_iter<I, W, It>(
    records: It,
    writer: W,
    translater: &Translater,
) -> Result<()>
where
    I: IntervalBounds<usize, usize>,
    W: Write,
    It: Iterator<Item = IntervalPair<I, usize, usize>>,
{
    let mut wtr = build_writer(writer);
    for record in records {
        let chr_a = translater.get_name(*record.iv_a.chr()).unwrap();
        let chr_b = if let Some(ref iv_b) = record.iv_b {
            Some(translater.get_name(*iv_b.chr()).unwrap())
        } else {
            None
        };
        let named_pair = (
            chr_a,
            record.iv_a.start(),
            record.iv_a.end(),
            chr_b,
            record.iv_b.as_ref().map(|iv| iv.start()),
            record.iv_b.as_ref().map(|iv| iv.end()),
        );
        wtr.serialize(named_pair)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn write_named_records_iter_dashmap<W: Write, I: Iterator<Item = GenomicInterval<usize>>>(
    records: I,
    writer: W,
    translater: &StreamTranslater,
) -> Result<()> {
    let mut wtr = build_writer(writer);
    for record in records {
        let chr = translater.get_idx_to_name().get(record.chr()).unwrap();
        let named_interval = (chr, record.start(), record.end());
        wtr.serialize(named_interval)?;
    }
    wtr.flush()?;
    Ok(())
}
