use anyhow::Result;
use bedrs::Coordinates;
use csv::ByteRecord;
use std::{io::Read, str::from_utf8};

use super::build_reader;
use crate::types::{
    Bed12Set, Bed3Set, Bed4Set, Bed6Set, BedGraphSet, GtfSet, MetaIntervalSet, NamedBed12,
    NamedBed3, NamedBed4, NamedBed6, NamedBedGraph, NamedGtf, NumericBed12, NumericBed3,
    NumericBed4, NumericBed6, NumericBedGraph, NumericGtf, NumericMetaInterval, SplitTranslater,
    TranslateGroup,
};

pub fn read_bed3_set_named<R: Read>(
    reader: R,
    set: &mut Bed3Set,
    translater: &mut SplitTranslater,
) -> Result<()> {
    let mut reader = build_reader(reader);
    let mut raw_record = ByteRecord::new();
    while reader.read_byte_record(&mut raw_record)? {
        let record: NamedBed3 = raw_record.deserialize(None)?;
        translater.add_name(record.chr(), TranslateGroup::Chr);
        let chr_int = translater
            .get_idx(record.chr(), TranslateGroup::Chr)
            .unwrap();
        let interval = NumericBed3::new(chr_int, record.start(), record.end());
        set.insert(interval);
    }
    Ok(())
}

pub fn read_bed4_set_named<R: Read>(
    reader: R,
    set: &mut Bed4Set,
    translater: &mut SplitTranslater,
) -> Result<()> {
    let mut reader = build_reader(reader);
    let mut raw_record = ByteRecord::new();
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
    Ok(())
}

pub fn read_bed6_set_named<R: Read>(
    reader: R,
    set: &mut Bed6Set,
    translater: &mut SplitTranslater,
) -> Result<()> {
    let mut reader = build_reader(reader);
    let mut raw_record = ByteRecord::new();
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
    Ok(())
}

pub fn read_bed12_set_named<R: Read>(
    reader: R,
    set: &mut Bed12Set,
    translater: &mut SplitTranslater,
) -> Result<()> {
    let mut reader = build_reader(reader);
    let mut raw_record = ByteRecord::new();
    while reader.read_byte_record(&mut raw_record)? {
        let record: NamedBed12 = raw_record.deserialize(None)?;
        translater.add_name(record.chr(), TranslateGroup::Chr);
        translater.add_name(record.name(), TranslateGroup::Meta);
        translater.add_name(record.item_rgb(), TranslateGroup::Meta);
        translater.add_name(record.block_sizes(), TranslateGroup::Meta);
        translater.add_name(record.block_starts(), TranslateGroup::Meta);

        let chr_int = translater
            .get_idx(record.chr(), TranslateGroup::Chr)
            .unwrap();
        let name_int = translater
            .get_idx(record.name(), TranslateGroup::Meta)
            .unwrap();
        let item_rgb_int = translater
            .get_idx(record.item_rgb(), TranslateGroup::Meta)
            .unwrap();
        let block_sizes_int = translater
            .get_idx(record.block_sizes(), TranslateGroup::Meta)
            .unwrap();
        let block_starts_int = translater
            .get_idx(record.block_starts(), TranslateGroup::Meta)
            .unwrap();
        let interval = NumericBed12::new(
            chr_int,
            record.start(),
            record.end(),
            name_int,
            record.score(),
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
    Ok(())
}

pub fn read_gtf_set_named<R: Read>(
    reader: R,
    set: &mut GtfSet,
    translater: &mut SplitTranslater,
) -> Result<()> {
    let mut reader = build_reader(reader);
    let mut raw_record = ByteRecord::new();
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
    Ok(())
}

pub fn read_bedgraph_set_named<R: Read>(
    reader: R,
    set: &mut BedGraphSet,
    translater: &mut SplitTranslater,
) -> Result<()> {
    let mut reader = build_reader(reader);
    let mut raw_record = ByteRecord::new();
    while reader.read_byte_record(&mut raw_record)? {
        let record: NamedBedGraph = raw_record.deserialize(None)?;
        translater.add_name(record.chr(), TranslateGroup::Chr);
        let chr_int = translater
            .get_idx(record.chr(), TranslateGroup::Chr)
            .unwrap();
        let interval = NumericBedGraph::new(chr_int, record.start(), record.end(), record.score());
        set.insert(interval);
    }
    Ok(())
}

pub fn read_meta_interval_set_named<R: Read>(
    reader: R,
    set: &mut MetaIntervalSet,
    translater: &mut SplitTranslater,
) -> Result<()> {
    let mut reader = build_reader(reader);
    let mut raw_record = ByteRecord::new();
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
    Ok(())
}
