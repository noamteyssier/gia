use crate::io::{match_input, match_output, read_name_map, read_set_with};
use anyhow::{bail, Result};
use bedrs::{Container, Coordinates, GenomicIntervalSet};
use faiquery::{FastaIndex, IndexedFasta};
use hashbrown::HashMap;
use std::str::from_utf8;

fn build_fasta_index(fasta: &str) -> Result<FastaIndex> {
    let index_path = format!("{}.fai", fasta);
    FastaIndex::from_filepath(&index_path)
}

fn build_null_map(set: &GenomicIntervalSet<usize>) -> HashMap<usize, String> {
    let mut map = HashMap::new();
    for iv in set.records() {
        if !map.contains_key(iv.chr()) {
            map.insert(iv.chr().clone(), format!("{}", iv.chr()));
        }
    }
    map
}

pub fn get_fasta(
    bed: Option<String>,
    fasta: &str,
    output: Option<String>,
    name_map: Option<String>,
    named: bool,
) -> Result<()> {
    let handle = match_input(bed)?;
    let (set, name_index) = read_set_with(handle, named)?;
    let fasta_index = build_fasta_index(fasta)?;
    let mut fasta = IndexedFasta::new(fasta_index, fasta)?;
    let mut output = match_output(output)?;

    if name_index.is_some() && name_map.is_some() {
        bail!("Cannot use both name index and name map");
    }
    let name_index = if let Some(name_index) = name_index {
        name_index
    } else if let Some(oath) = name_map {
        let handle = match_input(Some(oath))?;
        read_name_map(handle)?
    } else {
        build_null_map(&set)
    };

    let iv_iter = set
        .records()
        .into_iter()
        .filter(|iv| iv.start() != iv.end());
    for iv in iv_iter {
        let name = name_index.get(iv.chr()).expect("Could not get name");
        let seq = fasta.query(name, iv.start(), iv.end())?;
        let seq_str = from_utf8(&seq)?;
        output.write_fmt(format_args!(
            ">{}:{}-{}\n{}\n",
            name,
            iv.start(),
            iv.end(),
            seq_str
        ))?
    }

    Ok(())
}
