use crate::{
    io::{
        build_reader, iter_unnamed, match_output, read_bed12_set, read_bed3_set, read_bed6_set,
        write_3col_iter_with, write_records_iter, BedReader, WriteNamedIter, WriteNamedIterImpl,
    },
    types::{FieldFormat, InputFormat, NumericBed3, Translater},
};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, IntervalContainer, MergeIter};
use serde::Serialize;
use std::io::Write;

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

fn merge_by_format<W>(bed_reader: BedReader, output_handle: W, sorted: bool) -> Result<()>
where
    W: Write,
{
    let named = bed_reader.is_named();
    match bed_reader.input_format() {
        InputFormat::Bed3 => {
            let (set, translater) = read_bed3_set(bed_reader.reader(), named)?;
            merge_in_memory(set, translater, output_handle, sorted)
        }
        InputFormat::Bed6 => {
            let (set, translater) = read_bed6_set(bed_reader.reader(), named)?;
            merge_in_memory(set, translater, output_handle, sorted)
        }
        InputFormat::Bed12 => {
            let (set, translater) = read_bed12_set(bed_reader.reader(), named)?;
            merge_in_memory(set, translater, output_handle, sorted)
        }
    }
}

fn merge_streamed<Iv, W>(record_iter: impl Iterator<Item = Iv>, output_handle: W) -> Result<()>
where
    W: Write,
    Iv: IntervalBounds<usize, usize> + Serialize,
{
    let merged_iter = MergeIter::new(record_iter);
    write_records_iter(merged_iter, output_handle)?;
    Ok(())
}

fn merge_streamed_by_format<W: Write>(bed_reader: BedReader, output_handle: W) -> Result<()> {
    if bed_reader.is_named() {
        return Err(anyhow::anyhow!(
            "Named input is not supported for streaming"
        ));
    }
    let input_format = bed_reader.input_format();
    let mut csv_reader = build_reader(bed_reader.reader());
    match input_format {
        InputFormat::Bed3 => {
            let record_iter: Box<dyn Iterator<Item = NumericBed3>> = iter_unnamed(&mut csv_reader);
            merge_streamed(record_iter, output_handle)
        }
        InputFormat::Bed6 => {
            let record_iter: Box<dyn Iterator<Item = NumericBed3>> = iter_unnamed(&mut csv_reader);
            merge_streamed(record_iter, output_handle)
        }
        InputFormat::Bed12 => {
            let record_iter: Box<dyn Iterator<Item = NumericBed3>> = iter_unnamed(&mut csv_reader);
            merge_streamed(record_iter, output_handle)
        }
    }
}

pub fn merge(
    input: Option<String>,
    output: Option<String>,
    sorted: bool,
    stream: bool,
    input_format: Option<InputFormat>,
    field_format: Option<FieldFormat>,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    let bed_reader = BedReader::from_path(input, input_format, field_format)?;
    let output_handle = match_output(output, compression_threads, compression_level)?;
    if stream {
        merge_streamed_by_format(bed_reader, output_handle)
    } else {
        merge_by_format(bed_reader, output_handle, sorted)
    }
}
