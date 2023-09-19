use crate::{io::{match_input, match_output, read_set_with, write_set_with}, types::{Translater, Retranslater}};
use anyhow::Result;
use bedrs::{Container, Coordinates, GenomicIntervalSet};

pub fn reorder_chroms(set: &mut GenomicIntervalSet<usize>, translater: Translater) -> Retranslater {
    let retranslate = translater.lex_sort();
    set.apply_mut(|iv| {
        let new_chr = retranslate.get_rank(*iv.chr()).unwrap();
        iv.update_chr(&new_chr);
    });
    retranslate
}

pub fn sort(input: Option<String>, output: Option<String>, named: bool) -> Result<()> {
    let input_handle = match_input(input)?;
    let (mut set, translater) = read_set_with(input_handle, named)?;
    let translater = if let Some(t) = translater {
        Some(reorder_chroms(&mut set, t))
    } else {
        None
    };
    set.sort();
    let output_handle = match_output(output)?;
    write_set_with(&set, output_handle, translater.as_ref())?;
    Ok(())
}
