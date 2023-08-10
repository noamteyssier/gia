use std::sync::Mutex;

use crate::{
    io::{match_input, match_output, read_name_map, read_set_with, FastaIndex, IndexedFasta},
    utils::setup_rayon,
};
use anyhow::{bail, Result};
use bedrs::{Container, Coordinates, GenomicIntervalSet};
use hashbrown::HashMap;
use rayon::prelude::*;

fn build_fasta_index(fasta: &str) -> Result<FastaIndex> {
    let index_path = format!("{}.fai", fasta);
    FastaIndex::from_filepath(&index_path)
}

fn build_null_map(set: &GenomicIntervalSet<usize>) -> HashMap<usize, String> {
    let mut map = HashMap::new();
    for iv in set.records() {
        if !map.contains_key(&iv.chr()) {
            map.insert(iv.chr(), format!("{}", iv.chr()));
        }
    }
    map
}

pub fn get_fasta(
    bed: Option<String>,
    fasta: &str,
    output: Option<String>,
    name_map: Option<String>,
    threads: Option<usize>,
    named: bool,
) -> Result<()> {
    setup_rayon(threads)?;
    let handle = match_input(bed)?;
    let (set, name_index) = read_set_with(handle, named)?;
    let fasta_index = build_fasta_index(fasta)?;
    let fasta = IndexedFasta::new(fasta_index, fasta.to_string());
    let output = match_output(output)?;
    let mutex = Mutex::new(output);

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

    set.records()
        .into_par_iter()
        .filter(|iv| iv.start() != iv.end())
        .map(|iv| {
            let name = name_index.get(&iv.chr()).unwrap();
            let seq = fasta
                .query(name, iv.start(), iv.end())
                .expect("Could not query interval");
            let seq_str = String::from_utf8(seq).expect("Could not convert sequence to string");
            (name, iv.start(), iv.end(), seq_str)
        })
        .for_each(|(name, start, end, seq)| {
            let mut guarded_output = mutex.lock().expect("Could not lock output");
            guarded_output
                .write_fmt(format_args!(">{}:{}-{}\n{}\n", name, start, end, seq))
                .expect("Could not write to output");
        });

    Ok(())
}
