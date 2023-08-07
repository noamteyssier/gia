use crate::io::{match_input, match_output, read_set, write_records_iter};
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
    let mut iset = read_set(input_handle)?;
    let extend_iter = iset.records_mut().into_iter().map(|iv| {
        if let Some(ref val) = both {
            iv.extend_left(val);
            iv.extend_right(val);
        } else {
            if let Some(ref val) = left {
                iv.extend_left(val);
            }
            if let Some(ref val) = right {
                iv.extend_right(val);
            }
        }
        *iv
    });
    let output_handle = match_output(output)?;
    write_records_iter(extend_iter, output_handle)?;
    Ok(())
}
