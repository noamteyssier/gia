use bedrs::{traits::ValueBounds, types::QueryMethod};
use clap::Parser;

#[derive(Parser, Debug, Clone, Copy)]
pub struct OverlapPredicates {
    /// Minimum fraction of a's interval that must be covered by b's interval
    #[clap(short = 'f', long)]
    pub fraction_query: Option<f64>,

    /// Minimum fraction of b's interval that must be covered by a's interval
    #[clap(short = 'F', long)]
    pub fraction_target: Option<f64>,

    /// Require that the fraction provided with `-f` is reciprocal to both
    /// query and target
    #[clap(
        short,
        long,
        requires = "fraction_query",
        conflicts_with = "fraction_target"
    )]
    pub reciprocal: bool,

    /// Requires that either fraction provided with `-f` or `-F` is met
    #[clap(short, long, requires_all=&["fraction_query", "fraction_target"], conflicts_with = "reciprocal")]
    pub either: bool,
}

impl<T: ValueBounds> From<OverlapPredicates> for QueryMethod<T> {
    fn from(args: OverlapPredicates) -> Self {
        let fraction_target = if args.reciprocal {
            args.fraction_query
        } else {
            args.fraction_target
        };
        match (args.fraction_query, fraction_target) {
            (Some(fraction_query), Some(fraction_target)) => {
                if args.either {
                    QueryMethod::CompareReciprocalFractionOr(fraction_query, fraction_target)
                } else {
                    QueryMethod::CompareReciprocalFractionAnd(fraction_query, fraction_target)
                }
            }
            (Some(fraction_query), None) => QueryMethod::CompareByQueryFraction(fraction_query),
            (None, Some(fraction_target)) => QueryMethod::CompareByTargetFraction(fraction_target),
            (None, None) => QueryMethod::Compare,
        }
    }
}
