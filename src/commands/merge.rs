use crate::{
    cli::{MergeArgs, MergeParams},
    dispatch_single,
    io::{
        build_reader, iter_unnamed, write_3col_iter_with, write_records_iter, BedReader,
        WriteNamedIter, WriteNamedIterImpl,
    },
    types::{InputFormat, NumericBed12, NumericBed3, NumericBed4, NumericBed6, SplitTranslater},
};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, IntervalContainer, MergeIter};
use serde::Serialize;
use std::io::Write;

fn merge_in_memory<I, W>(
    mut set: IntervalContainer<I, usize, usize>,
    translater: Option<&SplitTranslater>,
    params: MergeParams,
    writer: W,
) -> Result<()>
where
    W: Write,
    I: IntervalBounds<usize, usize> + Serialize,
    WriteNamedIterImpl: WriteNamedIter<I>,
{
    if !params.sorted {
        set.sort();
    } else {
        set.set_sorted();
    }
    let merged = set.merge()?;
    write_3col_iter_with(merged.into_iter(), writer, translater)?;
    Ok(())
}

fn merge_streamed<Iv, W>(record_iter: impl Iterator<Item = Iv>, writer: W) -> Result<()>
where
    W: Write,
    Iv: IntervalBounds<usize, usize> + Serialize,
{
    let merged_iter = MergeIter::new(record_iter);
    write_records_iter(merged_iter, writer)?;
    Ok(())
}

fn merge_streamed_by_format<W: Write>(bed_reader: BedReader, writer: W) -> Result<()> {
    if bed_reader.is_named() {
        return Err(anyhow::anyhow!(
            "Named input is not supported for streaming"
        ));
    }
    let input_format = bed_reader.input_format();
    let mut csv_reader = build_reader(bed_reader.reader());
    match input_format {
        InputFormat::Bed3 | InputFormat::Ambiguous => {
            let record_iter: Box<dyn Iterator<Item = NumericBed3>> = iter_unnamed(&mut csv_reader);
            merge_streamed(record_iter, writer)
        }
        InputFormat::Bed4 => {
            let record_iter: Box<dyn Iterator<Item = NumericBed4>> = iter_unnamed(&mut csv_reader);
            merge_streamed(record_iter, writer)
        }
        InputFormat::Bed6 => {
            let record_iter: Box<dyn Iterator<Item = NumericBed6>> = iter_unnamed(&mut csv_reader);
            merge_streamed(record_iter, writer)
        }
        InputFormat::Bed12 => {
            let record_iter: Box<dyn Iterator<Item = NumericBed12>> = iter_unnamed(&mut csv_reader);
            merge_streamed(record_iter, writer)
        }
    }
}

pub fn merge(args: MergeArgs) -> Result<()> {
    let reader = args.input.get_reader()?;
    let writer = args.output.get_writer()?;
    if args.params.stream {
        merge_streamed_by_format(reader, writer)
    } else {
        dispatch_single!(reader, writer, args.params, merge_in_memory)
    }
}
