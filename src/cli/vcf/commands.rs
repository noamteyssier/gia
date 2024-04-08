use super::FilterArgs;
use clap::Parser;

#[derive(Parser, Debug)]
pub enum VcfCommand {
    /// Filter VCF records based on overlap criteria to other regions
    Filter(FilterArgs),
}
