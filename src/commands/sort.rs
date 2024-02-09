use crate::{
    cli::{SortArgs, SortParams},
    dispatch_single_owned_tl,
    io::{write_records_iter_with, WriteNamedIter, WriteNamedIterImpl},
    types::{InputFormat, Reorder, SplitRetranslater, SplitTranslater},
};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, IntervalContainer};
use serde::Serialize;
use std::io::Write;

fn sort_set<I>(
    set: &mut IntervalContainer<I, usize, usize>,
    translater: Option<SplitTranslater>,
    parallel: bool,
) -> Option<SplitRetranslater>
where
    I: IntervalBounds<usize, usize> + Reorder<I>,
{
    let translater = if let Some(translater) = translater {
        let (chr_tl, meta_tl) = translater.disband();
        let retranslater = I::reorder_translater(set, chr_tl);
        Some(SplitRetranslater::new(retranslater, meta_tl))
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
    translater: Option<SplitTranslater>,
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

pub fn sort(args: SortArgs) -> Result<()> {
    let reader = args.input.get_reader()?;
    let writer = args.output.get_writer()?;
    dispatch_single_owned_tl!(reader, writer, args.params, sort_and_write)
}
