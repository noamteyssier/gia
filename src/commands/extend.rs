use crate::{
    cli::{ExtendArgs, Growth},
    dispatch_single,
    io::{write_records_iter_with, WriteNamedIter, WriteNamedIterImpl},
    types::{Genome, InputFormat, SplitTranslater, TranslateGroup},
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
    translater: Option<&SplitTranslater>,
    growth: Growth,
    output: W,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Serialize + Copy,
    W: Write,
    WriteNamedIterImpl: WriteNamedIter<I>,
{
    growth.warn_args();
    let genome = growth.get_genome(translater.map(|x| x.get_translater(TranslateGroup::Chr)))?;
    let extend_iter = set.into_iter().map(|mut iv| {
        let (left, right) = growth.get_values(&iv);
        extend_interval(&mut iv, left, right, genome.as_ref());
        iv
    });
    write_records_iter_with(extend_iter, output, translater)
}

pub fn extend(args: ExtendArgs) -> Result<()> {
    let reader = args.input.get_reader()?;
    let writer = args.output.get_writer()?;
    dispatch_single!(reader, writer, args.growth, extend_set)
}
