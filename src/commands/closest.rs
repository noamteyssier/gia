use crate::{
    io::{match_output, write_pairs_iter_with, BedReader},
    types::{InputFormat, IntervalPair, Rename, Renamer, Translater},
    utils::sort_pairs,
};
use anyhow::{bail, Result};
use bedrs::{traits::IntervalBounds, IntervalContainer};
use serde::Serialize;
use std::io::Write;

#[derive(Debug, PartialEq)]
enum ClosestType {
    Upstream,
    Downstream,
    Both,
}
impl ClosestType {
    pub fn new(upstream: bool, downstream: bool) -> Self {
        if upstream && downstream {
            ClosestType::Both
        } else if upstream {
            ClosestType::Upstream
        } else if downstream {
            ClosestType::Downstream
        } else {
            ClosestType::Both
        }
    }
}

fn run_closest<'a, Ia, Ib, Na, Nb, W>(
    mut a_set: IntervalContainer<Ia, usize, usize>,
    mut b_set: IntervalContainer<Ib, usize, usize>,
    translater: Option<&'a Translater>,
    method: ClosestType,
    presorted: bool,
    output: W,
) -> Result<()>
where
    Ia: IntervalBounds<usize, usize> + Serialize + Copy,
    Ib: IntervalBounds<usize, usize> + Serialize + Copy,
    Na: IntervalBounds<&'a str, usize> + Serialize,
    Nb: IntervalBounds<&'a str, usize> + Serialize,
    W: Write,
    Renamer: Rename<'a, Ia, Na> + Rename<'a, Ib, Nb>,
{
    sort_pairs(&mut a_set, &mut b_set, presorted);
    let pairs_iter = a_set
        .iter()
        .map(|query| {
            let target = match method {
                ClosestType::Both => b_set.closest(query),
                ClosestType::Upstream => b_set.closest_upstream(query),
                ClosestType::Downstream => b_set.closest_downstream(query),
            }
            .expect("Could not build closest index");
            (query, target)
        })
        .filter_map(|(query, target)| {
            if let Some(target) = target {
                Some(IntervalPair::new(*query, *target, translater))
            } else {
                None
            }
        });
    write_pairs_iter_with(pairs_iter, output, translater)
}

fn dispatch_closest<W: Write>(
    bed_a: BedReader,
    bed_b: BedReader,
    method: ClosestType,
    presorted: bool,
    output: W,
) -> Result<()> {
    let mut translater = if bed_a.is_named() {
        Some(Translater::new())
    } else {
        None
    };
    match bed_a.input_format() {
        InputFormat::Bed3 => {
            let set_a = bed_a.bed3_set_with(translater.as_mut())?;
            match bed_b.input_format() {
                InputFormat::Bed3 => {
                    let set_b = bed_b.bed3_set_with(translater.as_mut())?;
                    run_closest(set_a, set_b, translater.as_ref(), method, presorted, output)
                }
                InputFormat::Bed6 => {
                    let set_b = bed_b.bed6_set_with(translater.as_mut())?;
                    run_closest(set_a, set_b, translater.as_ref(), method, presorted, output)
                }
                InputFormat::Bed12 => {
                    let set_b = bed_b.bed12_set_with(translater.as_mut())?;
                    run_closest(set_a, set_b, translater.as_ref(), method, presorted, output)
                }
            }
        }
        InputFormat::Bed6 => {
            let set_a = bed_a.bed6_set_with(translater.as_mut())?;
            match bed_b.input_format() {
                InputFormat::Bed3 => {
                    let set_b = bed_b.bed3_set_with(translater.as_mut())?;
                    run_closest(set_a, set_b, translater.as_ref(), method, presorted, output)
                }
                InputFormat::Bed6 => {
                    let set_b = bed_b.bed6_set_with(translater.as_mut())?;
                    run_closest(set_a, set_b, translater.as_ref(), method, presorted, output)
                }
                InputFormat::Bed12 => {
                    let set_b = bed_b.bed12_set_with(translater.as_mut())?;
                    run_closest(set_a, set_b, translater.as_ref(), method, presorted, output)
                }
            }
        }
        InputFormat::Bed12 => {
            let set_a = bed_a.bed12_set_with(translater.as_mut())?;
            match bed_b.input_format() {
                InputFormat::Bed3 => {
                    let set_b = bed_b.bed3_set_with(translater.as_mut())?;
                    run_closest(set_a, set_b, translater.as_ref(), method, presorted, output)
                }
                InputFormat::Bed6 => {
                    let set_b = bed_b.bed6_set_with(translater.as_mut())?;
                    run_closest(set_a, set_b, translater.as_ref(), method, presorted, output)
                }
                InputFormat::Bed12 => {
                    let set_b = bed_b.bed12_set_with(translater.as_mut())?;
                    run_closest(set_a, set_b, translater.as_ref(), method, presorted, output)
                }
            }
        }
    }
}

pub fn closest(
    a: Option<String>,
    b: String,
    output: Option<String>,
    upstream: bool,
    downstream: bool,
    presorted: bool,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    let bed_a = BedReader::from_path(a, None, None)?;
    let bed_b = BedReader::from_path(Some(b), None, None)?;
    if bed_a.is_named() != bed_b.is_named() {
        bail!("Input files must both be named or both be unnamed")
    }
    let method = ClosestType::new(upstream, downstream);
    let output = match_output(output, compression_threads, compression_level)?;
    dispatch_closest(bed_a, bed_b, method, presorted, output)
}

#[cfg(test)]
mod testing {

    use super::*;
    // use crate::{io::read_bed3_set, types::NumericBed3};
    // use bedrs::Coordinates;

    // #[test]
    // ///    x-----y      x-----y      x-------y
    // ///           i-j           i-j
    // /// =====================================
    // ///           i-j
    // ///                         i-j
    // ///                         i-j
    // fn closest() {
    //     let interval_text = "1\t10\t20\n1\t30\t40\n1\t50\t60\n";
    //     let (set, _) = read_bed3_set(interval_text.as_bytes(), false).unwrap();
    //     let query_set = IntervalContainer::from_unsorted(vec![
    //         NumericBed3::new(1, 22, 23),
    //         NumericBed3::new(1, 42, 43),
    //     ]);
    //     let method = ClosestType::Both;
    //     let closest = run_closest(&set, &query_set, method)
    //         .map(|pair| pair.iv_b)
    //         .collect::<Vec<_>>();
    //     assert!(closest.len() == 3);

    //     assert!(closest[0].unwrap().eq(&NumericBed3::new(1, 22, 23)));
    //     assert!(closest[1].unwrap().eq(&NumericBed3::new(1, 42, 43)));
    //     assert!(closest[2].unwrap().eq(&NumericBed3::new(1, 42, 43)));
    // }

    // #[test]
    // ///    x-----y      x-----y      x-------y
    // ///           i-j           i-j
    // /// =====================================
    // /// None
    // ///           i-j
    // ///                         i-j
    // fn closest_upstream() {
    //     let interval_text = "1\t10\t20\n1\t30\t40\n1\t50\t60\n";
    //     let (set, _) = read_bed3_set(interval_text.as_bytes(), false).unwrap();
    //     let query_set = IntervalContainer::from_unsorted(vec![
    //         NumericBed3::new(1, 22, 23),
    //         NumericBed3::new(1, 42, 43),
    //     ]);
    //     let method = ClosestType::Upstream;
    //     let closest = run_closest(&set, &query_set, method)
    //         .map(|pair| pair.iv_b)
    //         .collect::<Vec<_>>();
    //     assert!(closest.len() == 3);
    //     assert!(closest[0].is_none());
    //     assert!(closest[1].unwrap().eq(&NumericBed3::new(1, 22, 23)));
    //     assert!(closest[2].unwrap().eq(&NumericBed3::new(1, 42, 43)));
    // }

    // #[test]
    // ///    x-----y      x-----y      x-------y
    // ///           i-j           i-j
    // /// =====================================
    // ///           i-j
    // ///                         i-j
    // /// None
    // fn closest_downstream() {
    //     let interval_text = "1\t10\t20\n1\t30\t40\n1\t50\t60\n";
    //     let (set, _) = read_bed3_set(interval_text.as_bytes(), false).unwrap();
    //     let query_set = IntervalContainer::from_unsorted(vec![
    //         NumericBed3::new(1, 22, 23),
    //         NumericBed3::new(1, 42, 43),
    //     ]);
    //     let method = ClosestType::Downstream;
    //     let closest = run_closest(&set, &query_set, method)
    //         .map(|pair| pair.iv_b)
    //         .collect::<Vec<_>>();
    //     assert!(closest.len() == 3);
    //     assert!(closest[0].unwrap().eq(&NumericBed3::new(1, 22, 23)));
    //     assert!(closest[1].unwrap().eq(&NumericBed3::new(1, 42, 43)));
    //     assert!(closest[2].is_none());
    // }

    #[test]
    fn method_build() {
        let both = ClosestType::new(true, true);
        let upstream = ClosestType::new(true, false);
        let downstream = ClosestType::new(false, true);
        let none = ClosestType::new(false, false);

        assert_eq!(both, ClosestType::Both);
        assert_eq!(upstream, ClosestType::Upstream);
        assert_eq!(downstream, ClosestType::Downstream);
        assert_eq!(none, ClosestType::Both);
    }
}
