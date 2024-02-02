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
    if left.is_some() && right.is_some() {
        Box::new([left.unwrap(), right.unwrap()].into_iter())
    } else if left.is_some() {
        Box::new(std::iter::once(left.unwrap()))
    } else {
        Box::new(std::iter::once(right.unwrap()))
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
    both: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
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
            flank_interval(*iv, *val, *val, genome.as_ref())
        } else {
            let left = left.unwrap_or(0);
            let right = right.unwrap_or(0);
            flank_interval(*iv, left, right, genome.as_ref())
        }
    });
    write_records_iter_with(flank_iter, output, translater)
}

/// Flank the intervals in the set but first match the input formats
pub fn dispatch_flank<W: Write>(
    bed: BedReader,
    genome_path: Option<String>,
    both: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
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
                output,
            )
        }
    }
}

pub fn flank(
    input: Option<String>,
    output: Option<String>,
    genome_path: Option<String>,
    both: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    input_format: Option<InputFormat>,
    field_format: Option<FieldFormat>,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    let bed = BedReader::from_path(input, input_format, field_format)?;
    let output_handle = match_output(output, compression_threads, compression_level)?;
    dispatch_flank(bed, genome_path, both, left, right, output_handle)
}
