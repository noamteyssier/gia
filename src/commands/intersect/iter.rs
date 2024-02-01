use bedrs::{traits::IntervalBounds, Intersect};

/// Describes the method used to aggregate and return overlapping intervals.
#[derive(Debug, Copy, Clone)]
pub enum OutputMethod {
    /// Return the intersection of the query and target intervals.
    Intersection,

    /// Return the query interval for each overlapping target interval.
    Query,

    /// Return the overlapping target intervals for each query interval.
    Target,

    /// Return each query interval once if it overlaps with any target interval.
    QueryUnique,

    /// Return query intervals that do not overlap with any target interval.
    Inverse,
}
impl OutputMethod {
    pub fn from_inputs(with_query: bool, with_target: bool, unique: bool, inverse: bool) -> Self {
        if inverse {
            Self::Inverse
        } else if with_query && with_target {
            panic!("Cannot specify both query and target output")
        } else if with_query {
            if unique {
                Self::QueryUnique
            } else {
                Self::Query
            }
        } else if with_target {
            Self::Target
        } else {
            Self::Intersection
        }
    }
}

/// Run the function to find overlapping intervals between query and target intervals.
///
/// But return the results as an iterator of intervals from type A.
pub fn run_function_query<'a, It, Ia, Ib>(
    iv: &'a Ia,
    overlapping: It,
    method: OutputMethod,
) -> Box<dyn Iterator<Item = Ia> + 'a>
where
    It: Iterator<Item = Ib> + 'a,
    Ia: IntervalBounds<usize, usize> + Copy,
    Ib: IntervalBounds<usize, usize> + Copy,
{
    match method {
        OutputMethod::Intersection => Box::new(iter_intersections(iv, overlapping)),
        OutputMethod::Query => Box::new(iter_query(iv, overlapping, false)),
        OutputMethod::QueryUnique => Box::new(iter_query(iv, overlapping, true)),
        OutputMethod::Inverse => Box::new(iter_inverse(iv, overlapping)),
        _ => panic!("Invalid output method for query intervals"),
    }
}

/// Run the function to find overlapping intervals between query and target intervals.
///
/// But return the results as an iterator of intervals from type B.
pub fn run_function_target<'a, It, I>(
    overlapping: It,
    method: OutputMethod,
) -> Box<dyn Iterator<Item = I> + 'a>
where
    It: Iterator<Item = I> + 'a,
    I: IntervalBounds<usize, usize> + Copy + 'a,
{
    match method {
        OutputMethod::Target => Box::new(iter_targets(overlapping)),
        _ => panic!("Invalid output method for query intervals"),
    }
}

fn iter_intersections<'a, It, Ia, Ib>(iv: &'a Ia, overlapping: It) -> impl Iterator<Item = Ia> + 'a
where
    It: Iterator<Item = Ib> + 'a,
    Ia: IntervalBounds<usize, usize>,
    Ib: IntervalBounds<usize, usize>,
{
    overlapping.map(|ov| match ov.intersect(iv) {
        Some(ix) => ix,
        None => {
            panic!("Failed to intersect intervals: There may be a bug in FindIter")
        }
    })
}

fn iter_query<'a, It, Ia, Ib>(
    iv: &'a Ia,
    overlapping: It,
    unique: bool,
) -> Box<dyn Iterator<Item = Ia> + 'a>
where
    It: Iterator<Item = Ib> + 'a,
    Ia: IntervalBounds<usize, usize> + Copy,
    Ib: IntervalBounds<usize, usize> + Copy,
{
    let iter = overlapping.map(|_| *iv);
    if unique {
        let iter = iter.take(1);
        Box::new(iter.into_iter())
    } else {
        Box::new(iter)
    }
}

fn iter_targets<It, I>(overlapping: It) -> impl Iterator<Item = I>
where
    It: Iterator<Item = I>,
    I: IntervalBounds<usize, usize>,
{
    overlapping
}

fn iter_inverse<'a, It, Ia, Ib>(iv: &'a Ia, overlapping: It) -> Box<dyn Iterator<Item = Ia> + 'a>
where
    It: Iterator<Item = Ib> + 'a,
    Ia: IntervalBounds<usize, usize> + Copy,
    Ib: IntervalBounds<usize, usize> + Copy,
{
    let mut overlapping = overlapping.peekable();
    if overlapping.next().is_none() {
        Box::new(std::iter::once(*iv))
    } else {
        Box::new(std::iter::empty())
    }
}
