use crate::io::{match_input, match_output, read_set_with, write_records_with};
use anyhow::Result;
use bedrs::{Container, Merge};

pub fn merge(
    input: Option<String>,
    output: Option<String>,
    sorted: bool,
    named: bool,
) -> Result<()> {
    let input_handle = match_input(input)?;
    let (mut set, name_index) = read_set_with(input_handle, named)?;
    if !sorted {
        set.sort();
    } else {
        set.set_sorted();
    }
    let merged = set.merge()?;
    let output_handle = match_output(output)?;
    write_records_with(merged.records(), output_handle, name_index.as_ref())?;
    Ok(())
}
