use anyhow::{bail, Result};
use bedrs::{
    traits::{IntervalBounds, ValueBounds},
    Container, GenomicInterval, GenomicIntervalSet,
};
use csv::ByteRecord;
use hashbrown::HashMap;
use serde::de::DeserializeOwned;
use std::io::Read;

use super::{NameIndex, NamedInterval};

pub fn build_reader<R: Read>(reader: R) -> csv::Reader<R> {
    csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .comment(Some(b'#'))
        .from_reader(reader)
}

pub fn read_set_with<R: Read>(
    reader: R,
    named: bool,
) -> Result<(GenomicIntervalSet<usize>, Option<NameIndex>)> {
    if named {
        let (set, idx_map) = read_named_set(reader)?;
        Ok((set, Some(idx_map)))
    } else {
        let set = read_set(reader)?;
        Ok((set, None))
    }
}

pub fn read_set<R: Read>(reader: R) -> Result<GenomicIntervalSet<usize>> {
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

pub fn read_iter<'a, R, I, T>(reader: &'a mut csv::Reader<R>) -> Box<dyn Iterator<Item = I> + 'a>
where
    R: Read,
    I: IntervalBounds<T> + DeserializeOwned + 'a,
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

/// Reads a single file into a GenomicIntervalSet and a NameIndex
pub fn read_named_set<R: Read>(reader: R) -> Result<(GenomicIntervalSet<usize>, NameIndex)> {
    let mut name_map = HashMap::new();
    let mut idx_map = HashMap::new();
    let set = convert_set(reader, &mut name_map, &mut idx_map)?;
    Ok((set, idx_map))
}

/// Reads two files into two GenomicIntervalSets and a NameIndex
pub fn read_two_named_sets<R: Read>(
    reader_1: R,
    reader_2: R,
) -> Result<(
    GenomicIntervalSet<usize>,
    GenomicIntervalSet<usize>,
    NameIndex,
)> {
    let mut name_map = HashMap::new();
    let mut idx_map = HashMap::new();
    let set_1 = convert_set(reader_1, &mut name_map, &mut idx_map)?;
    let set_2 = convert_set(reader_2, &mut name_map, &mut idx_map)?;
    Ok((set_1, set_2, idx_map))
}

/// Convert a CSV reader into a GenomicIntervalSet
///
/// It uses an externally initialized name map and index map to keep track of
/// chromosome names and indices. This is useful for reading multiple files
/// and keeping track of the same chromosome names and indices.
fn convert_set<R: Read>(
    reader: R,
    name_map: &mut HashMap<String, usize>,
    idx_map: &mut HashMap<usize, String>,
) -> Result<GenomicIntervalSet<usize>> {
    let mut reader = build_reader(reader);
    let mut raw_record = ByteRecord::new();
    let mut set = GenomicIntervalSet::empty();
    while reader.read_byte_record(&mut raw_record)? {
        let record: NamedInterval = raw_record.deserialize(None)?;
        if !name_map.contains_key(record.name) {
            let idx = name_map.len();
            name_map.insert(record.name.to_string(), idx);
            idx_map.insert(idx, record.name.to_string());
        }
        let chr_int = name_map.get(record.name).unwrap();
        let interval = GenomicInterval::new(*chr_int, record.start, record.end);
        set.insert(interval);
    }
    Ok(set)
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
