use super::utils::get_stranded_bed3;
use crate::{
    cli::bam::{BamCoverageArgs, BamCoverageParams},
    dispatch_single_with_htslib,
    io::{write_depth_iter_with, WriteNamedIter, WriteNamedIterImpl},
    types::{IntervalDepth, Rename, Renamer, SplitTranslater},
};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, IntervalContainer};
use rust_htslib::bam::{Read, Reader as BamReader, Record};
use serde::Serialize;

fn run_coverage<'a, I, N, W>(
    bam: &mut BamReader,
    mut set: IntervalContainer<I, usize, usize>,
    translater: Option<&'a SplitTranslater>,
    params: BamCoverageParams,
    writer: &mut W,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Copy + Serialize,
    N: IntervalBounds<&'a str, usize> + Serialize,
    W: std::io::Write,
    WriteNamedIterImpl: WriteNamedIter<I>,
    Renamer: Rename<'a, I, N>,
{
    // Sort the BED set intervals if necessary
    if params.sorted {
        set.set_sorted();
    } else {
        set.sort();
    }

    // Set the number of threads for the BAM reader if necessary
    if params.threads > 1 {
        bam.set_threads(params.threads)?;
    }

    let mut coverage = vec![0; set.len()];

    // Get the BAM header
    let header = bam.header().clone();

    // Export the translater (should always be Some)
    let translater = translater.unwrap();

    // Initialize the overlap query method
    let query_method = params.overlap_predicates.into();

    // Initialize an empty record to avoid repeated allocations of the BAM
    let mut record = Record::new();

    // Main loop over the BAM records
    while let Some(result) = bam.read(&mut record) {
        // exhaust the result
        result?;
        // Get the stranded BED3 record
        if let Some(bed) = get_stranded_bed3(&record, &header, translater)? {
            // Increment the coverage for each overlapping interval
            for (idx, _ov) in set.query_iter_enumerate(&bed, query_method)? {
                coverage[idx] += 1;
            }
        }
    }

    // Define an iterator over the depth and BED intervals
    let depth_iter = set
        .iter()
        .zip(coverage.iter())
        .map(|(iv, depth)| IntervalDepth::new(*iv, *depth, Some(translater)));

    // Write the depth iterator to the writer
    write_depth_iter_with(depth_iter, writer, Some(translater))
}

/// This function runs differently than the standalone coverage
/// command because it assumes that the BAM file is significantly
/// larger than the BED file. This function will load the BED file
/// and keep an in-memory representation of the intervals.
///
/// It will then also keep a single array representing the coverage
/// at each position in the bed file.
///
/// The function is based off the idea that we can stream the
/// bam file and just increment coverage across all intervals
/// that meet the overlap criteria. This will be more memory
/// efficient than loading the entire BAM file into memory.
pub fn coverage(args: BamCoverageArgs) -> Result<()> {
    let bed_reader = args.inputs.get_reader_bed()?;
    let mut bam_reader = args.inputs.get_reader_bam()?;
    let mut writer = args.output.get_writer()?;
    dispatch_single_with_htslib!(
        &mut bam_reader,
        bed_reader,
        &mut writer,
        args.params,
        run_coverage
    )
}
