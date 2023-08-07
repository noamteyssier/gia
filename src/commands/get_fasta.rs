use std::sync::Mutex;

use crate::{
    io::{match_input, match_output, read_name_map, read_set, FastaIndex, IndexedFasta},
    utils::setup_rayon,
};
use anyhow::Result;
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
) -> Result<()> {
    setup_rayon(threads)?;
    let handle = match_input(bed)?;
    let set = read_set(handle)?;
    let fasta_index = build_fasta_index(fasta)?;
    let fasta = IndexedFasta::new(fasta_index, fasta.to_string());
    let output = match_output(output)?;
    let mutex = Mutex::new(output);
    let name_map = if let Some(path) = name_map {
        let handle = match_input(Some(path))?;
        let map = read_name_map(handle)?;
        map
    } else {
        build_null_map(&set)
    };

    set.records()
        .into_par_iter()
        .filter(|iv| iv.start() != iv.end())
        .map(|iv| {
            let name = name_map.get(&iv.chr()).unwrap();
            let seq = fasta
                .query(&name, iv.start(), iv.end())
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
