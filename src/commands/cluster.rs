use crate::{
    cli::{ClusterArgs, ClusterParams},
    dispatch_single,
    io::{write_depth_iter_with, WriteNamedIter, WriteNamedIterImpl},
    types::{IntervalDepth, Rename, Renamer, SplitTranslater},
};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, types::ClusterIter, IntervalContainer};
use serde::Serialize;
use std::io::Write;

fn cluster_in_memory<'a, I, N, W>(
    mut set: IntervalContainer<I, usize, usize>,
    translater: Option<&'a SplitTranslater>,
    params: ClusterParams,
    writer: W,
) -> Result<()>
where
    W: Write,
    I: IntervalBounds<usize, usize> + Copy + Serialize,
    N: IntervalBounds<&'a str, usize> + Serialize,
    WriteNamedIterImpl: WriteNamedIter<I>,
    Renamer: Rename<'a, I, N>,
{
    if !params.sorted {
        set.sort();
    } else {
        set.set_sorted();
    }
    let cluster_iter =
        ClusterIter::new(set.into_iter()).map(|(iv, cid)| IntervalDepth::new(iv, cid, translater));
    write_depth_iter_with(cluster_iter, writer, translater)
}

pub fn cluster(args: ClusterArgs) -> Result<()> {
    let reader = args.input.get_reader()?;
    let writer = args.output.get_writer()?;
    dispatch_single!(reader, writer, args.params, cluster_in_memory)
}
