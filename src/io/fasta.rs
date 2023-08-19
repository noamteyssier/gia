use std::{fs::File, io::Read};

use anyhow::{bail, Result};
use hashbrown::HashMap;
use memmap2::Mmap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexEntry {
    name: String,
    length: usize,
    offset: usize,
    line_bases: usize,
    line_width: usize,
}

#[derive(Debug)]
pub struct FastaIndex {
    entries: HashMap<String, IndexEntry>,
}
impl FastaIndex {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }
    pub fn insert(&mut self, entry: IndexEntry) {
        self.entries.insert(entry.name.clone(), entry);
    }
    pub fn from_reader<R: Read>(reader: R) -> Result<Self> {
        let mut csv_reader = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .has_headers(false)
            .from_reader(reader);
        let mut index = Self::new();
        for record in csv_reader.deserialize() {
            let record: IndexEntry = record?;
            index.insert(record);
        }
        Ok(index)
    }
    pub fn from_filepath(path: &str) -> Result<Self> {
        let file = File::open(path)?;
        Self::from_reader(file)
    }
    pub fn get(&self, name: &str) -> Option<&IndexEntry> {
        self.entries.get(name)
    }
}

#[derive(Debug)]
pub struct IndexedFasta {
    index: FastaIndex,
    map: Mmap,
    buffer: Vec<u8>,
}
impl IndexedFasta {
    pub fn new(index: FastaIndex, path: &str) -> Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        let buffer = Vec::new();
        Ok(Self {
            index,
            map: mmap,
            buffer,
        })
    }

    pub fn query(&mut self, name: &str, start: usize, end: usize) -> Result<&[u8]> {
        let entry = match self.index.get(name) {
            Some(entry) => entry,
            None => bail!("No entry found for {}", name),
        };
        self.buffer.clear();
        let query_pos = QueryPosition::new(start, end, entry);
        let seq_slice = &self.map[query_pos.pos..query_pos.pos + query_pos.buffer_size];
        self.buffer.extend_from_slice(seq_slice);
        self.buffer.retain(|&c| c != b'\n');
        Ok(&self.buffer)
    }
}

struct QueryPosition {
    pub buffer_size: usize,
    pub pos: usize,
}
impl QueryPosition {
    pub fn new(start: usize, end: usize, entry: &IndexEntry) -> Self {
        let size = end - start;
        let row_pos = (start / entry.line_bases) * entry.line_width;
        let col_pos = start % entry.line_bases;
        let num_lines = (size + col_pos) / entry.line_bases;
        let buffer_size = size + num_lines;
        let pos = entry.offset + row_pos + col_pos;
        Self { buffer_size, pos }
    }
}
