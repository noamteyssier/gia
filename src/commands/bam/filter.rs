use crate::{
    cli::bam::{FilterArgs, FilterParams},
    dispatch_single_with_htslib,
    io::{WriteNamedIter, WriteNamedIterImpl},
    types::{InputFormat, SplitTranslater},
};

use super::utils::get_stranded_bed3;
use anyhow::Result;
use bedrs::{traits::IntervalBounds, types::Query, IntervalContainer};
use rust_htslib::bam::{HeaderView, Read, Reader as BamReader, Record, Writer as BamWriter};
use serde::Serialize;

fn run_inverted_overlap<I>(
    record: &Record,
    header: &HeaderView,
    set: &IntervalContainer<I, usize, usize>,
    translater: &SplitTranslater,
    query_method: Query<usize>,
    wtr: &mut BamWriter,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Copy + Serialize,
    WriteNamedIterImpl: WriteNamedIter<I>,
{
    if let Some(bed) = get_stranded_bed3(record, header, translater)? {
        let no_overlaps = set.query_iter(&bed, query_method)?.next().is_none();
        if no_overlaps {
            wtr.write(record)?;
        }
    } else {
        wtr.write(record)?;
    }
    Ok(())
}

fn run_overlap<I>(
    record: &Record,
    header: &HeaderView,
    set: &IntervalContainer<I, usize, usize>,
    translater: &SplitTranslater,
    query_method: Query<usize>,
    wtr: &mut BamWriter,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Copy + Serialize,
    WriteNamedIterImpl: WriteNamedIter<I>,
{
    if let Some(bed) = get_stranded_bed3(record, header, translater)? {
        let any_overlaps = set.query_iter(&bed, query_method)?.next().is_some();
        if any_overlaps {
            wtr.write(record)?;
        }
    }
    Ok(())
}

fn run_filter<I>(
    bam: &mut BamReader,
    mut set: IntervalContainer<I, usize, usize>,
    translater: Option<&SplitTranslater>,
    params: FilterParams,
    writer: &mut BamWriter,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Copy + Serialize,
    WriteNamedIterImpl: WriteNamedIter<I>,
{
    // Get the header
    let header = bam.header().clone();

    // Sort the BED Set
    set.sort();

    // Export the translater
    let translater = translater.unwrap();

    // Initialize the overlap query method
    let query_method = params.overlap_predicates.into();

    // Initialize an empty record to avoid repeated allocations
    let mut record = Record::new();

    if params.output_predicates.invert {
        while let Some(result) = bam.read(&mut record) {
            result?;
            run_inverted_overlap(&record, &header, &set, translater, query_method, writer)?;
        }
    } else {
        while let Some(result) = bam.read(&mut record) {
            result?;
            run_overlap(&record, &header, &set, translater, query_method, writer)?;
        }
    }
    Ok(())
}

pub fn filter(args: FilterArgs) -> Result<()> {
    let bed_reader = args.inputs.get_reader_bed()?;
    let mut bam_reader = args.inputs.get_reader_bam()?;
    let mut writer = args.output.get_writer(bam_reader.header())?;
    dispatch_single_with_htslib!(
        &mut bam_reader,
        bed_reader,
        &mut writer,
        args.params,
        run_filter
    )
}
