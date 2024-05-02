use crate::{
    cli::{CoverageArgs, CoverageParams},
    dispatch_pair, dispatch_pair_multi,
    io::{write_depth_iter_with, write_par_depth_iter_with},
    types::{IntervalDepth, Rename, Renamer, SplitTranslater},
    utils::sort_pairs,
};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, IntervalContainer};
use rayon::prelude::*;
use serde::Serialize;
use std::io::Write;

fn initialize_thread_pool(n_threads: usize) -> Result<()> {
    if n_threads < 2 {
        return Ok(());
    }
    rayon::ThreadPoolBuilder::new()
        .num_threads(n_threads)
        .build_global()?;
    Ok(())
}

fn run_coverage<'a, Ia, Ib, Na, W>(
    mut set_a: IntervalContainer<Ia, usize, usize>,
    mut set_b: IntervalContainer<Ib, usize, usize>,
    translater: Option<&'a SplitTranslater>,
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
            .query_iter(iv, query_method)
            .expect("Error in finding overlaps")
            .count();
        IntervalDepth::new(*iv, n_overlaps, translater)
    });
    write_depth_iter_with(depth_iter, writer, translater)
}

fn run_par_coverage<'a, Ia, Ib, Na, W>(
    mut set_a: IntervalContainer<Ia, usize, usize>,
    mut set_b: IntervalContainer<Ib, usize, usize>,
    translater: Option<&'a SplitTranslater>,
    params: CoverageParams,
    writer: W,
) -> Result<()>
where
    Ia: IntervalBounds<usize, usize> + Copy + Serialize,
    Ib: IntervalBounds<usize, usize> + Copy + Serialize,
    Na: IntervalBounds<&'a str, usize> + Serialize,
    W: Write + Send + Sync,
    Renamer: Rename<'a, Ia, Na>,
{
    sort_pairs(&mut set_a, &mut set_b, params.sorted);
    let query_method = params.overlap_predicates.into();
    let depth_iter = set_a.records().into_par_iter().map(|iv| {
        let n_overlaps = set_b
            .query_iter(iv, query_method)
            .expect("Error in finding overlaps")
            .count();
        IntervalDepth::new(*iv, n_overlaps, translater)
    });
    write_par_depth_iter_with(depth_iter, params.chunk_size, writer, translater)
}

pub fn coverage(args: CoverageArgs) -> Result<()> {
    let writer = args.output.get_mt_writer()?;
    if args.inputs.is_multi() {
        let (reader_a, readers_b) = args.inputs.get_multi_readers()?;
        dispatch_pair_multi!(reader_a, readers_b, writer, args.params, run_coverage)
    } else {
        let (reader_a, reader_b) = args.inputs.get_readers()?;
        if let Some(t) = args.params.threads {
            if t < 2 {
                return dispatch_pair!(reader_a, reader_b, writer, args.params, run_coverage);
            }
            initialize_thread_pool(t)?;
            dispatch_pair!(reader_a, reader_b, writer, args.params, run_par_coverage)
        } else {
            dispatch_pair!(reader_a, reader_b, writer, args.params, run_coverage)
        }
    }
}
