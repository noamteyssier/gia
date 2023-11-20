use crate::{
    io::{
        match_input, match_output, read_bed12_set, read_bed3_set, read_bed6_set,
        write_records_iter_with, WriteNamedIter, WriteNamedIterImpl,
    },
    types::{InputFormat, Reorder, Retranslater, Translater},
};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, Container};
use rayon::ThreadPoolBuilder;
use serde::Serialize;

fn sort_set<I>(
    set: &mut impl Container<usize, usize, I>,
    translater: Option<Translater>,
    parallel: bool,
) -> Option<Retranslater>
where
    I: IntervalBounds<usize, usize> + Reorder<I>,
{
    let translater = if let Some(translater) = translater {
        let retranslater = I::reorder_translater(set, translater);
        Some(retranslater)
    } else {
        None
    };
    if parallel {
        set.par_sort();
    } else {
        set.sort();
    }
    translater
}

fn sort_and_write<I>(
    mut set: impl Container<usize, usize, I>,
    output: Option<String>,
    translater: Option<Translater>,
    parallel: bool,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Serialize + Reorder<I>,
    WriteNamedIterImpl: WriteNamedIter<I>,
{
    let translater = sort_set(&mut set, translater, parallel);
    let output_handle = match_output(output, compression_threads, compression_level)?;
    write_records_iter_with(set.into_iter(), output_handle, translater.as_ref())?;
    Ok(())
}

fn sort_bed3(
    input: Option<String>,
    output: Option<String>,
    named: bool,
    parallel: bool,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    let input_handle = match_input(input)?;
    let (set, translater) = read_bed3_set(input_handle, named)?;
    sort_and_write(
        set,
        output,
        translater,
        parallel,
        compression_threads,
        compression_level,
    )
}

fn sort_bed6(
    input: Option<String>,
    output: Option<String>,
    named: bool,
    parallel: bool,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    let input_handle = match_input(input)?;
    let (set, translater) = read_bed6_set(input_handle, named)?;
    sort_and_write(
        set,
        output,
        translater,
        parallel,
        compression_threads,
        compression_level,
    )
}

fn sort_bed12(
    input: Option<String>,
    output: Option<String>,
    named: bool,
    parallel: bool,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    let input_handle = match_input(input)?;
    let (set, translater) = read_bed12_set(input_handle, named)?;
    sort_and_write(
        set,
        output,
        translater,
        parallel,
        compression_threads,
        compression_level,
    )
}

fn initialize_thread_pool(threads: usize) -> Result<bool> {
    if threads > 1 {
        ThreadPoolBuilder::new()
            .num_threads(threads as usize)
            .build_global()
            .unwrap();
        Ok(true)
    } else if threads == 0 {
        // by default, rayon uses all available cores
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn sort(
    input: Option<String>,
    output: Option<String>,
    named: bool,
    format: InputFormat,
    threads: usize,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    let parallel = initialize_thread_pool(threads)?;
    match format {
        InputFormat::Bed3 => sort_bed3(
            input,
            output,
            named,
            parallel,
            compression_threads,
            compression_level,
        ),
        InputFormat::Bed6 => sort_bed6(
            input,
            output,
            named,
            parallel,
            compression_threads,
            compression_level,
        ),
        InputFormat::Bed12 => sort_bed12(
            input,
            output,
            named,
            parallel,
            compression_threads,
            compression_level,
        ),
    }
}
