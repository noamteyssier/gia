use crate::io::{match_input, match_output, read_set, write_set};
use anyhow::Result;
use bedrs::{Container, Coordinates};

pub fn extend(
    input: Option<String>,
    output: Option<String>,
    both: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
) -> Result<()> {
    let input_handle = match_input(input)?;
    let mut set = read_set(input_handle)?;
    if let Some(ext) = both {
        set.apply_mut(|iv| {
            iv.extend_left(&ext);
            iv.extend_right(&ext);
        })
    } else {
        set.apply_mut(|iv| {
            if let Some(val) = left {
                iv.extend_left(&val);
            }
            if let Some(val) = right {
                iv.extend_right(&val);
            }
        })
    }
    let output_handle = match_output(output)?;
    write_set(&set, output_handle)?;
    Ok(())
}
