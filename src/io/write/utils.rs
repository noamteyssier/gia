use crate::types::{IntervalPair, NumericBed3, Rename, Renamer, StreamTranslater, Translater};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, Coordinates};
use serde::Serialize;
use std::io::Write;

pub fn build_writer<W: Write>(writer: W) -> csv::Writer<W> {
    csv::WriterBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
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

pub fn write_pairs_iter_with<'a, W, Ia, Ib, Na, Nb, It>(
    records: It,
    writer: W,
    translater: Option<&Translater>,
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
