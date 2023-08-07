use crate::io::{match_input, match_output, read_set, write_set};
use anyhow::Result;
use bedrs::Container;

pub fn sort(input: Option<String>, output: Option<String>) -> Result<()> {
    let input_handle = match_input(input)?;
    let mut set = read_set(input_handle)?;
    set.sort();
    let output_handle = match_output(output)?;
    write_set(&set, output_handle)?;
    Ok(())
}
