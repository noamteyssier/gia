use super::ConvertArgs;
use clap::Parser;

#[derive(Parser, Debug)]
pub enum BamCommand {
    /// Convert BAM to different formats
    Convert(ConvertArgs),
}
