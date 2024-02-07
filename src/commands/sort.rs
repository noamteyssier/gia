use crate::{
    cli::{SortArgs, SortParams},
    io::{write_records_iter_with, BedReader, WriteNamedIter, WriteNamedIterImpl},
    types::{InputFormat, Reorder, Retranslater, Translater},
};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, IntervalContainer};
use serde::Serialize;
use std::io::Write;

fn sort_set<I>(
    set: &mut IntervalContainer<I, usize, usize>,
    translater: Option<Translater>,
    parallel: bool,
) -> Option<Retranslater>
where
    I: IntervalBounds<usize, usize> + Reorder<I>,
{
    let translater = if let Some(translater) = translater {
        let retranslater = I::reorder_translater(set, translater);
        Some(retranslater)
    } else {
        None
    };
    if parallel {
        set.par_sort();
    } else {
        set.sort();
    }
    translater
}

fn sort_and_write<I, W>(
    mut set: IntervalContainer<I, usize, usize>,
    translater: Option<Translater>,
    params: SortParams,
    writer: W,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Serialize + Reorder<I>,
    W: Write,
    WriteNamedIterImpl: WriteNamedIter<I>,
{
    params.initialize_thread_pool()?;
    let translater = sort_set(&mut set, translater, params.parallel());
    write_records_iter_with(set.into_iter(), writer, translater.as_ref())
}

fn dispatch_sort<W: Write>(reader: BedReader, writer: W, params: SortParams) -> Result<()> {
    match reader.input_format() {
        InputFormat::Bed3 => {
            let (set, translater) = reader.bed3_set()?;
            sort_and_write(set, translater, params, writer)
        }
        InputFormat::Bed6 => {
            let (set, translater) = reader.bed6_set()?;
            sort_and_write(set, translater, params, writer)
        }
        InputFormat::Bed12 => {
            let (set, translater) = reader.bed12_set()?;
            sort_and_write(set, translater, params, writer)
        }
    }
}

pub fn sort(args: SortArgs) -> Result<()> {
    let reader = args.input.get_reader()?;
    let writer = args.output.get_handle()?;
    dispatch_sort(reader, writer, args.params)
}
