use super::iter::{run_function_query, run_function_target};
use crate::{
    cli::{IntersectArgs, IntersectParams, OutputMethod},
    dispatch_pair, dispatch_pair_multi,
    io::{
        build_reader, write_named_records_iter_dashmap, write_records_iter_with, NamedIter,
        UnnamedIter, WriteNamedIter, WriteNamedIterImpl,
    },
    types::{NumericBed3, SplitTranslater, StreamTranslater},
    utils::sort_pairs,
};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, IntersectIter, IntervalContainer, MergeIter};
use serde::Serialize;
use std::io::Write;

pub fn intersect_sets<Ia, Ib, W>(
    mut set_a: IntervalContainer<Ia, usize, usize>,
    mut set_b: IntervalContainer<Ib, usize, usize>,
    translater: Option<&SplitTranslater>,
    params: IntersectParams,
    writer: W,
) -> Result<()>
where
    Ia: IntervalBounds<usize, usize> + Copy + Serialize,
    Ib: IntervalBounds<usize, usize> + Copy + Serialize,
    W: Write,
    WriteNamedIterImpl: WriteNamedIter<Ia> + WriteNamedIter<Ib>,
{
    let query_method = params.overlap_predicates.into();
    let output_method = params.output_predicates.try_into()?;
    sort_pairs(&mut set_a, &mut set_b, params.sorted);
    match output_method {
        // Output the target intervals
        OutputMethod::Target => {
            let ix_iter = set_a.records().iter().flat_map(|iv| {
                let overlaps = set_b
                    .query_iter(iv, query_method)
                    .expect("Error in finding overlaps")
                    .copied();
                let intersections = run_function_target(overlaps, output_method);
                intersections
            });
            write_records_iter_with(ix_iter, writer, translater)
        }
        // Output the query intervals with various formats
        _ => {
            let ix_iter = set_a.records().iter().flat_map(|iv| {
                let overlaps = set_b
                    .query_iter(iv, query_method)
                    .expect("Error in finding overlaps")
                    .copied();
                let intersections = run_function_query(iv, overlaps, output_method);
                intersections
            });
            write_records_iter_with(ix_iter, writer, translater)
        }
    }
}

pub fn intersect(args: IntersectArgs) -> Result<()> {
    if args.params.stream {
        intersect_stream(args)
    } else {
        let writer = args.output.get_writer()?;
        if args.inputs.is_multi() {
            let (bed_a, bed_b) = args.inputs.get_multi_readers()?;
            dispatch_pair_multi!(bed_a, bed_b, writer, args.params, intersect_sets)
        } else {
            let (bed_a, bed_b) = args.inputs.get_readers()?;
            dispatch_pair!(bed_a, bed_b, writer, args.params, intersect_sets)
        }
    }
}

fn intersect_stream(args: IntersectArgs) -> Result<()> {
    let (bed_a, bed_b) = args.inputs.get_readers()?;
    let named = bed_a.is_named();
    let query_handle = bed_a.reader();
    let target_handle = bed_b.reader();
    let mut query_csv = build_reader(query_handle);
    let mut target_csv = build_reader(target_handle);
    let writer = args.output.get_writer()?;
    let method = args.params.overlap_predicates.into();

    if named {
        let translater = StreamTranslater::new();
        let query_iter: NamedIter<'_, '_, _, NumericBed3> =
            NamedIter::new(&mut query_csv, &translater);
        let target_iter = NamedIter::new(&mut target_csv, &translater);
        let merged_query_iter = MergeIter::new(query_iter);
        let merged_target_iter = MergeIter::new(target_iter);
        let intersect_iter =
            IntersectIter::new_with_method(merged_query_iter, merged_target_iter, method);
        write_named_records_iter_dashmap(intersect_iter, writer, &translater)?;
    } else {
        let query_iter: UnnamedIter<'_, _, NumericBed3> = UnnamedIter::new(&mut query_csv);
        let target_iter = UnnamedIter::new(&mut target_csv);
        let merged_query_iter = MergeIter::new(query_iter);
        let merged_target_iter = MergeIter::new(target_iter);
        let intersect_iter =
            IntersectIter::new_with_method(merged_query_iter, merged_target_iter, method);
        write_records_iter_with(intersect_iter, writer, None::<&SplitTranslater>)?;
    }
    Ok(())
}
