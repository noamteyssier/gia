use super::build_reader;
use crate::types::{Bed4Set, NamedBed4, NumericBed4, SplitTranslater, TranslateGroup};
use anyhow::{bail, Result};
use bedrs::Coordinates;
use csv::ByteRecord;
use std::io::Read;

pub fn read_bed4_set<R: Read>(
    reader: R,
    named: bool,
) -> Result<(Bed4Set, Option<SplitTranslater>)> {
    if named {
        let (set, translater) = read_bed4_set_named(reader)?;
        Ok((set, Some(translater)))
    } else {
        let set = read_bed4_set_unnamed(reader)?;
        Ok((set, None))
    }
}

pub fn read_bed4_set_with<R: Read>(
    reader: R,
    translater: Option<&mut SplitTranslater>,
) -> Result<Bed4Set> {
    if let Some(translater) = translater {
        convert_bed4_set(reader, translater)
    } else {
        read_bed4_set_unnamed(reader)
    }
}

fn read_bed4_set_unnamed<R: Read>(reader: R) -> Result<Bed4Set> {
    let mut reader = build_reader(reader);
    let set = reader
        .deserialize()
        .map(|record| {
            let record: NumericBed4 = match record {
                Ok(record) => record,
                Err(e) => {
                    bail!("Could not build bed record:\n\nIf your BED has non-integer chromosome names try rerunning with the `-N` flag:\n\nERROR: {}", e)
                }
            };
            Ok(record)
        })
        .collect::<Result<Bed4Set>>()?;
    Ok(set)
}

/// Reads a single file into a GenomicIntervalSet and a SplitTranslater
fn read_bed4_set_named<R: Read>(reader: R) -> Result<(Bed4Set, SplitTranslater)> {
    let mut translater = SplitTranslater::new();
    let set = convert_bed4_set(reader, &mut translater)?;
    Ok((set, translater))
}

/// Convert a CSV reader into a GenomicIntervalSet
///
/// It uses an externally initialized name map and index map to keep track of
/// chromosome names and indices. This is useful for reading multiple files
/// and keeping track of the same chromosome names and indices.
fn convert_bed4_set<R: Read>(reader: R, translater: &mut SplitTranslater) -> Result<Bed4Set> {
    let mut reader = build_reader(reader);
    let mut raw_record = ByteRecord::new();
    let mut set = Bed4Set::empty();
    while reader.read_byte_record(&mut raw_record)? {
        let record: NamedBed4 = raw_record.deserialize(None)?;
        translater.add_name(record.chr(), TranslateGroup::Chr);
        translater.add_name(record.name(), TranslateGroup::Meta);
        let chr_int = translater
            .get_idx(record.chr(), TranslateGroup::Chr)
            .unwrap();
        let name_int = translater
            .get_idx(record.name(), TranslateGroup::Meta)
            .unwrap();
        let interval = NumericBed4::new(chr_int, record.start(), record.end(), name_int);
        set.insert(interval);
    }
    Ok(set)
}
