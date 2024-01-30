use crate::{
    io::{
        build_reader, iter_unnamed, match_input, match_output, read_bed12_set, read_bed3_set,
        read_bed6_set, write_3col_iter_with, write_records_iter, WriteNamedIter,
        WriteNamedIterImpl,
    },
    types::{InputFormat, NumericBed3, Translater},
};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, IntervalContainer, MergeIter};
use serde::Serialize;
use std::io::{Read, Write};

fn merge_in_memory<I, W>(
    mut set: IntervalContainer<I, usize, usize>,
    translater: Option<Translater>,
    output_handle: W,
    sorted: bool,
) -> Result<()>
where
    W: Write,
    I: IntervalBounds<usize, usize> + Serialize,
    WriteNamedIterImpl: WriteNamedIter<I>,
{
    if !sorted {
        set.sort();
    } else {
        set.set_sorted();
    }
    let merged = set.merge()?;
    write_3col_iter_with(merged.into_iter(), output_handle, translater.as_ref())?;
    Ok(())
}

fn merge_by_format<R, W>(
    input_handle: R,
    output_handle: W,
    format: InputFormat,
    named: bool,
    sorted: bool,
) -> Result<()>
where
    R: Read,
    W: Write,
{
    match format {
        InputFormat::Bed3 => {
            let (set, translater) = read_bed3_set(input_handle, named)?;
            merge_in_memory(set, translater, output_handle, sorted)
        }
        InputFormat::Bed6 => {
            let (set, translater) = read_bed6_set(input_handle, named)?;
            merge_in_memory(set, translater, output_handle, sorted)
        }
        InputFormat::Bed12 => {
            let (set, translater) = read_bed12_set(input_handle, named)?;
            merge_in_memory(set, translater, output_handle, sorted)
        }
    }
}

fn merge_streamed<R, W>(input_handle: R, output_handle: W) -> Result<()>
where
    R: Read,
    W: Write,
{
    let mut csv_reader = build_reader(input_handle);
    let record_iter: Box<dyn Iterator<Item = NumericBed3>> = iter_unnamed(&mut csv_reader);
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
    format: InputFormat,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    let input_handle = match_input(input)?;
    let output_handle = match_output(output, compression_threads, compression_level)?;
    if stream {
        merge_streamed(input_handle, output_handle)
    } else {
        merge_by_format(input_handle, output_handle, format, named, sorted)
    }
}
