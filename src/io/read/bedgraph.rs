use super::build_reader;
use crate::types::{BedGraphSet, NamedBedGraph, NumericBedGraph, SplitTranslater, TranslateGroup};
use anyhow::{bail, Result};
use bedrs::Coordinates;
use csv::ByteRecord;
use std::io::Read;

pub fn read_bedgraph_set<R: Read>(
    reader: R,
    named: bool,
) -> Result<(BedGraphSet, Option<SplitTranslater>)> {
    if named {
        let (set, translater) = read_bedgraph_set_named(reader)?;
        Ok((set, Some(translater)))
    } else {
        let set = read_bedgraph_set_unnamed(reader)?;
        Ok((set, None))
    }
}

pub fn read_bedgraph_set_with<R: Read>(
    reader: R,
    translater: Option<&mut SplitTranslater>,
) -> Result<BedGraphSet> {
    if let Some(translater) = translater {
        convert_bedgraph_set(reader, translater)
    } else {
        read_bedgraph_set_unnamed(reader)
    }
}

fn read_bedgraph_set_unnamed<R: Read>(reader: R) -> Result<BedGraphSet> {
    let mut reader = build_reader(reader);
    let set = reader
        .deserialize()
        .map(|record| {
            let record: NumericBedGraph = match record {
                Ok(record) => record,
                Err(e) => {
                    bail!("Could not build bed record:\n\nIf your BED has non-integer chromosome names try rerunning with the `-N` flag:\n\nERROR: {}", e)
                }
            };
            Ok(record)
        })
        .collect::<Result<BedGraphSet>>()?;
    Ok(set)
}

/// Reads a single file into a GenomicIntervalSet and a SplitTranslater
fn read_bedgraph_set_named<R: Read>(reader: R) -> Result<(BedGraphSet, SplitTranslater)> {
    let mut translater = SplitTranslater::new();
    let set = convert_bedgraph_set(reader, &mut translater)?;
    Ok((set, translater))
}

/// Convert a CSV reader into a GenomicIntervalSet
///
/// It uses an externally initialized name map and index map to keep track of
/// chromosome names and indices. This is useful for reading multiple files
/// and keeping track of the same chromosome names and indices.
fn convert_bedgraph_set<R: Read>(
    reader: R,
    translater: &mut SplitTranslater,
) -> Result<BedGraphSet> {
    let mut reader = build_reader(reader);
    let mut raw_record = ByteRecord::new();
    let mut set = BedGraphSet::empty();
    while reader.read_byte_record(&mut raw_record)? {
        let record: NamedBedGraph = raw_record.deserialize(None)?;
        translater.add_name(record.chr(), TranslateGroup::Chr);
        let chr_int = translater
            .get_idx(record.chr(), TranslateGroup::Chr)
            .unwrap();
        let interval = NumericBedGraph::new(chr_int, record.start(), record.end(), record.score());
        set.insert(interval);
    }
    Ok(set)
}
