use super::build_reader;
use crate::types::{Bed6Set, NamedBed6, NumericBed6, SplitTranslater, TranslateGroup};
use anyhow::{bail, Result};
use bedrs::Coordinates;
use csv::ByteRecord;
use std::io::Read;

pub fn read_bed6_set<R: Read>(
    reader: R,
    named: bool,
) -> Result<(Bed6Set, Option<SplitTranslater>)> {
    if named {
        let (set, translater) = read_bed6_set_named(reader)?;
        Ok((set, Some(translater)))
    } else {
        let set = read_bed6_set_unnamed(reader)?;
        Ok((set, None))
    }
}

pub fn read_bed6_set_with<R: Read>(
    reader: R,
    translater: Option<&mut SplitTranslater>,
) -> Result<Bed6Set> {
    if let Some(translater) = translater {
        convert_bed6_set(reader, translater)
    } else {
        read_bed6_set_unnamed(reader)
    }
}

fn read_bed6_set_unnamed<R: Read>(reader: R) -> Result<Bed6Set> {
    let mut reader = build_reader(reader);
    let set = reader
        .deserialize()
        .map(|record| {
            let record: NumericBed6 = match record {
                Ok(record) => record,
                Err(e) => {
                    bail!("Could not build bed record:\n\nIf your BED has non-integer chromosome names try rerunning with the `-N` flag:\n\nERROR: {}", e)
                }
            };
            Ok(record)
        })
        .collect::<Result<Bed6Set>>()?;
    Ok(set)
}

/// Reads a single file into a GenomicIntervalSet and a SplitTranslater
fn read_bed6_set_named<R: Read>(reader: R) -> Result<(Bed6Set, SplitTranslater)> {
    let mut translater = SplitTranslater::new();
    let set = convert_bed6_set(reader, &mut translater)?;
    Ok((set, translater))
}

/// Convert a CSV reader into a GenomicIntervalSet
///
/// It uses an externally initialized name map and index map to keep track of
/// chromosome names and indices. This is useful for reading multiple files
/// and keeping track of the same chromosome names and indices.
fn convert_bed6_set<R: Read>(reader: R, translater: &mut SplitTranslater) -> Result<Bed6Set> {
    let mut reader = build_reader(reader);
    let mut raw_record = ByteRecord::new();
    let mut set = Bed6Set::empty();
    while reader.read_byte_record(&mut raw_record)? {
        let record: NamedBed6 = raw_record.deserialize(None)?;
        translater.add_name(record.chr(), TranslateGroup::Chr);
        translater.add_name(record.name(), TranslateGroup::Meta);
        let chr_int = translater
            .get_idx(record.chr(), TranslateGroup::Chr)
            .unwrap();
        let name_int = translater
            .get_idx(record.name(), TranslateGroup::Meta)
            .unwrap();
        let interval = NumericBed6::new(
            chr_int,
            record.start(),
            record.end(),
            name_int,
            record.score(),
            record.strand().unwrap_or_default(),
        );
        set.insert(interval);
    }
    Ok(set)
}
