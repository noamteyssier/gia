use crate::types::{
    IntervalDepth, IntervalPair, IntervalSpacing, NumericBed3, Rename, Renamer, SplitTranslater,
    StreamTranslater,
};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, Coordinates};
use csv::QuoteStyle;
use serde::Serialize;
use std::io::Write;

pub fn build_writer<W: Write>(writer: W) -> csv::Writer<W> {
    csv::WriterBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .quote_style(QuoteStyle::Never)
        .from_writer(writer)
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

pub fn write_depth_iter_with<'a, W, I, N, It>(
    records: It,
    writer: W,
    translater: Option<&SplitTranslater>,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Serialize,
    N: IntervalBounds<&'a str, usize> + Serialize,
    W: Write,
    It: Iterator<Item = IntervalDepth<'a, I, N>>,
    Renamer: Rename<'a, I, N>,
{
    if translater.is_some() {
        write_named_depth_iter(records, writer)
    } else {
        write_depth_iter(records, writer)
    }
}

fn write_depth_iter<'a, W, I, N, It>(records: It, writer: W) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Serialize,
    N: IntervalBounds<&'a str, usize> + Serialize,
    W: Write,
    It: Iterator<Item = IntervalDepth<'a, I, N>>,
    Renamer: Rename<'a, I, N>,
{
    let mut wtr = build_writer(writer);
    for record in records {
        wtr.serialize(record.get_tuple())?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn write_named_depth_iter<'a, W, I, N, It>(records: It, writer: W) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Serialize,
    N: IntervalBounds<&'a str, usize> + Serialize,
    W: Write,
    It: Iterator<Item = IntervalDepth<'a, I, N>>,
    Renamer: Rename<'a, I, N>,
{
    let mut wtr = build_writer(writer);
    for record in records {
        wtr.serialize(record.get_named_tuple())?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn write_spacing_iter_with<'a, W, I, N, It>(
    records: It,
    writer: W,
    translater: Option<&SplitTranslater>,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Serialize,
    N: IntervalBounds<&'a str, usize> + Serialize,
    W: Write,
    It: Iterator<Item = IntervalSpacing<'a, I, N>>,
    Renamer: Rename<'a, I, N>,
{
    if translater.is_some() {
        write_named_spacing_iter(records, writer)
    } else {
        write_spacing_iter(records, writer)
    }
}

fn write_spacing_iter<'a, W, I, N, It>(records: It, writer: W) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Serialize,
    N: IntervalBounds<&'a str, usize> + Serialize,
    W: Write,
    It: Iterator<Item = IntervalSpacing<'a, I, N>>,
    Renamer: Rename<'a, I, N>,
{
    let mut wtr = build_writer(writer);
    for record in records {
        wtr.serialize(record.get_tuple())?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn write_named_spacing_iter<'a, W, I, N, It>(records: It, writer: W) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Serialize,
    N: IntervalBounds<&'a str, usize> + Serialize,
    W: Write,
    It: Iterator<Item = IntervalSpacing<'a, I, N>>,
    Renamer: Rename<'a, I, N>,
{
    let mut wtr = build_writer(writer);
    for record in records {
        wtr.serialize(record.get_named_tuple())?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn write_pairs_iter_with<'a, W, Ia, Ib, Na, Nb, It>(
    records: It,
    writer: W,
    translater: Option<&SplitTranslater>,
) -> Result<()>
where
    Ia: IntervalBounds<usize, usize> + Serialize,
    Ib: IntervalBounds<usize, usize> + Serialize,
    Na: IntervalBounds<&'a str, usize> + Serialize,
    Nb: IntervalBounds<&'a str, usize> + Serialize,
    W: Write,
    It: Iterator<Item = IntervalPair<'a, Ia, Ib, Na, Nb>>,
    Renamer: Rename<'a, Ia, Na> + Rename<'a, Ib, Nb>,
{
    if translater.is_some() {
        write_named_pairs_iter(records, writer)?;
    } else {
        write_pairs_iter(records, writer)?;
    }
    Ok(())
}

pub fn write_pairs_iter<'a, W, Ia, Ib, Na, Nb, It>(records: It, writer: W) -> Result<()>
where
    Ia: IntervalBounds<usize, usize> + Serialize,
    Ib: IntervalBounds<usize, usize> + Serialize,
    Na: IntervalBounds<&'a str, usize> + Serialize,
    Nb: IntervalBounds<&'a str, usize> + Serialize,
    W: Write,
    It: Iterator<Item = IntervalPair<'a, Ia, Ib, Na, Nb>>,
    Renamer: Rename<'a, Ia, Na> + Rename<'a, Ib, Nb>,
{
    let mut wtr = build_writer(writer);
    for record in records {
        wtr.serialize(record.get_tuple())?;
    }
    Ok(())
}

pub fn write_named_pairs_iter<'a, Ia, Ib, Na, Nb, W, It>(records: It, writer: W) -> Result<()>
where
    Ia: IntervalBounds<usize, usize> + Serialize,
    Ib: IntervalBounds<usize, usize> + Serialize,
    Na: IntervalBounds<&'a str, usize> + Serialize,
    Nb: IntervalBounds<&'a str, usize> + Serialize,
    W: Write,
    It: Iterator<Item = IntervalPair<'a, Ia, Ib, Na, Nb>>,
    Renamer: Rename<'a, Ia, Na> + Rename<'a, Ib, Nb>,
{
    let mut wtr = build_writer(writer);
    for record in records {
        let pair = record.get_named_tuple();
        wtr.serialize(pair)?;
    }
    wtr.flush()?;
    Ok(())
}

/// Write a segment to a CSV writer
/// without initializing the writer or flushing
///
/// Used in UnionBedGraph
pub fn write_segment<W: Write>(
    writer: &mut csv::Writer<W>,
    translater: Option<&SplitTranslater>,
    segment: NumericBed3,
    scores: &[f64],
) -> Result<()> {
    if let Some(tx) = translater {
        write_named_segment(writer, tx, segment, scores)
    } else {
        write_unnamed_segment(writer, segment, scores)
    }
}

/// Write a named segment and scores to a CSV writer
fn write_named_segment<W: Write>(
    writer: &mut csv::Writer<W>,
    translater: &SplitTranslater,
    segment: NumericBed3,
    scores: &[f64],
) -> Result<()> {
    let named_segment = Renamer::rename_with(&segment, translater);
    let tuple = (named_segment, scores);
    writer.serialize(tuple)?;
    Ok(())
}

/// Write an unnamed segment and scores to a CSV writer
fn write_unnamed_segment<W: Write>(
    writer: &mut csv::Writer<W>,
    segment: NumericBed3,
    scores: &[f64],
) -> Result<()> {
    let tuple = (segment, scores);
    writer.serialize(tuple)?;
    Ok(())
}

pub fn write_named_records_iter_dashmap<W: Write, I: Iterator<Item = NumericBed3>>(
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
