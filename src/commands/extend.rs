use crate::io::{match_input, match_output, read_bed3_set, read_genome, write_records_iter_with};
use anyhow::Result;
use bedrs::{Container, Coordinates, GenomicInterval};
use hashbrown::HashMap;

fn extend_left(iv: &mut GenomicInterval<usize>, val: usize) {
    if iv.start() < val {
        iv.update_start(&0);
    } else {
        iv.extend_left(&val);
    }
}

fn extend_right(
    iv: &mut GenomicInterval<usize>,
    val: usize,
    genome: Option<&HashMap<usize, usize>>,
) {
    if let Some(ref genome) = genome {
        if let Some(end) = genome.get(iv.chr()) {
            if iv.end() + val > *end {
                iv.update_end(end);
            } else {
                iv.extend_right(&val);
            }
        } else {
            panic!("Chromosome {} not found in genome", iv.chr());
        }
    } else {
        iv.extend_right(&val);
    }
}

pub fn extend(
    input: Option<String>,
    output: Option<String>,
    both: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    genome_path: Option<String>,
    named: bool,
) -> Result<()> {
    let input_handle = match_input(input)?;
    let (mut iset, translater) = read_bed3_set(input_handle, named)?;
    let genome = if let Some(path) = genome_path {
        let genome_handle = match_input(Some(path))?;
        let genome = read_genome(genome_handle)?;
        Some(genome)
    } else {
        None
    };
    let extend_iter = iset.records_mut().into_iter().map(|iv| {
        if let Some(ref val) = both {
            extend_left(iv, *val);
            extend_right(iv, *val, genome.as_ref());
        } else {
            if let Some(ref val) = left {
                extend_left(iv, *val);
            }
            if let Some(ref val) = right {
                extend_right(iv, *val, genome.as_ref());
            }
        }
        *iv
    });

    let output_handle = match_output(output)?;
    write_records_iter_with(extend_iter, output_handle, translater.as_ref())?;
    Ok(())
}
