use super::NamedInterval;
use bedrs::GenomicInterval;
use csv::ByteRecord;
use dashmap::DashMap;
use std::io::Read;

/// An iterator over a BED file that yields `GenomicInterval<usize>`s.
///
/// It keeps a reference to a `csv::Reader` and a `DashMap` that maps chromosome names to indices.
/// This allows it to yield `GenomicInterval`s with `usize` chromosome indices instead of `String`s.
pub struct NamedIter<'a, 'b, R: Read> {
    reader: &'a mut csv::Reader<R>,
    byterecord: ByteRecord,
    name_map: &'b DashMap<String, usize>,
    idx_map: &'b DashMap<usize, String>,
}
impl<'a, 'b, R: Read> NamedIter<'a, 'b, R> {
    pub fn new(
        reader: &'a mut csv::Reader<R>,
        name_map: &'b DashMap<String, usize>,
        idx_map: &'b DashMap<usize, String>,
    ) -> Self {
        Self {
            reader,
            byterecord: ByteRecord::new(),
            name_map,
            idx_map,
        }
    }
}
impl<'a, 'b, R: Read> Iterator for NamedIter<'a, 'b, R> {
    type Item = GenomicInterval<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self
            .reader
            .read_byte_record(&mut self.byterecord)
            .expect("Error reading BED file")
        {
            return None;
        }
        let record: NamedInterval = self
            .byterecord
            .deserialize(None)
            .expect("Error parsing BED record");
        if !self.name_map.contains_key(record.name) {
            let idx = self.name_map.len();
            self.name_map.insert(record.name.to_string(), idx);
            self.idx_map.insert(idx, record.name.to_string());
        }
        let chr_idx = self.name_map.get(record.name).unwrap();
        let iv = GenomicInterval::new(*chr_idx, record.start, record.end);
        Some(iv)
    }
}

/// An iterator over a BED file that yields `GenomicInterval<usize>`s.
///
/// It keeps a reference to a `csv::Reader`.
pub struct UnnamedIter<'a, R: Read> {
    reader: &'a mut csv::Reader<R>,
    byterecord: ByteRecord,
}
impl<'a, R: Read> UnnamedIter<'a, R> {
    pub fn new(reader: &'a mut csv::Reader<R>) -> Self {
        Self {
            reader,
            byterecord: ByteRecord::new(),
        }
    }
}
impl<'a, R: Read> Iterator for UnnamedIter<'a, R> {
    type Item = GenomicInterval<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self
            .reader
            .read_byte_record(&mut self.byterecord)
            .expect("Error reading BED file")
        {
            return None;
        }
        let iv: GenomicInterval<usize> = self
            .byterecord
            .deserialize(None)
            .expect("Error parsing BED record");
        Some(iv)
    }
}
