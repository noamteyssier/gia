use crate::{
    cli::{SegmentArgs, SegmentParams},
    dispatch_single,
    io::{write_records_iter_with, WriteNamedIter, WriteNamedIterImpl},
    types::SplitTranslater,
};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, IntervalContainer};
use serde::Serialize;
use std::io::Write;

fn run_segment<I, W>(
    mut set: IntervalContainer<I, usize, usize>,
    translater: Option<&SplitTranslater>,
    params: SegmentParams,
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
    let segments = set.segment()?;
    write_records_iter_with(segments.into_iter(), writer, translater)
}

pub fn segment(args: SegmentArgs) -> Result<()> {
    let reader = args.input.get_reader()?;
    let writer = args.output.get_writer()?;
    dispatch_single!(reader, writer, args.params, run_segment)
}
