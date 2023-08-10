use bedrs::{GenomicInterval, Intersect};

#[derive(Debug, Copy, Clone)]
pub enum OutputMethod {
    Intersection,
    Query,
    Target,
    QueryUnique,
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

pub fn run_function<'a, It>(
    iv: &'a GenomicInterval<usize>,
    overlapping: It,
    method: OutputMethod,
) -> Box<dyn Iterator<Item = GenomicInterval<usize>> + 'a>
where
    It: Iterator<Item = GenomicInterval<usize>> + 'a,
{
    match method {
        OutputMethod::Intersection => Box::new(iter_intersections(iv, overlapping)),
        OutputMethod::Query => Box::new(iter_query(iv, overlapping, false)),
        OutputMethod::QueryUnique => Box::new(iter_query(iv, overlapping, true)),
        OutputMethod::Target => Box::new(iter_targets(overlapping)),
        OutputMethod::Inverse => Box::new(iter_inverse(iv, overlapping)),
    }
}

fn iter_intersections<'a, It>(
    iv: &'a GenomicInterval<usize>,
    overlapping: It,
) -> impl Iterator<Item = GenomicInterval<usize>> + 'a
where
    It: Iterator<Item = GenomicInterval<usize>> + 'a,
{
    let iter = overlapping.map(|ov| {
        let ix = match ov.intersect(iv) {
            Some(ix) => ix,
            None => {
                panic!("Failed to intersect intervals: There may be a bug in FindIter")
            }
        };
        ix
    });
    iter
}

fn iter_query<'a, It>(
    iv: &'a GenomicInterval<usize>,
    overlapping: It,
    unique: bool,
) -> Box<dyn Iterator<Item = GenomicInterval<usize>> + 'a>
where
    It: Iterator<Item = GenomicInterval<usize>> + 'a,
{
    let iter = overlapping.map(|_| iv.clone());
    if unique {
        let iter = iter.take(1);
        Box::new(iter.into_iter())
    } else {
        Box::new(iter)
    }
}

fn iter_targets<It>(overlapping: It) -> impl Iterator<Item = GenomicInterval<usize>>
where
    It: Iterator<Item = GenomicInterval<usize>>,
{
    overlapping.map(|ov| ov.clone())
}

fn iter_inverse<'a, It>(
    iv: &'a GenomicInterval<usize>,
    overlapping: It,
) -> Box<dyn Iterator<Item = GenomicInterval<usize>> + 'a>
where
    It: Iterator<Item = GenomicInterval<usize>> + 'a,
{
    let mut overlapping = overlapping.peekable();
    if overlapping.next().is_none() {
        Box::new(std::iter::once(iv.clone()))
    } else {
        Box::new(std::iter::empty())
    }
}
