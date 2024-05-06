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

fn join_sets_inner<'a, Ia, Ib, Na, Nb, W>(
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
    let pairs_iter = set_a.records().iter().flat_map(|iv| {
        let overlaps = set_b
            .query_iter(iv, query_method)
            .expect("Error in finding overlaps")
            .copied();
        overlaps.map(|ov| IntervalPair::new(*iv, ov, translater))
    });
    write_pairs_iter_with(pairs_iter, output, translater)
}

fn join_sets_left<'a, Ia, Ib, Na, Nb, W>(
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
    let pairs_iter = set_a.records().iter().flat_map(|iv| {
        let overlaps = set_b
            .query_iter(iv, query_method)
            .expect("Error in finding overlaps")
            .copied();

        // Peek at the next item to see if there are overlaps or not
        let mut peek = overlaps.peekable();
        let adjusted_overlap = if peek.peek().is_none() {
            let null_pair = IntervalPair::from_option(Some(*iv), None, translater);
            let new_iter = std::iter::once(null_pair);
            Box::new(new_iter) as Box<dyn Iterator<Item = IntervalPair<Ia, Ib, Na, Nb>>>
        } else {
            let new_iter = peek.map(|ov| IntervalPair::new(*iv, ov, translater));
            Box::new(new_iter) as Box<dyn Iterator<Item = IntervalPair<Ia, Ib, Na, Nb>>>
        };
        adjusted_overlap
    });
    write_pairs_iter_with(pairs_iter, output, translater)
}

fn join_sets_right<'a, Ia, Ib, Na, Nb, W>(
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
    let pairs_iter = set_b.records().iter().flat_map(|iv| {
        let overlaps = set_a
            .query_iter(iv, query_method)
            .expect("Error in finding overlaps")
            .copied();

        // Peek at the next item to see if there are overlaps or not
        let mut peek = overlaps.peekable();
        let adjusted_overlap = if peek.peek().is_none() {
            let null_pair = IntervalPair::from_option(None, Some(*iv), translater);
            let new_iter = std::iter::once(null_pair);
            Box::new(new_iter) as Box<dyn Iterator<Item = IntervalPair<Ia, Ib, Na, Nb>>>
        } else {
            let new_iter = peek.map(|ov| IntervalPair::new(ov, *iv, translater));
            Box::new(new_iter) as Box<dyn Iterator<Item = IntervalPair<Ia, Ib, Na, Nb>>>
        };
        adjusted_overlap
    });
    write_pairs_iter_with(pairs_iter, output, translater)
}

pub fn join(args: JoinArgs) -> Result<()> {
    let writer = args.output.get_writer()?;
    if args.inputs.is_multi() {
        let (bed_a, bed_b) = args.inputs.get_multi_readers()?;
        match args.params.how {
            JoinMethod::Inner => {
                dispatch_pair_multi!(bed_a, bed_b, writer, args.params, join_sets_inner)
            }
            JoinMethod::Left => {
                dispatch_pair_multi!(bed_a, bed_b, writer, args.params, join_sets_left)
            }
            JoinMethod::Right => {
                dispatch_pair_multi!(bed_a, bed_b, writer, args.params, join_sets_right)
            }
        }
    } else {
        let (bed_a, bed_b) = args.inputs.get_readers()?;
        match args.params.how {
            JoinMethod::Inner => dispatch_pair!(bed_a, bed_b, writer, args.params, join_sets_inner),
            JoinMethod::Left => dispatch_pair!(bed_a, bed_b, writer, args.params, join_sets_left),
            JoinMethod::Right => dispatch_pair!(bed_a, bed_b, writer, args.params, join_sets_right),
        }
    }
}
