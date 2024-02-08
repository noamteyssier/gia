use crate::{
    cli::{FlankArgs, Growth},
    dispatch_single,
    io::{write_records_iter_with, WriteNamedIter, WriteNamedIterImpl},
    types::{Genome, InputFormat, Translater},
};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, IntervalContainer};
use serde::Serialize;
use std::io::Write;

/// Dispatch the left and right flanking functions
fn flank_interval<'a, I>(
    iv: I,
    val_left: usize,
    val_right: usize,
    genome: Option<&Genome>,
) -> impl Iterator<Item = I> + 'a
where
    I: IntervalBounds<usize, usize> + Copy + 'a,
{
    let left = left_flank(iv, val_left);
    let right = right_flank(iv, val_right, genome);
    flank_iterator(left, right)
}

/// Combine left and right flanks into a single iterator
fn flank_iterator<'a, I>(left: Option<I>, right: Option<I>) -> Box<dyn Iterator<Item = I> + 'a>
where
    I: IntervalBounds<usize, usize> + Copy + 'a,
{
    match (left, right) {
        (Some(left), Some(right)) => Box::new([left, right].into_iter()),
        (Some(left), None) => Box::new(std::iter::once(left)),
        (None, Some(right)) => Box::new(std::iter::once(right)),
        (None, None) => Box::new(std::iter::empty()),
    }
}

/// Perform the left flank - return None if the value is 0
fn left_flank<I>(iv: I, val: usize) -> Option<I>
where
    I: IntervalBounds<usize, usize> + Copy,
{
    if val == 0 {
        return None;
    }
    let mut left = iv;
    let left_pos = iv.start().saturating_sub(val).max(0);
    left.update_start(&left_pos);
    left.update_end(&iv.start());
    Some(left)
}

/// Perform the right flank - return None if the value is 0
fn right_flank<I>(iv: I, val: usize, genome: Option<&Genome>) -> Option<I>
where
    I: IntervalBounds<usize, usize> + Copy,
{
    if val == 0 {
        return None;
    }
    let mut right = iv;
    right.update_start(&iv.end());
    let end = if let Some(genome) = genome {
        let max = if let Some(max) = genome.chr_size(*iv.chr()) {
            max
        } else {
            panic!("Chromosome not found in genome: {}", iv.chr());
        };
        let end = iv.end() + val;
        end.min(*max)
    } else {
        iv.end() + val
    };
    right.update_end(&end);
    Some(right)
}

/// Flank the intervals in the set
fn flank_set<I, W>(
    set: IntervalContainer<I, usize, usize>,
    translater: Option<Translater>,
    growth: Growth,
    output: W,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Serialize + Copy,
    W: Write,
    WriteNamedIterImpl: WriteNamedIter<I>,
{
    growth.warn_args();
    let genome = growth.get_genome(translater.as_ref())?;
    let flank_iter = set.iter().flat_map(|iv| {
        let (left, right) = growth.get_values(iv);
        flank_interval(*iv, left, right, genome.as_ref())
    });
    write_records_iter_with(flank_iter, output, translater.as_ref())
}

pub fn flank(args: FlankArgs) -> Result<()> {
    let reader = args.input.get_reader()?;
    let writer = args.output.get_writer()?;
    dispatch_single!(reader, writer, args.growth, flank_set)
}

#[cfg(test)]
mod testing {

    use super::*;
    use bedrs::{Bed12, Bed3, Bed6, Coordinates, Strand};

    #[test]
    fn test_flank_left_bounds() {
        let iv = Bed3::new(1, 10, 400);
        let left = left_flank(iv, 50).unwrap();
        assert_eq!(left.start(), 0);
        assert_eq!(left.end(), 10);
    }

    #[test]
    fn test_flank_left_zero() {
        let iv = Bed3::new(1, 10, 400);
        let left = left_flank(iv, 0);
        assert!(left.is_none());
    }

    #[test]
    fn test_flank_left_bed3() {
        let iv = Bed3::new(1, 100, 400);
        let left = left_flank(iv, 50).unwrap();
        assert_eq!(left.start(), 50);
        assert_eq!(left.end(), 100);
    }

    #[test]
    fn test_flank_left_bed6() {
        let iv = Bed6::new(1, 100, 400, 1, 2, Strand::default());
        let left = left_flank(iv, 50).unwrap();
        assert_eq!(left.start(), 50);
        assert_eq!(left.end(), 100);
        assert_eq!(left.name(), iv.name());
        assert_eq!(left.score(), iv.score());
        assert_eq!(left.strand(), iv.strand());
    }

    #[test]
    fn test_flank_left_bed12() {
        let iv = Bed12::new(1, 100, 400, 1, 2, Strand::default(), 3, 4, 5, 6, 7, 8);
        let left = left_flank(iv, 50).unwrap();
        assert_eq!(left.start(), 50);
        assert_eq!(left.end(), 100);
        assert_eq!(left.name(), iv.name());
        assert_eq!(left.score(), iv.score());
        assert_eq!(left.strand(), iv.strand());
        assert_eq!(left.thick_start(), iv.thick_start());
        assert_eq!(left.thick_end(), iv.thick_end());
        assert_eq!(left.item_rgb(), iv.item_rgb());
        assert_eq!(left.block_count(), iv.block_count());
        assert_eq!(left.block_sizes(), iv.block_sizes());
        assert_eq!(left.block_starts(), iv.block_starts());
    }

    #[test]
    fn test_flank_right_bounds() {
        let genome = Genome::from_params(1, 100);
        let iv = Bed3::new(1, 10, 80);
        let right = right_flank(iv, 50, Some(&genome)).unwrap();
        assert_eq!(right.start(), 80);
        assert_eq!(right.end(), 100);
    }

    #[test]
    fn test_flank_right_zero() {
        let iv = Bed3::new(1, 10, 400);
        let right = right_flank(iv, 0, None);
        assert!(right.is_none());
    }

    #[test]
    fn test_flank_right_bed3() {
        let iv = Bed3::new(1, 100, 400);
        let right = right_flank(iv, 50, None).unwrap();
        assert_eq!(right.start(), 400);
        assert_eq!(right.end(), 450);
    }

    #[test]
    fn test_flank_right_bed6() {
        let iv = Bed6::new(1, 100, 400, 1, 2, Strand::default());
        let right = right_flank(iv, 50, None).unwrap();
        assert_eq!(right.start(), 400);
        assert_eq!(right.end(), 450);
        assert_eq!(right.name(), iv.name());
        assert_eq!(right.score(), iv.score());
        assert_eq!(right.strand(), iv.strand());
    }

    #[test]
    fn test_flank_right_bed12() {
        let iv = Bed12::new(1, 100, 400, 1, 2, Strand::default(), 3, 4, 5, 6, 7, 8);
        let right = right_flank(iv, 50, None).unwrap();
        assert_eq!(right.start(), 400);
        assert_eq!(right.end(), 450);
        assert_eq!(right.name(), iv.name());
        assert_eq!(right.score(), iv.score());
        assert_eq!(right.strand(), iv.strand());
        assert_eq!(right.thick_start(), iv.thick_start());
        assert_eq!(right.thick_end(), iv.thick_end());
        assert_eq!(right.item_rgb(), iv.item_rgb());
        assert_eq!(right.block_count(), iv.block_count());
        assert_eq!(right.block_sizes(), iv.block_sizes());
        assert_eq!(right.block_starts(), iv.block_starts());
    }

    #[test]
    fn test_flank_both() {
        let iv = Bed3::new(1, 100, 400);
        let mut it = flank_interval(iv, 50, 50, None);
        let left = it.next().unwrap();
        assert_eq!(left.start(), 50);
        assert_eq!(left.end(), 100);
        let right = it.next().unwrap();
        assert_eq!(right.start(), 400);
        assert_eq!(right.end(), 450);
        assert!(it.next().is_none());
    }

    #[test]
    fn test_flank_both_no_left() {
        let iv = Bed3::new(1, 100, 400);
        let mut it = flank_interval(iv, 0, 50, None);
        let right = it.next().unwrap();
        assert_eq!(right.start(), 400);
        assert_eq!(right.end(), 450);
        assert!(it.next().is_none());
    }

    #[test]
    fn test_flank_both_no_right() {
        let iv = Bed3::new(1, 100, 400);
        let mut it = flank_interval(iv, 50, 0, None);
        let left = it.next().unwrap();
        assert_eq!(left.start(), 50);
        assert_eq!(left.end(), 100);
        assert!(it.next().is_none());
    }

    #[test]
    fn test_flank_none() {
        let iv = Bed3::new(1, 100, 400);
        let mut it = flank_interval(iv, 0, 0, None);
        assert!(it.next().is_none());
    }

    #[test]
    fn test_pc_calculation_shrink() {
        let iv = Bed3::new(1, 100, 400);
        let pc = iv.f_len(0.5);
        assert_eq!(pc, 150);
    }

    #[test]
    fn test_pc_calculation_grow() {
        let iv = Bed3::new(1, 100, 400);
        let pc = iv.f_len(1.5);
        assert_eq!(pc, 450);
    }
}
