use super::{
    ClosestArgs, ComplementArgs, CoverageArgs, ExtendArgs, FlankArgs, GetFastaArgs, IntersectArgs,
    MergeArgs, RandomArgs, SampleArgs, SegmentArgs, ShiftArgs, SortArgs, SpacingArgs, SubtractArgs,
    WindowArgs,
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

    /// Segments a BED file into non-overlapping regions
    Segment(SegmentArgs),

    /// Shifts the intervals of a BED file by a specified amount
    Shift(ShiftArgs),

    /// Sorts a BED file by chromosome, start, and end
    Sort(SortArgs),

    /// Calculates the spacing between intervals in a BED file
    Spacing(SpacingArgs),

    /// Subtracts two BED files
    ///
    /// Will subtract `b` from `a`
    Subtract(SubtractArgs),

    /// Finds all the overlapping intervals in Set B after adding a window around all
    /// intervals in Set A
    Window(WindowArgs),
}
