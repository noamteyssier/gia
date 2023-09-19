use crate::types::StreamTranslater;

use super::NamedInterval;
use bedrs::GenomicInterval;
use csv::ByteRecord;
use std::io::Read;

/// An iterator over a BED file that yields `GenomicInterval<usize>`s.
///
/// It keeps a reference to a `csv::Reader` and a `DashMap` that maps chromosome names to indices.
/// This allows it to yield `GenomicInterval`s with `usize` chromosome indices instead of `String`s.
pub struct NamedIter<'a, 'b, R: Read> {
    reader: &'a mut csv::Reader<R>,
    byterecord: ByteRecord,
    translater: &'b StreamTranslater,
}
impl<'a, 'b, R: Read> NamedIter<'a, 'b, R> {
    pub fn new(reader: &'a mut csv::Reader<R>, translater: &'b StreamTranslater) -> Self {
        Self {
            reader,
            byterecord: ByteRecord::new(),
            translater,
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
        self.translater.add_name(record.name);
        let chr_idx = self.translater.get_name_to_idx().get(record.name).unwrap();
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
