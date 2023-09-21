use crate::{
    io::{
        match_input, match_output, read_bed3_set, read_bed6_set, write_records_iter_with,
        WriteNamedIter, WriteNamedIterImpl,
    },
    types::{InputFormat, Reorder, Retranslater, Translater},
};
use anyhow::Result;
use bedrs::{traits::IntervalBounds, Container};
use serde::Serialize;

fn sort_set<I>(
    set: &mut impl Container<usize, usize, I>,
    translater: Option<Translater>,
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
    set.sort();
    translater
}

fn sort_and_write<I>(
    mut set: impl Container<usize, usize, I>,
    output: Option<String>,
    translater: Option<Translater>,
) -> Result<()>
where
    I: IntervalBounds<usize, usize> + Serialize + Reorder<I>,
    WriteNamedIterImpl: WriteNamedIter<I>,
{
    let translater = sort_set(&mut set, translater);
    let output_handle = match_output(output)?;
    write_records_iter_with(set.into_iter(), output_handle, translater.as_ref())?;
    Ok(())
}

fn sort_bed3(input: Option<String>, output: Option<String>, named: bool) -> Result<()> {
    let input_handle = match_input(input)?;
    let (set, translater) = read_bed3_set(input_handle, named)?;
    sort_and_write(set, output, translater)
}

fn sort_bed6(input: Option<String>, output: Option<String>, named: bool) -> Result<()> {
    let input_handle = match_input(input)?;
    let (set, translater) = read_bed6_set(input_handle, named)?;
    sort_and_write(set, output, translater)
}

pub fn sort(
    input: Option<String>,
    output: Option<String>,
    named: bool,
    format: InputFormat,
) -> Result<()> {
    match format {
        InputFormat::Bed3 => sort_bed3(input, output, named),
        InputFormat::Bed6 => sort_bed6(input, output, named),
    }
}
