use super::build_reader;
use crate::types::{MetaIntervalSet, NumericMetaInterval, SplitTranslater, TranslateGroup};
use anyhow::{bail, Result};
use csv::ByteRecord;
use std::{io::Read, str::from_utf8};

pub fn read_meta_interval_set<R: Read>(
    reader: R,
    named: bool,
) -> Result<(MetaIntervalSet, Option<SplitTranslater>)> {
    if named {
        let (set, translater) = read_meta_interval_set_named(reader)?;
        Ok((set, Some(translater)))
    } else {
        let set = read_meta_interval_set_unnamed(reader)?;
        Ok((set, None))
    }
}

pub fn read_meta_interval_set_with<R: Read>(
    reader: R,
    translater: Option<&mut SplitTranslater>,
) -> Result<MetaIntervalSet> {
    if let Some(translater) = translater {
        convert_meta_interval_set(reader, translater)
    } else {
        read_meta_interval_set_unnamed(reader)
    }
}

fn read_meta_interval_set_unnamed<R: Read>(reader: R) -> Result<MetaIntervalSet> {
    let mut reader = build_reader(reader);
    let set = reader
        .deserialize()
        .map(|record| {
            let record: NumericMetaInterval= match record {
                Ok(record) => record,
                Err(e) => {
                    bail!("Could not build bed record:\n\nIf your BED has non-integer chromosome names try rerunning with the `-N` flag:\n\nERROR: {}", e)
                }
            };
            Ok(record)
        })
        .collect::<Result<MetaIntervalSet>>()?;
    Ok(set)
}

/// Reads a single file into a GenomicIntervalSet and a SplitTranslater
fn read_meta_interval_set_named<R: Read>(reader: R) -> Result<(MetaIntervalSet, SplitTranslater)> {
    let mut translater = SplitTranslater::new();
    let set = convert_meta_interval_set(reader, &mut translater)?;
    Ok((set, translater))
}

/// Convert a CSV reader into a GenomicIntervalSet
///
/// It uses an externally initialized name map and index map to keep track of
/// chromosome names and indices. This is useful for reading multiple files
/// and keeping track of the same chromosome names and indices.
fn convert_meta_interval_set<R: Read>(
    reader: R,
    translater: &mut SplitTranslater,
) -> Result<MetaIntervalSet> {
    let mut reader = build_reader(reader);
    let mut raw_record = ByteRecord::new();
    let mut set = MetaIntervalSet::empty();
    let mut buffer = String::new();
    while reader.read_byte_record(&mut raw_record)? {
        // Iterate over the fields of the record
        let mut record_iter = raw_record.iter();

        // Parse the chromosome
        let chr = record_iter.next().map(from_utf8).unwrap()?;

        // Parse the start and end
        let start = record_iter
            .next()
            .map(from_utf8)
            .unwrap()?
            .parse::<usize>()?;
        let end = record_iter
            .next()
            .map(from_utf8)
            .unwrap()?
            .parse::<usize>()?;

        // Parse the metadata into a single long string
        buffer.clear();
        let first_meta = record_iter.next().unwrap();
        buffer.push_str(from_utf8(first_meta)?);
        for field in record_iter {
            buffer.push('\t');
            buffer.push_str(from_utf8(field)?);
        }

        // Add the chromosome and metadata to the translater
        translater.add_name(chr, TranslateGroup::Chr);
        translater.add_name(&buffer, TranslateGroup::Meta);
        let chr_int = translater.get_idx(chr, TranslateGroup::Chr).unwrap();
        let name_int = translater.get_idx(&buffer, TranslateGroup::Meta).unwrap();

        // Create the interval and add it to the set
        let interval = NumericMetaInterval::new(chr_int, start, end, name_int);
        set.insert(interval);
    }
    Ok(set)
}
