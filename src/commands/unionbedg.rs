use crate::{
    cli::{UnionBedGraphArgs, UnionBedGraphParams},
    io::{build_writer, write_segment, BedReader},
    types::{BedGraphSet, NumericBed3, SplitTranslater},
};
use anyhow::Result;
use bedrs::IntervalContainer;
use std::io::Write;

fn calculate_segment_scores(sets: &[BedGraphSet], segment: NumericBed3, scores: &mut Vec<f64>) {
    // Calculate the scores for each set
    for set in sets.iter() {
        let score = set
            .find_iter_sorted_unchecked(&segment)
            .map(|ix| ix.score())
            .sum::<f64>();
        scores.push(score);
    }
}

fn run_unionbedgraph<W: Write>(
    mut sets: Vec<BedGraphSet>,
    translater: Option<&SplitTranslater>,
    params: UnionBedGraphParams,
    writer: W,
) -> Result<()> {
    // Sort the sets
    if !params.sorted {
        for set in sets.iter_mut() {
            set.sort();
        }
    } else {
        for set in sets.iter_mut() {
            set.set_sorted();
        }
    }

    // Create a union of the sets as Bed3 records
    let mut union: IntervalContainer<NumericBed3, usize, usize> = IntervalContainer::empty();
    for set in sets.iter() {
        for record in set.records() {
            union.insert((*record).into());
        }
    }

    // Sort the union
    union.sort();

    // Segment the union
    let segments = union.segment()?;

    // Initialize the writer
    let mut wtr = build_writer(writer);

    // Initialize the scores vector
    let mut scores = vec![];

    // Write the segments
    for segment in segments.records() {
        // Clear the scores for each new segment
        scores.clear();

        // Calculate the scores for each set
        calculate_segment_scores(&sets, *segment, &mut scores);

        // Write the segment
        write_segment(&mut wtr, translater, *segment, &scores)?;
    }
    wtr.flush()?;
    Ok(())
}

fn dispatch_unionbedgraph<W: Write>(
    readers: Vec<BedReader>,
    writer: W,
    params: UnionBedGraphParams,
) -> Result<()> {
    let mut translater = readers[0].is_named().then_some(SplitTranslater::new());
    let mut sets = vec![];
    for reader in readers {
        let set = reader.bedgraph_set_with(translater.as_mut())?;
        sets.push(set);
    }
    run_unionbedgraph(sets, translater.as_ref(), params, writer)
}

pub fn unionbedgraph(args: UnionBedGraphArgs) -> Result<()> {
    let readers = args.inputs.get_readers()?;
    let writer = args.output.get_writer()?;
    dispatch_unionbedgraph(readers, writer, args.params)
}
