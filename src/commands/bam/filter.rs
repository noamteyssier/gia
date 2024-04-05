use crate::{
    cli::bam::{FilterArgs, FilterParams},
    dispatch_single_with_bam,
    io::{match_bam_input, WriteNamedIter, WriteNamedIterImpl},
    types::{InputFormat, NumericBed3, SplitTranslater},
};

use super::utils::{parse_chr_name, parse_endpoints};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, types::QueryMethod, IntervalContainer};
use noodles::bam::io::Writer as BamWriter;
use noodles::bam::Record as BamRecord;
use noodles::{
    bam::io::{reader::Builder, Reader},
    sam::Header,
};
use serde::Serialize;
use std::io::{Read, Write};

fn temp_bed3(
    record: &BamRecord,
    header: &Header,
    translater: &SplitTranslater,
) -> Result<Option<NumericBed3>> {
    let chr_bytes = parse_chr_name(record, header)?;
    let chr_name = std::str::from_utf8(chr_bytes)?;
    let chr_idx = if let Some(idx) = translater.get_chr_idx(chr_name) {
        idx
    } else {
        return Ok(None);
    };
    let (start, end) = parse_endpoints(record)?;
    Ok(Some(NumericBed3::new(chr_idx, start, end)))
}

fn run_inverted_overlap<I, W>(
    record: &BamRecord,
    header: &Header,
    set: &IntervalContainer<I, usize, usize>,
    translater: &SplitTranslater,
    query_method: QueryMethod<usize>,
    wtr: &mut BamWriter<W>,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Copy + Serialize,
    W: Write,
    WriteNamedIterImpl: WriteNamedIter<I>,
{
    if let Some(bed) = temp_bed3(record, header, translater)? {
        let no_overlaps = set
            .find_iter_sorted_method_unchecked(&bed, query_method)?
            .next()
            .is_none();
        if no_overlaps {
            wtr.write_record(header, record)?;
        }
    } else {
        wtr.write_record(header, record)?;
    }
    Ok(())
}

fn run_overlap<I, W>(
    record: &BamRecord,
    header: &Header,
    set: &IntervalContainer<I, usize, usize>,
    translater: &SplitTranslater,
    query_method: QueryMethod<usize>,
    wtr: &mut BamWriter<W>,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Copy + Serialize,
    W: Write,
    WriteNamedIterImpl: WriteNamedIter<I>,
{
    if let Some(bed) = temp_bed3(record, header, translater)? {
        let any_overlaps = set
            .find_iter_sorted_method_unchecked(&bed, query_method)?
            .next()
            .is_some();
        if any_overlaps {
            wtr.write_record(header, record)?;
        }
    }
    Ok(())
}

fn run_filter<I, R, W>(
    mut bam: Reader<R>,
    header: Header,
    mut set: IntervalContainer<I, usize, usize>,
    translater: Option<&SplitTranslater>,
    params: FilterParams,
    writer: W,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Copy + Serialize,
    R: Read,
    W: Write,
    WriteNamedIterImpl: WriteNamedIter<I>,
{
    // Sort the BED Set
    set.sort();

    // Export the translater
    let translater = translater.unwrap();

    // Initialize the BAM Writer
    let mut wtr = BamWriter::new(writer);
    wtr.write_header(&header)?;

    // Initialize the overlap query method
    let query_method: QueryMethod<usize> = params.overlap_predicates.into();

    if params.output_predicates.invert {
        for record in bam.records() {
            let record = record?;
            run_inverted_overlap(&record, &header, &set, translater, query_method, &mut wtr)?;
        }
    } else {
        for record in bam.records() {
            let record = record?;
            run_overlap(&record, &header, &set, translater, query_method, &mut wtr)?;
        }
    }
    Ok(())
}

pub fn filter(args: FilterArgs) -> Result<()> {
    let bed_reader = args.inputs.get_reader_bed()?;
    let bam_handle = match_bam_input(args.inputs.bam)?;
    let mut bam = Builder.build_from_reader(bam_handle);
    let header = bam.read_header()?;
    let writer = args.output.get_writer()?;
    dispatch_single_with_bam!(bam, header, bed_reader, writer, args.params, run_filter)
}
