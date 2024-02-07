use crate::{
    cli::ClosestArgs,
    io::{write_pairs_iter_with, BedReader},
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
            target.map(|target| IntervalPair::new(*query, *target, translater))
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

pub fn closest(args: ClosestArgs) -> Result<()> {
    let (bed_a, bed_b) = args.inputs.get_readers()?;
    let method = ClosestType::new(args.upstream, args.downstream);
    let output = args.output.get_handle()?;
    dispatch_closest(bed_a, bed_b, method, args.sorted, output)
}

#[cfg(test)]
mod testing {

    use super::*;

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
