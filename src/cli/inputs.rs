use crate::types::{FieldFormat, InputFormat};
use clap::Parser;

#[derive(Parser, Debug)]
pub struct SingleInput {
    /// Input BED file to process (default=stdin)
    #[clap(short, long)]
    pub input: Option<String>,

    /// Format of input file
    #[clap(short = 'T', long)]
    pub input_format: Option<InputFormat>,

    /// Allow for non-integer chromosome names
    #[clap(short = 'N', long)]
    pub field_format: Option<FieldFormat>,
}

#[derive(Parser, Debug)]
pub struct DualInput {
    /// Primary BED file to use (default=stdin)
    #[clap(short, long)]
    pub a: Option<String>,

    /// Secondary BED file to use
    #[clap(short, long)]
    pub b: String,
}
