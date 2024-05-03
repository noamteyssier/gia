use crate::{
    cli::{JoinArgs, JoinMethod, JoinParams},
    dispatch_pair, dispatch_pair_multi,
    io::{write_pairs_iter_with, WriteNamedIter, WriteNamedIterImpl},
    types::{IntervalPair, Rename, Renamer, SplitTranslater},
    utils::sort_pairs,
};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, IntervalContainer};
use serde::Serialize;
use std::io::Write;

pub fn join_sets<'a, Ia, Ib, Na, Nb, W>(
    mut set_a: IntervalContainer<Ia, usize, usize>,
    mut set_b: IntervalContainer<Ib, usize, usize>,
    translater: Option<&'a SplitTranslater>,
    params: JoinParams,
    output: W,
) -> Result<()>
where
    Ia: IntervalBounds<usize, usize> + Copy + Serialize,
    Ib: IntervalBounds<usize, usize> + Copy + Serialize,
    Na: IntervalBounds<&'a str, usize> + Serialize,
    Nb: IntervalBounds<&'a str, usize> + Serialize,
    W: Write,
    WriteNamedIterImpl: WriteNamedIter<Ia> + WriteNamedIter<Ib>,
    Renamer: Rename<'a, Ia, Na> + Rename<'a, Ib, Nb>,
{
    let query_method = params.overlap_predicates.into();
    sort_pairs(&mut set_a, &mut set_b, params.sorted);
    match params.how {
        JoinMethod::Inner => {
            let pairs_iter = set_a.records().iter().flat_map(|iv| {
                let overlaps = set_b
                    .query_iter(iv, query_method)
                    .expect("Error in finding overlaps");
                overlaps.map(|ov| IntervalPair::new(*iv, *ov, translater))
            });
            write_pairs_iter_with(pairs_iter, output, translater)
        }
        _ => unimplemented!(),
    }
}

pub fn join(args: JoinArgs) -> Result<()> {
    let writer = args.output.get_writer()?;
    if args.inputs.is_multi() {
        let (bed_a, bed_b) = args.inputs.get_multi_readers()?;
        dispatch_pair_multi!(bed_a, bed_b, writer, args.params, join_sets)
    } else {
        let (bed_a, bed_b) = args.inputs.get_readers()?;
        dispatch_pair!(bed_a, bed_b, writer, args.params, join_sets)
    }
}
