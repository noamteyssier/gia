use super::FilterArgs;
use clap::Parser;

#[derive(Parser, Debug)]
pub enum BcfCommand {
    /// Filter BCF records based on overlap criteria to other regions
    Filter(FilterArgs),
}
