use super::build_reader;
use crate::types::{GtfSet, NamedGtf, NumericGtf, SplitTranslater, TranslateGroup};
use anyhow::{bail, Result};
use bedrs::Coordinates;
use csv::ByteRecord;
use std::io::Read;

pub fn read_gtf_set<R: Read>(reader: R, named: bool) -> Result<(GtfSet, Option<SplitTranslater>)> {
    if named {
        let (set, translater) = read_gtf_set_named(reader)?;
        Ok((set, Some(translater)))
    } else {
        let set = read_gtf_set_unnamed(reader)?;
        Ok((set, None))
    }
}

pub fn read_gtf_set_with<R: Read>(
    reader: R,
    translater: Option<&mut SplitTranslater>,
) -> Result<GtfSet> {
    if let Some(translater) = translater {
        convert_gtf_set(reader, translater)
    } else {
        read_gtf_set_unnamed(reader)
    }
}

fn read_gtf_set_unnamed<R: Read>(reader: R) -> Result<GtfSet> {
    let mut reader = build_reader(reader);
    let set = reader
        .deserialize()
        .map(|record| {
            let record: NumericGtf = match record {
                Ok(record) => record,
                Err(e) => {
                    bail!("Could not build bed record:\n\nIf your BED has non-integer chromosome names try rerunning with the `-N` flag:\n\nERROR: {}", e)
                }
            };
            Ok(record)
        })
        .collect::<Result<GtfSet>>()?;
    Ok(set)
}

/// Reads a single file into a GenomicIntervalSet and a SplitTranslater
fn read_gtf_set_named<R: Read>(reader: R) -> Result<(GtfSet, SplitTranslater)> {
    let mut translater = SplitTranslater::new();
    let set = convert_gtf_set(reader, &mut translater)?;
    Ok((set, translater))
}

/// Convert a CSV reader into a GenomicIntervalSet
///
/// It uses an externally initialized name map and index map to keep track of
/// chromosome names and indices. This is useful for reading multiple files
/// and keeping track of the same chromosome names and indices.
fn convert_gtf_set<R: Read>(reader: R, translater: &mut SplitTranslater) -> Result<GtfSet> {
    let mut reader = build_reader(reader);
    let mut raw_record = ByteRecord::new();
    let mut set = GtfSet::empty();
    while reader.read_byte_record(&mut raw_record)? {
        let record: NamedGtf = raw_record.deserialize(None)?;
        translater.add_name(record.chr(), TranslateGroup::Chr);
        translater.add_name(record.source(), TranslateGroup::Meta);
        translater.add_name(record.feature(), TranslateGroup::Meta);
        translater.add_name(record.attributes(), TranslateGroup::Meta);
        let chr_int = translater
            .get_idx(record.chr(), TranslateGroup::Chr)
            .unwrap();
        let source_int = translater
            .get_idx(record.source(), TranslateGroup::Meta)
            .unwrap();
        let feature_int = translater
            .get_idx(record.feature(), TranslateGroup::Meta)
            .unwrap();
        let attributes_int = translater
            .get_idx(record.attributes(), TranslateGroup::Meta)
            .unwrap();
        let interval = NumericGtf::new(
            chr_int,
            source_int,
            feature_int,
            record.start(),
            record.end(),
            record.score(),
            record.strand().unwrap_or_default(),
            record.frame(),
            attributes_int,
        );
        set.insert(interval);
    }
    Ok(set)
}
