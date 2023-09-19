use super::build_reader;
use anyhow::Result;
use hashbrown::HashMap;
use std::io::Read;

pub fn read_genome<R: Read>(reader: R) -> Result<HashMap<usize, usize>> {
    let mut reader = build_reader(reader);
    let map = reader
        .deserialize()
        .map(|record| {
            let record: (usize, usize) = record?;
            Ok(record)
        })
        .collect::<Result<HashMap<usize, usize>>>()?;
    Ok(map)
}
