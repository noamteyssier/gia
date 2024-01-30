use crate::io::{
    build_reader, iter_unnamed, match_input, match_output, read_bed3_set, write_records_iter,
    write_records_iter_with,
};
use anyhow::Result;
use bedrs::{types::iterator::ComplementIter, GenomicInterval, MergeIter};

fn complement_inplace(
    input: Option<String>,
    output: Option<String>,
    named: bool,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    // Build input handle
    let input_handle = match_input(input)?;

    // Read records into a set
    let (mut iset, translater) = read_bed3_set(input_handle, named)?;

    // Sort the set
    iset.sort();

    // Merge the set
    let merged = iset.merge()?;

    // Complement the set
    let complement_iter = merged.complement()?;

    // Match the output handle
    let output_handle = match_output(output, compression_threads, compression_level)?;

    // Write the records
    write_records_iter_with(complement_iter, output_handle, translater.as_ref())?;

    Ok(())
}

fn complement_stream(
    input: Option<String>,
    output: Option<String>,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    // Build input handle
    let input_handle = match_input(input)?;

    // Build the CSV reader
    let mut csv_reader = build_reader(input_handle);

    // Build the record iterator
    let record_iter: Box<dyn Iterator<Item = GenomicInterval<usize>>> =
        iter_unnamed(&mut csv_reader);

    // Pipe the record iterator into the merge iterator
    let merged_iter = MergeIter::new(record_iter);

    // Pipe the merge iterator into the complement iterator
    let comp_iter = ComplementIter::new(merged_iter);

    // Match the output handle
    let output_handle = match_output(output, compression_threads, compression_level)?;

    // Write the records
    write_records_iter(comp_iter, output_handle)?;
    Ok(())
}

pub fn complement(
    input: Option<String>,
    output: Option<String>,
    named: bool,
    stream: bool,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    if stream {
        complement_stream(input, output, compression_threads, compression_level)
    } else {
        complement_inplace(input, output, named, compression_threads, compression_level)
    }
}
