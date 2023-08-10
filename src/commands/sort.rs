use crate::io::{match_input, match_output, read_set_with, write_set_with};
use anyhow::Result;
use bedrs::Container;

pub fn sort(input: Option<String>, output: Option<String>, named: bool) -> Result<()> {
    let input_handle = match_input(input)?;
    let (mut set, name_index) = read_set_with(input_handle, named)?;
    set.sort();
    let output_handle = match_output(output)?;
    write_set_with(&set, output_handle, name_index.as_ref())?;
    Ok(())
}
