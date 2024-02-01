use super::build_reader;
use crate::{
    io::NamedInterval,
    types::{Bed3Set, NumericBed3, Translater},
};
use anyhow::{bail, Result};
use bedrs::IntervalContainer;
use csv::ByteRecord;
use std::io::Read;

pub fn read_bed3_set<R: Read>(reader: R, named: bool) -> Result<(Bed3Set, Option<Translater>)> {
    if named {
        let (set, idx_map) = read_bed3_set_named(reader)?;
        Ok((set, Some(idx_map)))
    } else {
        let set = read_bed3_set_unnamed(reader)?;
        Ok((set, None))
    }
}

pub fn read_bed3_set_with<R: Read>(
    reader: R,
    translater: Option<&mut Translater>,
) -> Result<Bed3Set> {
    if let Some(translater) = translater {
        convert_bed3_set(reader, translater)
    } else {
        read_bed3_set_unnamed(reader)
    }
}

fn read_bed3_set_unnamed<R: Read>(reader: R) -> Result<Bed3Set> {
    let mut reader = build_reader(reader);
    let set = reader
        .deserialize()
        .map(|record| {
            let record: NumericBed3 = match record {
                Ok(record) => record,
                Err(e) => {
                    bail!("Could not build bed record:\n\nIf your BED has non-integer chromosome names try rerunning with the `-N` flag:\n\nERROR: {}", e)
                }
            };
            Ok(record)
        })
        .collect::<Result<Bed3Set>>()?;
    Ok(set)
}

/// Reads a single file into a GenomicIntervalSet and a Translater
fn read_bed3_set_named<R: Read>(reader: R) -> Result<(Bed3Set, Translater)> {
    let mut translater = Translater::new();
    let set = convert_bed3_set(reader, &mut translater)?;
    Ok((set, translater))
}

/// Convert a CSV reader into a GenomicIntervalSet
///
/// It uses an externally initialized name map and index map to keep track of
/// chromosome names and indices. This is useful for reading multiple files
/// and keeping track of the same chromosome names and indices.
fn convert_bed3_set<R: Read>(reader: R, translater: &mut Translater) -> Result<Bed3Set> {
    let mut reader = build_reader(reader);
    let mut raw_record = ByteRecord::new();
    let mut set = IntervalContainer::empty();
    while reader.read_byte_record(&mut raw_record)? {
        let record: NamedInterval = raw_record.deserialize(None)?;
        translater.add_name(record.name);
        let chr_int = translater.get_idx(record.name).unwrap();
        let interval = NumericBed3::new(chr_int, record.start, record.end);
        set.insert(interval);
    }
    Ok(set)
}
