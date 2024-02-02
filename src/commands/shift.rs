use std::io::Write;

use crate::{
    io::{match_output, write_records_iter_with, BedReader, WriteNamedIter, WriteNamedIterImpl},
    types::{FieldFormat, Genome, InputFormat, Translater},
};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, IntervalContainer};
use serde::Serialize;

fn calculate_percentage<I>(iv: I, val: f32) -> i32
where
    I: IntervalBounds<usize, usize>,
{
    let size = iv.len();
    (size as f32 * val) as i32
}

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

fn shift_interval<I>(mut iv: I, amount: f32, percent: bool, genome: Option<&Genome>) -> I
where
    I: IntervalBounds<usize, usize> + Copy,
{
    let shift = if percent {
        calculate_percentage(iv, amount)
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
    amount: f32,
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
    amount: f32,
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
    amount: f32,
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
