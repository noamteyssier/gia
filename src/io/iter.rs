use super::NamedInterval;
use crate::types::{Bed6, NumericBed6, StreamTranslater};
use bedrs::{Coordinates, GenomicInterval};
use csv::ByteRecord;
use std::{io::Read, marker::PhantomData};

/// An iterator over a BED file that yields `GenomicInterval<usize>`s.
///
/// It keeps a reference to a `csv::Reader` and a `DashMap` that maps chromosome names to indices.
/// This allows it to yield `GenomicInterval`s with `usize` chromosome indices instead of `String`s.
pub struct NamedIter<'a, 'b, R: Read, C: Coordinates<usize, usize>> {
    reader: &'a mut csv::Reader<R>,
    byterecord: ByteRecord,
    translater: &'b StreamTranslater,
    phantom: PhantomData<C>,
}
impl<'a, 'b, R: Read, C: Coordinates<usize, usize>> NamedIter<'a, 'b, R, C> {
    pub fn new(reader: &'a mut csv::Reader<R>, translater: &'b StreamTranslater) -> Self {
        Self {
            reader,
            byterecord: ByteRecord::new(),
            translater,
            phantom: PhantomData,
        }
    }
}
impl<'a, 'b, R: Read> Iterator for NamedIter<'a, 'b, R, GenomicInterval<usize>> {
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
impl<'a, 'b, R: Read> Iterator for NamedIter<'a, 'b, R, NumericBed6> {
    type Item = NumericBed6;

    fn next(&mut self) -> Option<Self::Item> {
        if !self
            .reader
            .read_byte_record(&mut self.byterecord)
            .expect("Error reading BED file")
        {
            return None;
        }
        let record: Bed6 = self
            .byterecord
            .deserialize(None)
            .expect("Error parsing BED record");
        self.translater.add_name(record.chr);
        self.translater.add_name(record.name);
        let chr_idx = self.translater.get_name_to_idx().get(record.chr).unwrap();
        let name_idx = self.translater.get_name_to_idx().get(record.name).unwrap();
        let iv = NumericBed6::new(
            *chr_idx,
            record.start,
            record.end,
            *name_idx,
            record.score,
            record.strand,
        );
        Some(iv)
    }
}

/// An iterator over a BED file that yields `Coordinates`s.
///
/// It keeps a reference to a `csv::Reader`.
pub struct UnnamedIter<'a, R: Read, C: Coordinates<usize, usize>> {
    reader: &'a mut csv::Reader<R>,
    byterecord: ByteRecord,
    phantom: PhantomData<C>,
}
impl<'a, R: Read, C: Coordinates<usize, usize>> UnnamedIter<'a, R, C> {
    pub fn new(reader: &'a mut csv::Reader<R>) -> Self {
        Self {
            reader,
            byterecord: ByteRecord::new(),
            phantom: PhantomData,
        }
    }
}
impl<'a, R: Read> Iterator for UnnamedIter<'a, R, GenomicInterval<usize>> {
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
impl<'a, R: Read> Iterator for UnnamedIter<'a, R, NumericBed6> {
    type Item = NumericBed6;

    fn next(&mut self) -> Option<Self::Item> {
        if !self
            .reader
            .read_byte_record(&mut self.byterecord)
            .expect("Error reading BED file")
        {
            return None;
        }
        let iv: NumericBed6 = self
            .byterecord
            .deserialize(None)
            .expect("Error parsing BED record");
        Some(iv)
    }
}
