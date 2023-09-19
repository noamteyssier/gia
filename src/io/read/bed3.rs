use super::build_reader;
use crate::{io::NamedInterval, types::Translater};
use anyhow::{bail, Result};
use bedrs::{
    traits::{ChromBounds, IntervalBounds, ValueBounds},
    Container, GenomicInterval, GenomicIntervalSet,
};
use csv::ByteRecord;
use serde::de::DeserializeOwned;
use std::io::Read;

pub fn read_bed3_set<R: Read>(
    reader: R,
    named: bool,
) -> Result<(GenomicIntervalSet<usize>, Option<Translater>)> {
    if named {
        let (set, idx_map) = read_bed3_set_named(reader)?;
        Ok((set, Some(idx_map)))
    } else {
        let set = read_bed3_set_unnamed(reader)?;
        Ok((set, None))
    }
}

pub fn read_paired_bed3_sets<R: Read>(
    reader_1: R,
    reader_2: R,
    named: bool,
) -> Result<(
    GenomicIntervalSet<usize>,
    GenomicIntervalSet<usize>,
    Option<Translater>,
)> {
    if named {
        let (query_set, target_set, translater) = read_paired_bed3_named(reader_1, reader_2)?;
        Ok((query_set, target_set, Some(translater)))
    } else {
        let query_set = read_bed3_set_unnamed(reader_1)?;
        let target_set = read_bed3_set_unnamed(reader_2)?;
        Ok((query_set, target_set, None))
    }
}

pub fn read_bed3_set_unnamed<R: Read>(reader: R) -> Result<GenomicIntervalSet<usize>> {
    let mut reader = build_reader(reader);
    let set = reader
        .deserialize()
        .map(|record| {
            let record: GenomicInterval<usize> = match record {
                Ok(record) => record,
                Err(e) => {
                    bail!("Could not build bed record:\n\nIf your BED has non-integer chromosome names try rerunning with the `-N` flag:\n\nERROR: {}", e)
                }
            };
            Ok(record)
        })
        .collect::<Result<GenomicIntervalSet<usize>>>()?;
    Ok(set)
}

/// Reads a single file into a GenomicIntervalSet and a Translater
pub fn read_bed3_set_named<R: Read>(reader: R) -> Result<(GenomicIntervalSet<usize>, Translater)> {
    let mut translater = Translater::new();
    let set = convert_bed3_set(reader, &mut translater)?;
    Ok((set, translater))
}

/// Convert a CSV reader into a GenomicIntervalSet
///
/// It uses an externally initialized name map and index map to keep track of
/// chromosome names and indices. This is useful for reading multiple files
/// and keeping track of the same chromosome names and indices.
fn convert_bed3_set<R: Read>(
    reader: R,
    translater: &mut Translater,
) -> Result<GenomicIntervalSet<usize>> {
    let mut reader = build_reader(reader);
    let mut raw_record = ByteRecord::new();
    let mut set = GenomicIntervalSet::empty();
    while reader.read_byte_record(&mut raw_record)? {
        let record: NamedInterval = raw_record.deserialize(None)?;
        translater.add_name(record.name);
        let chr_int = translater.get_idx(record.name).unwrap();
        let interval = GenomicInterval::new(chr_int, record.start, record.end);
        set.insert(interval);
    }
    Ok(set)
}

pub fn iter_bed3_unnamed<'a, R, I, C, T>(
    reader: &'a mut csv::Reader<R>,
) -> Box<dyn Iterator<Item = I> + 'a>
where
    R: Read,
    I: IntervalBounds<C, T> + DeserializeOwned + 'a,
    C: ChromBounds,
    T: ValueBounds,
{
    let record_iter = reader
        .deserialize()
        .map(|record| {
            let record: I = match record {
                Ok(record) => record,
                Err(e) => {
                    panic!("Could not build bed record:\n\nIf your BED has non-integer chromosome names try rerunning with the `-N` flag:\n\nERROR: {}", e)
                }
            };
            record
        });
    Box::new(record_iter)
}

/// Reads two files into two GenomicIntervalSets and a NameIndex
pub fn read_paired_bed3_named<R: Read>(
    reader_1: R,
    reader_2: R,
) -> Result<(
    GenomicIntervalSet<usize>,
    GenomicIntervalSet<usize>,
    Translater,
)> {
    let mut translater = Translater::new();
    let set_1 = convert_bed3_set(reader_1, &mut translater)?;
    let set_2 = convert_bed3_set(reader_2, &mut translater)?;
    Ok((set_1, set_2, translater))
}
