use std::io::Write;

use crate::{
    io::{match_output, write_records_iter_with, BedReader, WriteNamedIter, WriteNamedIterImpl},
    types::{FieldFormat, Genome, InputFormat, Translater},
};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, IntervalContainer};
use serde::Serialize;

fn bound_value<I>(iv: I, val: i32, genome: Option<&Genome>) -> usize
where
    I: IntervalBounds<usize, usize>,
{
    let bound = if let Some(genome) = genome {
        let max = if let Some(max) = genome.chr_size(*iv.chr()) {
            max
        } else {
            panic!("Chromosome not found in genome: {}", iv.chr());
        };
        // If the value is greater than the chromosome size, return the chromosome size
        val.min(*max as i32)
    } else {
        val
    };

    // If the value is less than 0, return 1
    let bound = bound.max(1);

    bound as usize
}

fn shift_interval<I>(mut iv: I, amount: f64, percent: bool, genome: Option<&Genome>) -> I
where
    I: IntervalBounds<usize, usize> + Copy,
{
    let shift = if percent {
        let f_len = iv.f_len(amount.abs()) as i32;
        if amount.is_sign_positive() {
            f_len
        } else {
            -f_len
        }
    } else {
        amount as i32
    };
    let new_start = bound_value(iv, (iv.start() as i32) + shift, genome);
    let new_end = bound_value(iv, (iv.end() as i32) + shift, genome);

    iv.update_start(&new_start);
    iv.update_end(&new_end);
    iv
}

fn shift_set<I, W>(
    set: &IntervalContainer<I, usize, usize>,
    genome_path: Option<String>,
    translater: Option<&Translater>,
    amount: f64,
    percent: bool,
    output: W,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Serialize + Copy,
    W: Write,
    WriteNamedIterImpl: WriteNamedIter<I>,
{
    let genome = Genome::from_opt_path_immutable_with(genome_path, translater, false)?;
    let shift_iter = set
        .iter()
        .map(|iv| shift_interval(*iv, amount, percent, genome.as_ref()));
    write_records_iter_with(shift_iter, output, translater)
}

fn dispatch_shift<W: Write>(
    bed: BedReader,
    genome_path: Option<String>,
    amount: f64,
    percent: bool,
    output: W,
) -> Result<()> {
    let mut translater = bed.is_named().then_some(Translater::new());
    match bed.input_format() {
        InputFormat::Bed3 => {
            let set = bed.bed3_set_with(translater.as_mut())?;
            shift_set(
                &set,
                genome_path,
                translater.as_ref(),
                amount,
                percent,
                output,
            )
        }
        InputFormat::Bed6 => {
            let set = bed.bed6_set_with(translater.as_mut())?;
            shift_set(
                &set,
                genome_path,
                translater.as_ref(),
                amount,
                percent,
                output,
            )
        }
        InputFormat::Bed12 => {
            let set = bed.bed12_set_with(translater.as_mut())?;
            shift_set(
                &set,
                genome_path,
                translater.as_ref(),
                amount,
                percent,
                output,
            )
        }
    }
}

pub fn shift(
    input: Option<String>,
    output: Option<String>,
    genome_path: Option<String>,
    amount: f64,
    percent: bool,
    input_format: Option<InputFormat>,
    field_format: Option<FieldFormat>,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    let bed = BedReader::from_path(input, input_format, field_format)?;
    let output_handle = match_output(output, compression_threads, compression_level)?;
    if !percent && amount < 1.0 {
        eprintln!("Warning: Provided shift amount is less than 1.0 and percent is not set. This will shift intervals by the rounded integer value, which may not be the intended behavior. If you want to shift by a percentage, set the percent flag.");
    }
    dispatch_shift(bed, genome_path, amount, percent, output_handle)
}

#[cfg(test)]
mod testing {

    use super::*;
    use bedrs::{Bed12, Bed3, Bed6, Coordinates, Strand};

    #[test]
    fn test_calculate_percentage() {
        let iv = Bed3::new(1, 100, 200);
        assert_eq!(iv.f_len(0.5), 50);
        assert_eq!(iv.f_len(0.1), 10);
        assert_eq!(iv.f_len(0.0), 0);
        assert_eq!(iv.f_len(1.0), 100);
        assert_eq!(iv.f_len(1.5), 150);
    }

    #[test]
    fn test_shift_negative() {
        let iv = Bed3::new(1, 100, 200);
        let si = shift_interval(iv, -50.0, false, None);
        assert_eq!(si.start(), 50);
        assert_eq!(si.end(), 150);
    }

    #[test]
    fn test_shift_left_bounds() {
        let iv = Bed3::new(1, 100, 200);
        let si = shift_interval(iv, -150.0, false, None);
        assert_eq!(si.start(), 1);
        assert_eq!(si.end(), 50);
    }

    #[test]
    fn test_shift_right_bounds_end() {
        let iv = Bed3::new(1, 100, 200);
        let genome = Genome::from_params(1, 300);
        let si = shift_interval(iv, 150.0, false, Some(&genome));
        assert_eq!(si.start(), 250);
        assert_eq!(si.end(), 300);
    }

    #[test]
    fn test_shift_right_bounds_both() {
        let iv = Bed3::new(1, 100, 200);
        let genome = Genome::from_params(1, 300);
        let si = shift_interval(iv, 250.0, false, Some(&genome));
        assert_eq!(si.start(), 300);
        assert_eq!(si.end(), 300);
    }

    #[test]
    fn test_shift_fractional() {
        let iv = Bed3::new(1, 100, 200);
        let si = shift_interval(iv, 0.5, true, None);
        assert_eq!(si.start(), 150);
        assert_eq!(si.end(), 250);

        let si = shift_interval(iv, 0.1, true, None);
        assert_eq!(si.start(), 110);
        assert_eq!(si.end(), 210);

        let si = shift_interval(iv, -0.1, true, None);
        assert_eq!(si.start(), 90);
        assert_eq!(si.end(), 190);

        // let si = shift_interval(iv, 2.0, true, None);
        // assert_eq!(si.start(), 300);
        // assert_eq!(si.end(), 400);
    }

    #[test]
    fn test_shift_bed3() {
        let iv = Bed3::new(1, 100, 200);
        let si = shift_interval(iv, 50.0, false, None);
        assert_eq!(si.start(), 150);
        assert_eq!(si.end(), 250);
    }

    #[test]
    fn test_shift_bed6() {
        let iv = Bed6::new(1, 100, 200, 1, 2, Strand::default());
        let si = shift_interval(iv, 50.0, false, None);
        assert_eq!(si.start(), 150);
        assert_eq!(si.end(), 250);
        assert_eq!(si.name(), iv.name());
        assert_eq!(si.score(), iv.score());
        assert_eq!(si.strand(), iv.strand());
    }

    #[test]
    fn test_shift_bed12() {
        let iv = Bed12::new(1, 100, 400, 1, 2, Strand::default(), 3, 4, 5, 6, 7, 8);
        let si = shift_interval(iv, 50.0, false, None);
        assert_eq!(si.start(), 150);
        assert_eq!(si.end(), 450);
        assert_eq!(si.name(), iv.name());
        assert_eq!(si.score(), iv.score());
        assert_eq!(si.strand(), iv.strand());
        assert_eq!(si.thick_start(), iv.thick_start());
        assert_eq!(si.thick_end(), iv.thick_end());
        assert_eq!(si.item_rgb(), iv.item_rgb());
        assert_eq!(si.block_count(), iv.block_count());
        assert_eq!(si.block_sizes(), iv.block_sizes());
        assert_eq!(si.block_starts(), iv.block_starts());
    }
}
