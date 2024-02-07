use super::{
    ClosestArgs, ComplementArgs, CoverageArgs, ExtendArgs, FlankArgs, GetFastaArgs, IntersectArgs,
    MergeArgs, RandomArgs, SampleArgs, ShiftArgs, SortArgs, WindowArgs,
};
use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
    /// Finds the closest interval in a secondary BED file for all intervals in a primary BED file
    Closest(ClosestArgs),

    /// Generates the complement of a BED file
    ///
    /// This reports the regions that are not covered by the input
    /// BED file but excludes regions preceding the first interval
    /// and following the last interval.
    Complement(ComplementArgs),

    /// Calculates the coverage of intervals in Set A by intervals in Set B
    Coverage(CoverageArgs),

    /// Extends the intervals of a BED file
    ///
    /// The extension is either done on both sides at once
    /// or on the left and right side separately
    Extend(ExtendArgs),

    /// Flanks the intervals of a BED file
    ///
    /// This will crefate two new flanking intervals for each interval
    /// in the input file, one on the left and one on the right side.
    Flank(FlankArgs),

    /// Extracts FASTA sequences using intervals from a BED file
    GetFasta(GetFastaArgs),

    /// Intersects two BED files
    Intersect(IntersectArgs),

    /// Merges intervals of a BED file with overlapping regions
    Merge(MergeArgs),

    /// Generates a random BED file given some parameterizations
    Random(RandomArgs),

    /// Randomly sample a BED file
    Sample(SampleArgs),

    /// Shifts the intervals of a BED file by a specified amount
    Shift(ShiftArgs),

    /// Sorts a BED file by chromosome, start, and end
    Sort(SortArgs),

    /// Subtracts two BED files
    ///
    /// Will remove subtract `b` from `a`
    Subtract {
        /// Input BED file to subtract from (default=stdin)
        #[clap(short, long)]
        a: Option<String>,

        /// Secondary BED file to subtract with
        #[clap(short, long)]
        b: String,

        /// Output BED file to write to (default=stdout)
        #[clap(short, long)]
        output: Option<String>,

        /// Minimum fraction of a's interval that must be covered by b's interval
        #[clap(short = 'f', long)]
        fraction_query: Option<f64>,

        /// Minimum fraction of b's interval that must be covered by a's interval
        #[clap(short = 'F', long)]
        fraction_target: Option<f64>,

        /// Require that the fraction provided with `-f` is reciprocal to both
        /// query and target
        #[clap(
            short,
            long,
            requires = "fraction_query",
            conflicts_with = "fraction_target"
        )]
        reciprocal: bool,

        /// Requires that either fraction provided with `-f` or `-F` is met
        #[clap(short, long, requires_all=&["fraction_query", "fraction_target"], conflicts_with = "reciprocal")]
        either: bool,

        /// Keep the query records unmerged (i.e. report all subtractions)
        ///
        /// By default, the query records are merged to remove overlapping
        /// regions.
        #[clap(short, long)]
        unmerged: bool,
    },

    /// Finds all the overlapping intervals in Set B after adding a window around all
    /// intervals in Set A
    Window(WindowArgs),
}
