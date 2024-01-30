use crate::{
    io::{
        match_output, read_bed12_set, read_bed3_set, read_bed6_set, write_records_iter_with,
        BedReader, WriteNamedIter, WriteNamedIterImpl,
    },
    types::{FieldFormat, InputFormat, Reorder, Retranslater, Translater},
};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, IntervalContainer};
use rayon::ThreadPoolBuilder;
use serde::Serialize;

fn sort_set<I>(
    set: &mut IntervalContainer<I, usize, usize>,
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
    mut set: IntervalContainer<I, usize, usize>,
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

fn match_and_sort(
    bed_reader: BedReader,
    output: Option<String>,
    parallel: bool,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    let named = bed_reader.is_named();
    match bed_reader.input_format() {
        InputFormat::Bed3 => {
            let (set, translater) = read_bed3_set(bed_reader.reader(), named)?;
            sort_and_write(
                set,
                output,
                translater,
                parallel,
                compression_threads,
                compression_level,
            )
        }
        InputFormat::Bed6 => {
            let (set, translater) = read_bed6_set(bed_reader.reader(), named)?;
            sort_and_write(
                set,
                output,
                translater,
                parallel,
                compression_threads,
                compression_level,
            )
        }
        InputFormat::Bed12 => {
            let (set, translater) = read_bed12_set(bed_reader.reader(), named)?;
            sort_and_write(
                set,
                output,
                translater,
                parallel,
                compression_threads,
                compression_level,
            )
        }
    }
}

pub fn sort(
    input: Option<String>,
    output: Option<String>,
    input_format: Option<InputFormat>,
    field_format: Option<FieldFormat>,
    threads: usize,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    let parallel = initialize_thread_pool(threads)?;
    let bed_reader = BedReader::from_path(input, input_format, field_format)?;
    match_and_sort(
        bed_reader,
        output,
        parallel,
        compression_threads,
        compression_level,
    )
}
