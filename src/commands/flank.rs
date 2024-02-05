use crate::{
    io::{match_output, write_records_iter_with, BedReader, WriteNamedIter, WriteNamedIterImpl},
    types::{FieldFormat, Genome, InputFormat, Translater},
};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, IntervalContainer};
use serde::Serialize;
use std::io::Write;

/// Dispatch the left and right flanking functions
fn flank_interval<'a, I>(
    iv: I,
    val_left: f64,
    val_right: f64,
    percent: bool,
    genome: Option<&Genome>,
) -> impl Iterator<Item = I> + 'a
where
    I: IntervalBounds<usize, usize> + Copy + 'a,
{
    let (left, right) = if percent {
        (iv.f_len(val_left), iv.f_len(val_right))
    } else {
        (val_left as usize, val_right as usize)
    };

    let left = left_flank(iv, left);
    let right = right_flank(iv, right, genome);
    flank_iterator(left, right)
}

/// Combine left and right flanks into a single iterator
fn flank_iterator<'a, I>(left: Option<I>, right: Option<I>) -> Box<dyn Iterator<Item = I> + 'a>
where
    I: IntervalBounds<usize, usize> + Copy + 'a,
{
    if left.is_some() && right.is_some() {
        Box::new([left.unwrap(), right.unwrap()].into_iter())
    } else if left.is_some() {
        Box::new(std::iter::once(left.unwrap()))
    } else if right.is_some() {
        Box::new(std::iter::once(right.unwrap()))
    } else {
        Box::new(std::iter::empty())
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
    let mut left = iv.clone();
    let left_pos = iv.start().saturating_sub(val).max(1); // 1-based coordinates
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
    let mut right = iv.clone();
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
    set: &IntervalContainer<I, usize, usize>,
    genome_path: Option<String>,
    translater: Option<&Translater>,
    both: Option<f64>,
    left: Option<f64>,
    right: Option<f64>,
    percent: bool,
    output: W,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Serialize + Copy,
    W: Write,
    WriteNamedIterImpl: WriteNamedIter<I>,
{
    let genome = Genome::from_opt_path_immutable_with(genome_path, translater, false)?;
    let flank_iter = set.iter().flat_map(|iv| {
        if let Some(ref val) = both {
            flank_interval(*iv, *val, *val, percent, genome.as_ref())
        } else {
            let left = left.unwrap_or(0.0);
            let right = right.unwrap_or(0.0);
            flank_interval(*iv, left, right, percent, genome.as_ref())
        }
    });
    write_records_iter_with(flank_iter, output, translater)
}

/// Flank the intervals in the set but first match the input formats
pub fn dispatch_flank<W: Write>(
    bed: BedReader,
    genome_path: Option<String>,
    both: Option<f64>,
    left: Option<f64>,
    right: Option<f64>,
    percent: bool,
    output: W,
) -> Result<()> {
    let mut translater = bed.is_named().then_some(Translater::new());
    match bed.input_format() {
        InputFormat::Bed3 => {
            let set = bed.bed3_set_with(translater.as_mut())?;
            flank_set(
                &set,
                genome_path,
                translater.as_ref(),
                both,
                left,
                right,
                percent,
                output,
            )
        }
        InputFormat::Bed6 => {
            let set = bed.bed6_set_with(translater.as_mut())?;
            flank_set(
                &set,
                genome_path,
                translater.as_ref(),
                both,
                left,
                right,
                percent,
                output,
            )
        }
        InputFormat::Bed12 => {
            let set = bed.bed12_set_with(translater.as_mut())?;
            flank_set(
                &set,
                genome_path,
                translater.as_ref(),
                both,
                left,
                right,
                percent,
                output,
            )
        }
    }
}

pub fn flank(
    input: Option<String>,
    output: Option<String>,
    genome_path: Option<String>,
    both: Option<f64>,
    left: Option<f64>,
    right: Option<f64>,
    percent: bool,
    input_format: Option<InputFormat>,
    field_format: Option<FieldFormat>,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    let bed = BedReader::from_path(input, input_format, field_format)?;
    let output_handle = match_output(output, compression_threads, compression_level)?;
    dispatch_flank(bed, genome_path, both, left, right, percent, output_handle)
}

#[cfg(test)]
mod testing {

    use super::*;
    use bedrs::{Bed12, Bed3, Bed6, Coordinates, Strand};

    #[test]
    fn test_flank_left_bounds() {
        let iv = Bed3::new(1, 10, 400);
        let left = left_flank(iv, 50).unwrap();
        assert_eq!(left.start(), 1);
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
        let mut it = flank_interval(iv, 50.0, 50.0, false, None);
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
        let mut it = flank_interval(iv, 0.0, 50.0, false, None);
        let right = it.next().unwrap();
        assert_eq!(right.start(), 400);
        assert_eq!(right.end(), 450);
        assert!(it.next().is_none());
    }

    #[test]
    fn test_flank_both_no_right() {
        let iv = Bed3::new(1, 100, 400);
        let mut it = flank_interval(iv, 50.0, 0.0, false, None);
        let left = it.next().unwrap();
        assert_eq!(left.start(), 50);
        assert_eq!(left.end(), 100);
        assert!(it.next().is_none());
    }

    #[test]
    fn test_flank_none() {
        let iv = Bed3::new(1, 100, 400);
        let mut it = flank_interval(iv, 0.0, 0.0, false, None);
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

    #[test]
    fn test_flank_both_percentage() {
        let iv = Bed3::new(1, 100, 400);
        let mut it = flank_interval(iv, 0.25, 0.25, true, None);
        let left = it.next().unwrap();
        assert_eq!(left.start(), 25);
        assert_eq!(left.end(), 100);
        let right = it.next().unwrap();
        assert_eq!(right.start(), 400);
        assert_eq!(right.end(), 475);
        assert!(it.next().is_none());
    }

    #[test]
    fn test_flank_both_percentage_lt_gt() {
        let iv = Bed3::new(1, 100, 400);
        let mut it = flank_interval(iv, 0.25, 1.25, true, None);
        let left = it.next().unwrap();
        assert_eq!(left.start(), 25);
        assert_eq!(left.end(), 100);
        let right = it.next().unwrap();
        assert_eq!(right.start(), 400);
        assert_eq!(right.end(), 775);
        assert!(it.next().is_none());
    }
}
