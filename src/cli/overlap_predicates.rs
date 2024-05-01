use bedrs::{
    traits::ValueBounds,
    types::{Query, QueryMethod, StrandMethod},
};
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug, Clone, Copy)]
#[clap(next_help_heading = "Overlap Predicates")]
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

    /// Strand-specificity to use when comparing intervals
    ///
    /// i: Ignore strand (default)
    ///
    /// m: Match strand (+/+ or -/- only)
    ///
    /// o: Opposite strand (+/- or -/+ only)
    #[clap(short = 's', long, default_value = "i")]
    pub strandedness: WrapStrandedness,
}
impl OverlapPredicates {
    fn get_strand_method(&self) -> StrandMethod {
        self.strandedness.into()
    }
    fn get_overlap_method<T: ValueBounds>(&self) -> QueryMethod<T> {
        let fraction_target = if self.reciprocal {
            self.fraction_query
        } else {
            self.fraction_target
        };
        match (self.fraction_query, fraction_target) {
            (Some(fraction_query), Some(fraction_target)) => {
                if self.either {
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

#[derive(Parser, Debug, Clone, Copy, ValueEnum)]
pub enum WrapStrandedness {
    #[clap(name = "i")]
    Ignore,
    #[clap(name = "m")]
    Match,
    #[clap(name = "o")]
    Opposite,
}
impl From<WrapStrandedness> for StrandMethod {
    fn from(wrap: WrapStrandedness) -> Self {
        match wrap {
            WrapStrandedness::Ignore => StrandMethod::Ignore,
            WrapStrandedness::Match => StrandMethod::MatchStrand,
            WrapStrandedness::Opposite => StrandMethod::OppositeStrand,
        }
    }
}

impl<T: ValueBounds> From<OverlapPredicates> for Query<T> {
    fn from(args: OverlapPredicates) -> Self {
        Query::new(args.get_overlap_method(), args.get_strand_method())
    }
}
