use std::io::Write;

use anyhow::Result;
use bedrs::{traits::IntervalBounds, IntervalContainer};
use serde::Serialize;

use crate::{
    cli::{CoverageArgs, CoverageParams},
    dispatch_pair,
    io::write_depth_iter_with,
    types::{InputFormat, IntervalDepth, Rename, Renamer, Translater},
    utils::sort_pairs,
};

fn run_coverage<'a, Ia, Ib, Na, W>(
    mut set_a: IntervalContainer<Ia, usize, usize>,
    mut set_b: IntervalContainer<Ib, usize, usize>,
    translater: Option<&'a Translater>,
    params: CoverageParams,
    writer: W,
) -> Result<()>
where
    Ia: IntervalBounds<usize, usize> + Copy + Serialize,
    Ib: IntervalBounds<usize, usize> + Copy + Serialize,
    Na: IntervalBounds<&'a str, usize> + Serialize,
    W: Write,
    Renamer: Rename<'a, Ia, Na>,
{
    sort_pairs(&mut set_a, &mut set_b, params.sorted);
    let query_method = params.overlap_predicates.into();
    let depth_iter = set_a.records().iter().map(|iv| {
        let n_overlaps = set_b
            .find_iter_sorted_method_unchecked(iv, query_method)
            .expect("Error in finding overlaps")
            .count();
        IntervalDepth::new(*iv, n_overlaps, translater)
    });
    write_depth_iter_with(depth_iter, writer, translater)
}

pub fn coverage(args: CoverageArgs) -> Result<()> {
    let (reader_a, reader_b) = args.inputs.get_readers()?;
    let writer = args.output.get_handle()?;
    dispatch_pair!(reader_a, reader_b, writer, args.params, run_coverage)
}
