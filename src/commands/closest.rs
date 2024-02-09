use crate::{
    cli::{ClosestArgs, ClosestParams},
    dispatch_pair,
    io::write_pairs_iter_with,
    types::{InputFormat, IntervalPair, Rename, Renamer, SplitTranslater},
    utils::sort_pairs,
};
use anyhow::Result;
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
impl From<ClosestParams> for ClosestType {
    fn from(params: ClosestParams) -> Self {
        ClosestType::new(params.upstream, params.downstream)
    }
}

fn run_closest<'a, Ia, Ib, Na, Nb, W>(
    mut a_set: IntervalContainer<Ia, usize, usize>,
    mut b_set: IntervalContainer<Ib, usize, usize>,
    translater: Option<&'a SplitTranslater>,
    params: ClosestParams,
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
    sort_pairs(&mut a_set, &mut b_set, params.sorted);
    let pairs_iter = a_set
        .iter()
        .map(|query| {
            let target = match params.into() {
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

pub fn closest(args: ClosestArgs) -> Result<()> {
    let (bed_a, bed_b) = args.inputs.get_readers()?;
    let writer = args.output.get_writer()?;
    dispatch_pair!(bed_a, bed_b, writer, args.params, run_closest)
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
