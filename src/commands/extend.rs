use crate::{
    io::{
        match_input, match_output, read_bed3_set, read_bed6_set, write_records_iter_with,
        WriteNamedIter, WriteNamedIterImpl,
    },
    types::{Genome, InputFormat, Translater},
};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, Container, Coordinates};
use serde::Serialize;

fn extend_left(iv: &mut impl Coordinates<usize, usize>, val: usize) {
    if iv.start() < val {
        iv.update_start(&0);
    } else {
        iv.extend_left(&val);
    }
}

fn extend_right(iv: &mut impl Coordinates<usize, usize>, val: usize, genome: Option<&Genome>) {
    if let Some(ref genome) = genome {
        if let Some(end) = genome.chr_size(*iv.chr()) {
            if iv.end() + val > *end {
                iv.update_end(end);
            } else {
                iv.extend_right(&val);
            }
        } else {
            panic!("Chromosome {} not found in genome", iv.chr());
        }
    } else {
        iv.extend_right(&val);
    }
}

fn extend_set<I>(
    output: Option<String>,
    set: &mut impl Container<usize, usize, I>,
    both: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    genome: Option<Genome>,
    translater: Option<&Translater>,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Serialize + Copy,
    WriteNamedIterImpl: WriteNamedIter<I>,
{
    let extend_iter = set.records_mut().into_iter().map(|iv| {
        if let Some(ref val) = both {
            extend_left(iv, *val);
            extend_right(iv, *val, genome.as_ref());
        } else {
            if let Some(ref val) = left {
                extend_left(iv, *val);
            }
            if let Some(ref val) = right {
                extend_right(iv, *val, genome.as_ref());
            }
        }
        *iv
    });
    let output_handle = match_output(output)?;
    write_records_iter_with(extend_iter, output_handle, translater)?;
    Ok(())
}

fn extend_bed3(
    input: Option<String>,
    output: Option<String>,
    both: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    genome_path: Option<String>,
    named: bool,
) -> Result<()> {
    let input_handle = match_input(input)?;
    let (mut iset, translater) = read_bed3_set(input_handle, named)?;
    let genome = if let Some(path) = genome_path {
        let genome_handle = match_input(Some(path))?;
        let genome = Genome::from_reader_immutable(genome_handle, translater.as_ref(), false)?;
        Some(genome)
    } else {
        None
    };
    extend_set(
        output,
        &mut iset,
        both,
        left,
        right,
        genome,
        translater.as_ref(),
    )?;
    Ok(())
}

fn extend_bed6(
    input: Option<String>,
    output: Option<String>,
    both: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    genome_path: Option<String>,
    named: bool,
) -> Result<()> {
    let input_handle = match_input(input)?;
    let (mut iset, translater) = read_bed6_set(input_handle, named)?;
    let genome = if let Some(path) = genome_path {
        let genome_handle = match_input(Some(path))?;
        let genome = Genome::from_reader_immutable(genome_handle, translater.as_ref(), false)?;
        Some(genome)
    } else {
        None
    };
    extend_set(
        output,
        &mut iset,
        both,
        left,
        right,
        genome,
        translater.as_ref(),
    )?;
    Ok(())
}

pub fn extend(
    input: Option<String>,
    output: Option<String>,
    both: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    genome_path: Option<String>,
    named: bool,
    format: InputFormat,
) -> Result<()> {
    match format {
        InputFormat::Bed3 => extend_bed3(input, output, both, left, right, genome_path, named),
        InputFormat::Bed6 => extend_bed6(input, output, both, left, right, genome_path, named),
    }
}
