use crate::{
    io::{
        match_input, match_output, read_bed12_set, read_bed3_set, read_bed6_set,
        write_records_iter_with, BedReader, WriteNamedIter, WriteNamedIterImpl,
    },
    types::{FieldFormat, Genome, InputFormat, Translater},
};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, Coordinates, IntervalContainer};
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
    genome_path: Option<String>,
    output: Option<String>,
    set: &mut IntervalContainer<I, usize, usize>,
    both: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    translater: Option<&Translater>,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Serialize + Copy,
    WriteNamedIterImpl: WriteNamedIter<I>,
{
    let genome = if let Some(path) = genome_path {
        let genome_handle = match_input(Some(path))?;
        let genome = Genome::from_reader_immutable(genome_handle, translater, false)?;
        Some(genome)
    } else {
        None
    };
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
    let output_handle = match_output(output, compression_threads, compression_level)?;
    write_records_iter_with(extend_iter, output_handle, translater)?;
    Ok(())
}

fn match_and_extend(
    bed_reader: BedReader,
    output: Option<String>,
    both: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    genome_path: Option<String>,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    let named = bed_reader.is_named();
    match bed_reader.input_format() {
        InputFormat::Bed3 => {
            let (mut iset, translater) = read_bed3_set(bed_reader.reader(), named)?;
            extend_set(
                genome_path,
                output,
                &mut iset,
                both,
                left,
                right,
                translater.as_ref(),
                compression_threads,
                compression_level,
            )?;
        }
        InputFormat::Bed6 => {
            let (mut iset, translater) = read_bed6_set(bed_reader.reader(), named)?;
            extend_set(
                genome_path,
                output,
                &mut iset,
                both,
                left,
                right,
                translater.as_ref(),
                compression_threads,
                compression_level,
            )?;
        }
        InputFormat::Bed12 => {
            let (mut iset, translater) = read_bed12_set(bed_reader.reader(), named)?;
            extend_set(
                genome_path,
                output,
                &mut iset,
                both,
                left,
                right,
                translater.as_ref(),
                compression_threads,
                compression_level,
            )?;
        }
    }
    Ok(())
}

pub fn extend(
    input: Option<String>,
    output: Option<String>,
    both: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    genome_path: Option<String>,
    input_format: Option<InputFormat>,
    field_format: Option<FieldFormat>,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    let bed_reader = BedReader::from_path(input, input_format, field_format)?;
    match_and_extend(
        bed_reader,
        output,
        both,
        left,
        right,
        genome_path,
        compression_threads,
        compression_level,
    )
}
