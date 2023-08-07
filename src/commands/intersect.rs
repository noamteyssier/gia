use crate::io::{match_input, match_output, read_set, write_records};
use anyhow::{bail, Result};
use bedrs::{Container, Find, Intersect};

pub fn intersect(a: Option<String>, b: String, output: Option<String>) -> Result<()> {
    let a_handle = match_input(a)?;
    let b_handle = match_input(Some(b))?;
    let mut a_set = read_set(a_handle)?;
    let mut b_set = read_set(b_handle)?;
    a_set.sort();
    b_set.sort();

    let mut intersections = Vec::new();
    for iv in a_set.records() {
        let overlapping = b_set.find_iter_sorted_unchecked(iv);
        for ov in overlapping {
            let ix = match iv.intersect(ov) {
                Some(ix) => ix,
                None => bail!("Failed to intersect intervals: There may be a bug in FindIter"),
            };
            intersections.push(ix);
        }
    }
    let output_handle = match_output(output)?;
    write_records(&intersections, output_handle)?;
    Ok(())
}
