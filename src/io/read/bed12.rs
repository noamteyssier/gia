use super::build_reader;
use crate::types::{Bed12Set, NamedBed12, NumericBed12, Translater};
use anyhow::{bail, Result};
use bedrs::{Coordinates, IntervalContainer};
use csv::ByteRecord;
use std::io::Read;

pub fn read_bed12_set<R: Read>(reader: R, named: bool) -> Result<(Bed12Set, Option<Translater>)> {
    if named {
        let (set, translater) = read_bed12_set_named(reader)?;
        Ok((set, Some(translater)))
    } else {
        let set = read_bed12_set_unnamed(reader)?;
        Ok((set, None))
    }
}

pub fn read_bed12_set_with<R: Read>(
    reader: R,
    translater: Option<&mut Translater>,
) -> Result<Bed12Set> {
    if let Some(translater) = translater {
        convert_bed12_set(reader, translater)
    } else {
        read_bed12_set_unnamed(reader)
    }
}

fn read_bed12_set_unnamed<R: Read>(reader: R) -> Result<Bed12Set> {
    let mut reader = build_reader(reader);
    let set = reader
        .deserialize()
        .map(|record| {
            let record: NumericBed12 = match record {
                Ok(record) => record,
                Err(e) => {
                    bail!("Could not build bed record:\n\nIf your BED has non-integer chromosome names try rerunning with the `-N` flag:\n\nERROR: {}", e)
                }
            };
            Ok(record)
        })
        .collect::<Result<IntervalContainer<NumericBed12, usize, usize>>>()?;
    Ok(set)
}

/// Reads a single file into a GenomicIntervalSet and a Translater
fn read_bed12_set_named<R: Read>(reader: R) -> Result<(Bed12Set, Translater)> {
    let mut translater = Translater::new();
    let set = convert_bed12_set(reader, &mut translater)?;
    Ok((set, translater))
}

/// Convert a CSV reader into a GenomicIntervalSet
///
/// It uses an externally initialized name map and index map to keep track of
/// chromosome names and indices. This is useful for reading multiple files
/// and keeping track of the same chromosome names and indices.
fn convert_bed12_set<R: Read>(reader: R, translater: &mut Translater) -> Result<Bed12Set> {
    let mut reader = build_reader(reader);
    let mut raw_record = ByteRecord::new();
    let mut set = Bed12Set::empty();
    while reader.read_byte_record(&mut raw_record)? {
        let record: NamedBed12 = raw_record.deserialize(None)?;
        translater.add_name(record.chr());
        translater.add_name(record.name());
        translater.add_name(record.item_rgb());
        translater.add_name(record.block_sizes());
        translater.add_name(record.block_starts());

        let chr_int = translater.get_idx(record.chr()).unwrap();
        let name_int = translater.get_idx(record.name()).unwrap();
        let item_rgb_int = translater.get_idx(record.item_rgb()).unwrap();
        let block_sizes_int = translater.get_idx(record.block_sizes()).unwrap();
        let block_starts_int = translater.get_idx(record.block_starts()).unwrap();
        let interval = NumericBed12::new(
            chr_int,
            record.start(),
            record.end(),
            name_int,
            *record.score(),
            record.strand().unwrap_or_default(),
            record.thick_start(),
            record.thick_end(),
            item_rgb_int,
            record.block_count(),
            block_sizes_int,
            block_starts_int,
        );
        set.insert(interval);
    }
    Ok(set)
}
