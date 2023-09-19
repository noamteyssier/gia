use crate::{
    io::{
        match_input, match_output, read_bed3_set_unnamed, read_paired_bed3_named,
        read_paired_bed3_sets, read_paired_bed6_sets, write_pairs_iter_with,
    },
    types::{InputFormat, IntervalPair, Translater},
    utils::sort_pairs,
};
use anyhow::Result;
use bedrs::{
    traits::{ChromBounds, IntervalBounds, ValueBounds},
    Closest, Container, GenomicInterval, GenomicIntervalSet,
};

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

fn run_closest<'a, C, T, I>(
    a_set: &'a impl Container<C, T, I>,
    b_set: &'a impl Container<C, T, I>,
    method: ClosestType,
) -> impl Iterator<Item = IntervalPair<I, C, T>> + 'a
where
    C: ChromBounds + 'a,
    T: ValueBounds + 'a,
    I: IntervalBounds<C, T> + 'a,
{
    a_set
        .iter()
        .map(move |query| {
            let target = match method {
                ClosestType::Both => b_set.closest(query),
                ClosestType::Upstream => b_set.closest_upstream(query),
                ClosestType::Downstream => b_set.closest_downstream(query),
            }
            .expect("Could build closest index");
            (query, target)
        })
        .map(|(query, target)| IntervalPair::new(query.clone(), target.cloned()))
}

fn load_pairs(
    query_input: Option<String>,
    target_input: Option<String>,
    named: bool,
    sorted: bool,
) -> Result<(
    GenomicIntervalSet<usize>,
    GenomicIntervalSet<usize>,
    Option<Translater>,
)> {
    let query_handle = match_input(query_input)?;
    let target_handle = match_input(target_input)?;
    let (mut query_set, mut target_set, translater) =
        read_paired_bed3_sets(query_handle, target_handle, named)?;
    sort_pairs(&mut query_set, &mut target_set, sorted);
    Ok((query_set, target_set, translater))
}

pub fn closest_bed3(
    a: Option<String>,
    b: String,
    output: Option<String>,
    upstream: bool,
    downstream: bool,
    named: bool,
    sorted: bool,
) -> Result<()> {
    // load pairs
    let query_handle = match_input(a)?;
    let target_handle = match_input(Some(b))?;
    let (mut a_set, mut b_set, translater) =
        read_paired_bed3_sets(query_handle, target_handle, named)?;
    sort_pairs(&mut a_set, &mut b_set, sorted);

    // run closest
    let method = ClosestType::new(upstream, downstream);
    let pairs_iter = run_closest(&a_set, &b_set, method);

    // write output
    let output_handle = match_output(output)?;
    write_pairs_iter_with(pairs_iter, output_handle, translater.as_ref())?;
    Ok(())
}

pub fn closest_bed6(
    a: Option<String>,
    b: String,
    output: Option<String>,
    upstream: bool,
    downstream: bool,
    named: bool,
    sorted: bool,
) -> Result<()> {
    // load pairs
    let query_handle = match_input(a)?;
    let target_handle = match_input(Some(b))?;
    let (mut a_set, mut b_set, translater) =
        read_paired_bed6_sets(query_handle, target_handle, named)?;
    sort_pairs(&mut a_set, &mut b_set, sorted);

    // run closest
    let method = ClosestType::new(upstream, downstream);
    let pairs_iter = run_closest(&a_set, &b_set, method);

    // write output
    let output_handle = match_output(output)?;
    write_pairs_iter_with(pairs_iter, output_handle, translater.as_ref())?;
    Ok(())
}

pub fn closest(
    a: Option<String>,
    b: String,
    output: Option<String>,
    upstream: bool,
    downstream: bool,
    named: bool,
    format: InputFormat,
    sorted: bool,
) -> Result<()> {
    if format == InputFormat::Bed3 {
        closest_bed3(a, b, output, upstream, downstream, named, sorted)
    } else {
        closest_bed6(a, b, output, upstream, downstream, named, sorted)
    }
}

#[cfg(test)]
mod testing {

    use super::*;

    #[test]
    ///    x-----y      x-----y      x-------y
    ///           i-j           i-j
    /// =====================================
    ///           i-j
    ///                         i-j
    ///                         i-j
    fn closest() {
        let interval_text = "1\t10\t20\n1\t30\t40\n1\t50\t60\n";
        let set = read_bed3_set_unnamed(interval_text.as_bytes()).unwrap();
        let query_set = GenomicIntervalSet::from_unsorted(vec![
            GenomicInterval::new(1, 22, 23),
            GenomicInterval::new(1, 42, 43),
        ]);
        let method = ClosestType::Both;
        let closest = run_closest(&set, &query_set, method)
            .map(|pair| pair.iv_b)
            .collect::<Vec<_>>();
        assert!(closest.len() == 3);
        assert_eq!(closest[0], Some(GenomicInterval::new(1, 22, 23)));
        assert_eq!(closest[1], Some(GenomicInterval::new(1, 42, 43)));
        assert_eq!(closest[2], Some(GenomicInterval::new(1, 42, 43)));
    }

    #[test]
    ///    x-----y      x-----y      x-------y
    ///           i-j           i-j
    /// =====================================
    /// None
    ///           i-j
    ///                         i-j
    fn closest_upstream() {
        let interval_text = "1\t10\t20\n1\t30\t40\n1\t50\t60\n";
        let set = read_bed3_set_unnamed(interval_text.as_bytes()).unwrap();
        let query_set = GenomicIntervalSet::from_unsorted(vec![
            GenomicInterval::new(1, 22, 23),
            GenomicInterval::new(1, 42, 43),
        ]);
        let method = ClosestType::Upstream;
        let closest = run_closest(&set, &query_set, method)
            .map(|pair| pair.iv_b)
            .collect::<Vec<_>>();
        assert!(closest.len() == 3);
        assert_eq!(closest[0], None);
        assert_eq!(closest[1], Some(GenomicInterval::new(1, 22, 23)));
        assert_eq!(closest[2], Some(GenomicInterval::new(1, 42, 43)));
    }

    #[test]
    ///    x-----y      x-----y      x-------y
    ///           i-j           i-j
    /// =====================================
    ///           i-j
    ///                         i-j
    /// None
    fn closest_downstream() {
        let interval_text = "1\t10\t20\n1\t30\t40\n1\t50\t60\n";
        let set = read_bed3_set_unnamed(interval_text.as_bytes()).unwrap();
        let query_set = GenomicIntervalSet::from_unsorted(vec![
            GenomicInterval::new(1, 22, 23),
            GenomicInterval::new(1, 42, 43),
        ]);
        let method = ClosestType::Downstream;
        let closest = run_closest(&set, &query_set, method)
            .map(|pair| pair.iv_b)
            .collect::<Vec<_>>();
        assert!(closest.len() == 3);
        assert_eq!(closest[0], Some(GenomicInterval::new(1, 22, 23)));
        assert_eq!(closest[1], Some(GenomicInterval::new(1, 42, 43)));
        assert_eq!(closest[2], None);
    }

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
