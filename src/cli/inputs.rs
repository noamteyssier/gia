use crate::{
    io::BedReader,
    types::{FieldFormat, InputFormat},
};
use anyhow::{bail, Result};
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
impl SingleInput {
    pub fn get_reader(self) -> Result<BedReader> {
        BedReader::from_path(self.input, self.input_format, self.field_format)
    }
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
impl DualInput {
    pub fn reader_from_a(&self) -> Result<BedReader> {
        BedReader::from_path(self.a.clone(), None, None)
    }
    pub fn reader_from_b(&self) -> Result<BedReader> {
        BedReader::from_path(Some(self.b.clone()), None, None)
    }
    pub fn get_readers(self) -> Result<(BedReader, BedReader)> {
        let bed_a = self.reader_from_a()?;
        let bed_b = self.reader_from_b()?;
        if bed_a.is_named() != bed_b.is_named() {
            bail!("Input files must both be either named or unnamed");
        }
        Ok((self.reader_from_a()?, self.reader_from_b()?))
    }
}
