use crate::io::{match_input, match_output};
use anyhow::Result;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NamedInterval {
    name: String,
    start: u64,
    end: u64,
}

pub fn name_map(input: Option<String>, output: Option<String>, map: Option<String>) -> Result<()> {
    let map_name = match map {
        Some(map_name) => map_name,
        None => "name_map.tsv".to_string(),
    };
    let input_handle = match_input(input)?;
    let mut csv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .comment(Some(b'#'))
        .from_reader(input_handle);

    let bed_output_handle = match_output(output)?;
    let map_output_handle = match_output(Some(map_name))?;
    let mut bed_writer = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_writer(bed_output_handle);

    let mut chr_map = HashMap::new();
    csv_reader.deserialize().for_each(|rec| {
        let rec: NamedInterval = rec.unwrap();
        if !chr_map.contains_key(&rec.name) {
            chr_map.insert(rec.name.clone(), 1 + chr_map.len() as u64);
        }
        let chr_index = chr_map.get(&rec.name).unwrap();
        bed_writer
            .serialize((chr_index, rec.start, rec.end))
            .unwrap();
    });
    bed_writer.flush()?;

    let mut map_writer = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_writer(map_output_handle);
    for (chr, index) in chr_map {
        map_writer.serialize((index, chr)).unwrap();
    }
    map_writer.flush()?;

    Ok(())
}
