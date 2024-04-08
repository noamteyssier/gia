use crate::{
    cli::bcf::{FilterArgs, FilterParams},
    dispatch_single_with_htslib,
    io::{WriteNamedIter, WriteNamedIterImpl},
    types::{InputFormat, NumericBed3, SplitTranslater},
};

use super::utils::{parse_chr_name, parse_endpoints};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, types::QueryMethod, IntervalContainer};
use rust_htslib::bcf::{
    header::HeaderView, Read as VcfRead, Reader as VcfReader, Record, Writer as VcfWriter,
};
use serde::Serialize;

fn temp_bed3(
    record: &Record,
    header: &HeaderView,
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

fn run_inverted_overlap<I>(
    record: &Record,
    header: &HeaderView,
    set: &IntervalContainer<I, usize, usize>,
    translater: &SplitTranslater,
    query_method: QueryMethod<usize>,
    wtr: &mut VcfWriter,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Copy + Serialize,
    WriteNamedIterImpl: WriteNamedIter<I>,
{
    if let Some(bed) = temp_bed3(record, header, translater)? {
        let no_overlaps = set
            .find_iter_sorted_method_unchecked(&bed, query_method)?
            .next()
            .is_none();
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
    query_method: QueryMethod<usize>,
    wtr: &mut VcfWriter,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Copy + Serialize,
    WriteNamedIterImpl: WriteNamedIter<I>,
{
    if let Some(bed) = temp_bed3(record, header, translater)? {
        let any_overlaps = set
            .find_iter_sorted_method_unchecked(&bed, query_method)?
            .next()
            .is_some();
        if any_overlaps {
            wtr.write(record)?;
        }
    }
    Ok(())
}

fn run_filter<I>(
    vcf: &mut VcfReader,
    mut set: IntervalContainer<I, usize, usize>,
    translater: Option<&SplitTranslater>,
    params: FilterParams,
    writer: &mut VcfWriter,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Copy + Serialize,
    WriteNamedIterImpl: WriteNamedIter<I>,
{
    // Get the header
    let header = vcf.header().clone();

    // Sort the BED Set
    set.sort();

    // Export the translater
    let translater = translater.unwrap();

    // Initialize the overlap query method
    let query_method: QueryMethod<usize> = params.overlap_predicates.into();

    // Initialize an empty VCF record to avoid repeated allocations
    let mut record = vcf.empty_record();

    if params.output_predicates.invert {
        while let Some(record_result) = vcf.read(&mut record) {
            record_result?;
            run_inverted_overlap(&record, &header, &set, translater, query_method, writer)?;
        }
    } else {
        while let Some(record_result) = vcf.read(&mut record) {
            record_result?;
            run_overlap(&record, &header, &set, translater, query_method, writer)?;
        }
    }
    Ok(())
}

pub fn filter(args: FilterArgs) -> Result<()> {
    let bed_reader = args.inputs.get_reader_bed()?;
    let mut bcf_reader = args.inputs.get_reader_bcf()?;
    let mut writer = args.output.get_writer(bcf_reader.header())?;
    dispatch_single_with_htslib!(
        &mut bcf_reader,
        bed_reader,
        &mut writer,
        args.params,
        run_filter
    )
}
