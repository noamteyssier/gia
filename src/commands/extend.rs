use crate::{
    cli::{ExtendArgs, Growth},
    io::{write_records_iter_with, BedReader, WriteNamedIter, WriteNamedIterImpl},
    types::{Genome, InputFormat, Translater},
};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, IntervalContainer};
use serde::Serialize;
use std::io::Write;

fn extend_interval<I>(iv: &mut I, left: usize, right: usize, genome: Option<&Genome>)
where
    I: IntervalBounds<usize, usize>,
{
    if left != 0 {
        iv.extend_left(&left);
    }
    if right != 0 {
        if let Some(genome) = genome {
            let end = genome.chr_size(*iv.chr()).copied();
            iv.extend_right(&right, end);
        } else {
            iv.extend_right(&right, None);
        }
    }
}

fn extend_set<I, W>(
    set: IntervalContainer<I, usize, usize>,
    translater: Option<&Translater>,
    growth: &Growth,
    output: W,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Serialize + Copy,
    W: Write,
    WriteNamedIterImpl: WriteNamedIter<I>,
{
    growth.warn_args();
    let genome = growth.get_genome(translater)?;
    let extend_iter = set.into_iter().map(|mut iv| {
        let (left, right) = growth.get_values(&iv);
        extend_interval(&mut iv, left, right, genome.as_ref());
        iv
    });
    write_records_iter_with(extend_iter, output, translater)
}

fn dispatch_extend<W: Write>(bed_reader: BedReader, output: W, growth: &Growth) -> Result<()> {
    match bed_reader.input_format() {
        InputFormat::Bed3 => {
            let (iset, translater) = bed_reader.bed3_set()?;
            extend_set(iset, translater.as_ref(), growth, output)
        }
        InputFormat::Bed6 => {
            let (iset, translater) = bed_reader.bed6_set()?;
            extend_set(iset, translater.as_ref(), growth, output)
        }
        InputFormat::Bed12 => {
            let (iset, translater) = bed_reader.bed12_set()?;
            extend_set(iset, translater.as_ref(), growth, output)
        }
    }
}

pub fn extend(args: ExtendArgs) -> Result<()> {
    let reader = args.input.get_reader()?;
    let output = args.output.get_handle()?;
    dispatch_extend(reader, output, &args.growth)
}
