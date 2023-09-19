use std::io::{Read, Write};

use crate::io::{
    build_reader, iter_bed3_unnamed, match_input, match_output, read_bed3_set, write_records_iter,
    write_records_with,
};
use anyhow::Result;
use bedrs::{Container, GenomicInterval, Merge, MergeIter};

fn merge_in_memory<R, W>(input_handle: R, output_handle: W, sorted: bool, named: bool) -> Result<()>
where
    R: Read,
    W: Write,
{
    let (mut set, translater) = read_bed3_set(input_handle, named)?;
    if !sorted {
        set.sort();
    } else {
        set.set_sorted();
    }
    let merged = set.merge()?;
    write_records_with(merged.records(), output_handle, translater.as_ref())?;
    Ok(())
}

fn merge_streamed<R, W>(input_handle: R, output_handle: W) -> Result<()>
where
    R: Read,
    W: Write,
{
    let mut csv_reader = build_reader(input_handle);
    let record_iter: Box<dyn Iterator<Item = GenomicInterval<usize>>> =
        iter_bed3_unnamed(&mut csv_reader);
    let merged_iter = MergeIter::new(record_iter);
    write_records_iter(merged_iter, output_handle)?;
    Ok(())
}

pub fn merge(
    input: Option<String>,
    output: Option<String>,
    sorted: bool,
    named: bool,
    stream: bool,
) -> Result<()> {
    let input_handle = match_input(input)?;
    let output_handle = match_output(output)?;
    if stream {
        merge_streamed(input_handle, output_handle)
    } else {
        merge_in_memory(input_handle, output_handle, sorted, named)
    }
}
