use bedrs::{Coordinates, Strand};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use crate::types::{Translate, Translater};

#[derive(Debug, Deserialize, Serialize)]
pub struct Bed6<'a> {
    pub chr: &'a str,
    pub start: usize,
    pub end: usize,
    pub name: &'a str,
    pub score: f64,
    pub strand: Strand,
}
#[allow(dead_code)]
impl<'a> Bed6<'a> {
    pub fn from_numeric(record: &NumericBed6, translater: &'a Translater) -> Self {
        Self {
            chr: translater.get_name(record.chr).unwrap(),
            start: record.start,
            end: record.end,
            name: translater.get_name(record.name).unwrap(),
            score: record.score,
            strand: record.strand,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NumericBed6Set {
    records: Vec<NumericBed6>,
    max_len: Option<usize>,
    is_sorted: bool,
}
impl FromIterator<NumericBed6> for NumericBed6Set {
    fn from_iter<I: IntoIterator<Item = NumericBed6>>(iter: I) -> Self {
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
// impl Container<usize, usize, NumericBed6> for NumericBed6Set {
//     fn new(records: Vec<NumericBed6>) -> Self {
//         let max_len = records.iter().map(|iv| iv.len()).max();
//         Self {
//             records,
//             max_len,
//             is_sorted: false,
//         }
//     }

//     fn records(&self) -> &Vec<NumericBed6> {
//         &self.records
//     }
//     fn records_mut(&mut self) -> &mut Vec<NumericBed6> {
//         &mut self.records
//     }
//     fn records_owned(self) -> Vec<NumericBed6> {
//         self.records
//     }
//     fn is_sorted(&self) -> bool {
//         self.is_sorted
//     }
//     fn sorted_mut(&mut self) -> &mut bool {
//         &mut self.is_sorted
//     }
//     fn max_len(&self) -> Option<usize> {
//         self.max_len
//     }
//     fn max_len_mut(&mut self) -> &mut Option<usize> {
//         &mut self.max_len
//     }
//     fn span(&self) -> Result<NumericBed6> {
//         if self.is_empty() {
//             bail!("Cannot get span of empty interval set")
//         } else if !self.is_sorted() {
//             bail!("Cannot get span of unsorted interval set")
//         } else {
//             let first = self.records().first().unwrap();
//             let last = self.records().last().unwrap();
//             if first.chr() != last.chr() {
//                 bail!("Cannot get span of interval set spanning multiple chromosomes")
//             } else {
//                 let iv = NumericBed6::new(
//                     *first.chr(),
//                     first.start(),
//                     last.end(),
//                     0,
//                     0.0,
//                     Strand::Unknown,
//                 );
//                 Ok(iv)
//             }
//         }
//     }
// }

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct NumericBed6 {
    pub chr: usize,
    pub start: usize,
    pub end: usize,
    pub name: usize,
    pub score: f64,
    pub strand: Strand,
}
#[allow(dead_code)]
impl NumericBed6 {
    pub fn from_bed6(bed6: &Bed6, name_to_idx: &HashMap<String, usize>) -> Self {
        Self {
            chr: name_to_idx[bed6.chr],
            start: bed6.start,
            end: bed6.end,
            name: name_to_idx[bed6.name],
            score: bed6.score,
            strand: bed6.strand,
        }
    }
    pub fn new(
        chr: usize,
        start: usize,
        end: usize,
        name: usize,
        score: f64,
        strand: Strand,
    ) -> Self {
        Self {
            chr,
            start,
            end,
            name,
            score,
            strand,
        }
    }
    pub fn name(&self) -> usize {
        self.name
    }
    pub fn update_name(&mut self, name: &usize) {
        self.name = *name;
    }
}
impl Coordinates<usize, usize> for NumericBed6 {
    fn chr(&self) -> &usize {
        &self.chr
    }

    fn start(&self) -> usize {
        self.start
    }

    fn end(&self) -> usize {
        self.end
    }

    fn empty() -> Self {
        Self {
            chr: 0,
            start: 0,
            end: 0,
            name: 0,
            score: 0.0,
            strand: Strand::Unknown,
        }
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

    fn from<Iv: Coordinates<usize, usize>>(other: &Iv) -> Self {
        Self {
            chr: *other.chr(),
            start: other.start(),
            end: other.end(),
            name: 0,
            score: 0.0,
            strand: other.strand().unwrap_or(Strand::Unknown),
        }
    }
}
impl Coordinates<usize, usize> for &NumericBed6 {
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

    fn empty() -> Self {
        unreachable!("Cannot create empty reference")
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
    fn from<Iv>(other: &Iv) -> Self {
        unimplemented!("Cannot create owned instance of a reference")
    }
}
