use anyhow::{bail, Result};
use bedrs::{Container, Coordinates, Strand};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use crate::types::{Translate, Translater};

#[derive(Debug, Deserialize, Serialize)]
pub struct Bed12<'a> {
    pub chr: &'a str,
    pub start: usize,
    pub end: usize,
    pub name: &'a str,
    pub score: f64,
    pub strand: Strand,
    pub thick_start: usize,
    pub thick_end: usize,
    pub item_rgb: &'a str,
    pub block_count: usize,
    pub block_sizes: &'a str,
    pub block_starts: &'a str,
}
#[allow(dead_code)]
impl<'a> Bed12<'a> {
    pub fn from_numeric(record: &NumericBed12, translater: &'a Translater) -> Self {
        Self {
            chr: translater.get_name(record.chr).unwrap(),
            start: record.start,
            end: record.end,
            name: translater.get_name(record.name).unwrap(),
            score: record.score,
            strand: record.strand,
            thick_start: record.thick_start,
            thick_end: record.thick_end,
            item_rgb: translater.get_name(record.item_rgb).unwrap(),
            block_count: record.block_count,
            block_sizes: translater.get_name(record.block_sizes).unwrap(),
            block_starts: translater.get_name(record.block_starts).unwrap(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NumericBed12Set {
    records: Vec<NumericBed12>,
    max_len: Option<usize>,
    is_sorted: bool,
}
impl FromIterator<NumericBed12> for NumericBed12Set {
    fn from_iter<I: IntoIterator<Item = NumericBed12>>(iter: I) -> Self {
        let mut max_len = 0;
        let records = iter
            .into_iter()
            .map(|interval| {
                max_len = max_len.max(interval.len());
                interval
            })
            .collect();
        let max_len = if max_len == 0 { None } else { Some(max_len) };
        Self {
            records,
            max_len,
            is_sorted: false,
        }
    }
}
impl Container<usize, usize, NumericBed12> for NumericBed12Set {
    fn new(records: Vec<NumericBed12>) -> Self {
        let max_len = records.iter().map(|iv| iv.len()).max();
        Self {
            records,
            max_len,
            is_sorted: false,
        }
    }

    fn records(&self) -> &Vec<NumericBed12> {
        &self.records
    }
    fn records_mut(&mut self) -> &mut Vec<NumericBed12> {
        &mut self.records
    }
    fn records_owned(self) -> Vec<NumericBed12> {
        self.records
    }
    fn is_sorted(&self) -> bool {
        self.is_sorted
    }
    fn sorted_mut(&mut self) -> &mut bool {
        &mut self.is_sorted
    }
    fn max_len(&self) -> Option<usize> {
        self.max_len
    }
    fn max_len_mut(&mut self) -> &mut Option<usize> {
        &mut self.max_len
    }
    fn span(&self) -> Result<NumericBed12> {
        if self.is_empty() {
            bail!("Cannot get span of empty interval set")
        } else if !self.is_sorted() {
            bail!("Cannot get span of unsorted interval set")
        } else {
            let first = self.records().first().unwrap();
            let last = self.records().last().unwrap();
            if first.chr() != last.chr() {
                bail!("Cannot get span of interval set spanning multiple chromosomes")
            } else {
                let iv = NumericBed12::new(
                    *first.chr(),
                    first.start(),
                    last.end(),
                    0,
                    0.0,
                    Strand::Unknown,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                );
                Ok(iv)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct NumericBed12 {
    pub chr: usize,
    pub start: usize,
    pub end: usize,
    pub name: usize,
    pub score: f64,
    pub strand: Strand,
    pub thick_start: usize,
    pub thick_end: usize,
    pub item_rgb: usize,
    pub block_count: usize,
    pub block_sizes: usize,
    pub block_starts: usize,
}
#[allow(dead_code)]
impl NumericBed12 {
    pub fn from_bed12(bed12: &Bed12, name_to_idx: &HashMap<String, usize>) -> Self {
        Self {
            chr: name_to_idx[bed12.chr],
            start: bed12.start,
            end: bed12.end,
            name: name_to_idx[bed12.name],
            score: bed12.score,
            strand: bed12.strand,
            thick_start: bed12.thick_start,
            thick_end: bed12.thick_end,
            item_rgb: name_to_idx[bed12.item_rgb],
            block_count: bed12.block_count,
            block_sizes: name_to_idx[bed12.block_sizes],
            block_starts: name_to_idx[bed12.block_starts],
        }
    }
    pub fn new(
        chr: usize,
        start: usize,
        end: usize,
        name: usize,
        score: f64,
        strand: Strand,
        thick_start: usize,
        thick_end: usize,
        item_rgb: usize,
        block_count: usize,
        block_sizes: usize,
        block_starts: usize,
    ) -> Self {
        Self {
            chr,
            start,
            end,
            name,
            score,
            strand,
            thick_start,
            thick_end,
            item_rgb,
            block_count,
            block_sizes,
            block_starts,
        }
    }
    pub fn name(&self) -> usize {
        self.name
    }
    pub fn update_name(&mut self, name: &usize) {
        self.name = *name;
    }
    pub fn update_item_rgb(&mut self, item_rgb: &usize) {
        self.item_rgb = *item_rgb;
    }
    pub fn update_block_sizes(&mut self, block_sizes: &usize) {
        self.block_sizes = *block_sizes;
    }
    pub fn update_block_starts(&mut self, block_starts: &usize) {
        self.block_starts = *block_starts;
    }
}
impl Coordinates<usize, usize> for NumericBed12 {
    fn chr(&self) -> &usize {
        &self.chr
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.end
    }

    fn strand(&self) -> Option<Strand> {
        Some(self.strand)
    }

    fn update_chr(&mut self, chr: &usize) {
        self.chr = *chr;
    }

    fn update_start(&mut self, start: &usize) {
        self.start = *start;
    }

    fn update_end(&mut self, end: &usize) {
        self.end = *end;
    }

    fn from(other: &Self) -> Self {
        Self {
            chr: other.chr,
            start: other.start,
            end: other.end,
            name: other.name,
            score: other.score,
            strand: other.strand,
            thick_start: other.thick_start,
            thick_end: other.thick_end,
            item_rgb: other.item_rgb,
            block_count: other.block_count,
            block_sizes: other.block_sizes,
            block_starts: other.block_starts,
        }
    }
}
impl Coordinates<usize, usize> for &NumericBed12 {
    fn chr(&self) -> &usize {
        &self.chr
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.end
    }

    fn strand(&self) -> Option<Strand> {
        Some(self.strand)
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn update_chr(&mut self, chr: &usize) {
        unreachable!("Cannot update chr of immutable reference")
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn update_start(&mut self, start: &usize) {
        unreachable!("Cannot update start of immutable reference")
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn update_end(&mut self, end: &usize) {
        unreachable!("Cannot update end of immutable reference")
    }

    #[allow(dead_code)]
    #[allow(unused_variables)]
    fn from(other: &Self) -> Self {
        unimplemented!("Cannot create owned instance of a reference")
    }
}
