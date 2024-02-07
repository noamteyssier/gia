use super::{DualInput, Output, OverlapPredicates};
use anyhow::{bail, Result};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct IntersectArgs {
    #[clap(flatten)]
    pub inputs: DualInput,

    #[clap(flatten)]
    pub output: Output,

    #[clap(flatten)]
    pub overlap_predicates: OverlapPredicates,

    #[clap(flatten)]
    pub output_predicates: OutputPredicates,

    /// Stream the input files instead of loading them into memory
    /// (only works if both files are sorted)
    #[clap(short = 'S', long, conflicts_with_all = &["with_query", "with_target", "unique", "inverse"])]
    pub stream: bool,
}

#[derive(Parser, Debug)]
pub struct OutputPredicates {
    /// Return the records from a that overlap with b instead of the intersection
    #[clap(short = 'q', long, conflicts_with = "with_target")]
    pub with_query: bool,

    /// Return the records from b that overlap with a instead of the intersection
    #[clap(short = 't', long, conflicts_with = "with_query")]
    pub with_target: bool,

    /// Only write the query record once if it overlaps with multiple target records
    #[clap(short, long, requires = "with_query", conflicts_with = "with_target")]
    pub unique: bool,

    /// Only report the intervals in the query that do not overlap with the target
    /// (i.e. the inverse of the intersection)
    #[clap(short = 'v', long, conflicts_with_all = &["with_query", "with_target", "unique"])]
    pub inverse: bool,
}
impl TryFrom<OutputPredicates> for OutputMethod {
    type Error = anyhow::Error;
    fn try_from(value: OutputPredicates) -> Result<Self> {
        if value.inverse {
            Ok(Self::Inverse)
        } else if value.with_query && value.with_target {
            bail!("Cannot specify both query and target output")
        } else if value.with_query {
            if value.unique {
                Ok(Self::QueryUnique)
            } else {
                Ok(Self::Query)
            }
        } else if value.with_target {
            Ok(Self::Target)
        } else {
            Ok(Self::Intersection)
        }
    }
}

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
