use anyhow::Result;
use bedrs::{traits::IntervalBounds, types::Query, IntervalContainer};
use serde::Serialize;
use std::io::Write;

use crate::{
    cli::{WindowArgs, WindowParams},
    dispatch_pair, dispatch_pair_multi,
    io::{write_pairs_iter_with, write_records_iter_with, WriteNamedIter, WriteNamedIterImpl},
    types::{InputFormat, IntervalPair, Rename, Renamer, SplitTranslater},
    utils::sort_pairs,
};

fn windowed_set_overlaps<'a, Ia, Ib, Na, Nb, W>(
    mut set_a: IntervalContainer<Ia, usize, usize>,
    mut set_b: IntervalContainer<Ib, usize, usize>,
    translater: Option<&'a SplitTranslater>,
    params: WindowParams,
    output: W,
) -> Result<()>
where
    Ia: IntervalBounds<usize, usize> + Serialize + Copy,
    Ib: IntervalBounds<usize, usize> + Serialize + Copy,
    Na: IntervalBounds<&'a str, usize> + Serialize,
    Nb: IntervalBounds<&'a str, usize> + Serialize,
    W: Write,
    WriteNamedIterImpl: WriteNamedIter<Ia> + WriteNamedIter<Ib>,
    Renamer: Rename<'a, Ia, Na> + Rename<'a, Ib, Nb>,
{
    sort_pairs(&mut set_a, &mut set_b, false);
    let method = Query::default();
    if params.inverse {
        let iv_iter = set_a
            .iter()
            .map(|iv| {
                let (left, right) = params.growth.get_values(iv);
                let mut w_iv = *iv;
                w_iv.extend_left(&left);
                w_iv.extend_right(&right, None);
                (iv, w_iv)
            })
            .filter(|(_iv, w_iv)| {
                let overlaps = set_b
                    .query_iter(w_iv, method)
                    .expect("Unexpected error")
                    .count();
                overlaps == 0
            })
            .map(|(iv, _w_iv)| *iv);
        write_records_iter_with(iv_iter, output, translater)?;
    } else {
        let windows_iter = set_a.iter().map(|iv| {
            let (left, right) = params.growth.get_values(iv);
            let mut w_iv = *iv;
            w_iv.extend_left(&left);
            w_iv.extend_right(&right, None);
            (iv, w_iv)
        });
        let pairs_iter = windows_iter.flat_map(|(iv, w_iv)| {
            let overlaps = set_b
                .query_iter_owned(w_iv, method)
                .expect("Unexpected error");
            overlaps.map(|ov| IntervalPair::new(*iv, *ov, translater))
        });
        write_pairs_iter_with(pairs_iter, output, translater)?;
    }
    Ok(())
}

pub fn window(args: WindowArgs) -> Result<()> {
    let writer = args.output.get_writer()?;
    if args.inputs.is_multi() {
        let (bed_a, bed_b) = args.inputs.get_multi_readers()?;
        dispatch_pair_multi!(bed_a, bed_b, writer, args.params, windowed_set_overlaps)
    } else {
        let (bed_a, bed_b) = args.inputs.get_readers()?;
        dispatch_pair!(bed_a, bed_b, writer, args.params, windowed_set_overlaps)
    }
}
