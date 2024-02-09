use crate::{
    cli::{SubtractArgs, SubtractParams},
    dispatch_pair,
    io::{write_records_iter_with, WriteNamedIter, WriteNamedIterImpl},
    types::{InputFormat, SplitTranslater},
    utils::sort_pairs,
};
use anyhow::Result;
use bedrs::{
    traits::{ChromBounds, IntervalBounds, ValueBounds},
    types::QueryMethod,
    Coordinates, IntervalContainer, Subtract,
};
use serde::Serialize;
use std::{fmt::Debug, io::Write};

fn queued_diff<It, Ia, Ib, C, T>(query: &Ia, overlaps: It) -> Box<dyn Iterator<Item = Ia>>
where
    It: Iterator<Item = Ib>,
    Ia: IntervalBounds<C, T> + Copy + 'static + Debug,
    Ib: IntervalBounds<C, T> + Copy + 'static + Debug,
    C: ChromBounds,
    T: ValueBounds,
{
    let mut differences = Vec::new();
    let mut num_overlaps = 0;
    for ov in overlaps {
        if differences.is_empty() {
            differences.extend(query.subtract_iter(&ov));
        } else {
            let last_difference = differences.pop().unwrap();
            differences.extend(last_difference.subtract_iter(&ov));
        }
        num_overlaps += 1;
    }
    if num_overlaps == 0 {
        Box::new(std::iter::once(*query))
    } else {
        Box::new(differences.into_iter())
    }
}

fn iter_subtraction<'a, Ia, Ib, C, T>(
    aset: &'a IntervalContainer<Ia, C, T>,
    bset: &'a IntervalContainer<Ib, C, T>,
    method: QueryMethod<T>,
) -> Box<dyn Iterator<Item = Ia> + 'a>
where
    Ia: IntervalBounds<C, T> + Copy + 'static + Debug,
    Ib: IntervalBounds<C, T> + Copy + 'static + Debug,
    C: ChromBounds,
    T: ValueBounds,
{
    let sub_iter = aset.records().iter().flat_map(move |iv| {
        let overlaps = bset
            .find_iter_sorted_method_unchecked(iv, method)
            .expect("Error in finding overlaps")
            .copied();
        queued_diff(iv, overlaps)
    });
    Box::new(sub_iter)
}

fn run_subtract<Ia, Ib, C, T, W>(
    mut aset: IntervalContainer<Ia, C, T>,
    mut bset: IntervalContainer<Ib, C, T>,
    translater: Option<&SplitTranslater>,
    params: SubtractParams,
    writer: W,
) -> Result<()>
where
    Ia: IntervalBounds<C, T> + Copy + 'static + Coordinates<usize, usize> + Serialize + Debug,
    Ib: IntervalBounds<C, T> + Copy + 'static + Coordinates<usize, usize> + Serialize + Debug,
    C: ChromBounds,
    T: ValueBounds,
    W: Write,
    WriteNamedIterImpl: WriteNamedIter<Ia> + WriteNamedIter<Ib>,
{
    sort_pairs(&mut aset, &mut bset, false);
    let method = params.overlap_predicates.into();
    if params.unmerged {
        let sub_iter = iter_subtraction(&aset, &bset, method);
        write_records_iter_with(sub_iter, writer, translater)
    } else {
        let aset = aset.merge()?;
        let sub_iter = iter_subtraction(&aset, &bset, method);
        write_records_iter_with(sub_iter, writer, translater)
    }
}

pub fn subtract(args: SubtractArgs) -> Result<()> {
    let (reader_a, reader_b) = args.inputs.get_readers()?;
    let writer = args.output.get_writer()?;
    dispatch_pair!(reader_a, reader_b, writer, args.params, run_subtract)
}
