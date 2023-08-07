use crate::io::{match_input, match_output, read_set, write_records};
use anyhow::Result;
use bedrs::{Container, Merge};

pub fn merge(input: Option<String>, output: Option<String>) -> Result<()> {
    let input_handle = match_input(input)?;
    let mut set = read_set(input_handle)?;
    set.sort();
    let merged = set.merge()?;
    let output_handle = match_output(output)?;
    write_records(merged.records(), output_handle)?;
    Ok(())
}
