use std::sync::Mutex;

use crate::{
    io::{match_input, match_output, read_set, FastaIndex, IndexedFasta},
    utils::setup_rayon,
};
use anyhow::Result;
use bedrs::{Container, Coordinates};
use rayon::prelude::*;

fn build_fasta_index(fasta: &str) -> Result<FastaIndex> {
    let index_path = format!("{}.fai", fasta);
    FastaIndex::from_filepath(&index_path)
}

pub fn get_fasta(
    bed: Option<String>,
    fasta: &str,
    output: Option<String>,
    threads: Option<usize>,
) -> Result<()> {
    setup_rayon(threads)?;
    let handle = match_input(bed)?;
    let set = read_set(handle)?;
    let fasta_index = build_fasta_index(fasta)?;
    let fasta = IndexedFasta::new(fasta_index, fasta.to_string());
    let output = match_output(output)?;
    let mutex = Mutex::new(output);

    set.records()
        .into_par_iter()
        .filter(|iv| iv.start() != iv.end())
        .map(|iv| {
            let name = format!("{}", iv.chr());
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
