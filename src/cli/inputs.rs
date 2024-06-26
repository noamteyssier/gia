use crate::{
    io::{match_bam_input, match_bcf_input, BedReader},
    types::{FieldFormat, InputFormat},
};
use anyhow::{bail, Result};
use clap::Parser;
use rust_htslib::{bam::Reader as BamReader, bcf::Reader as BcfReader};

#[derive(Parser, Debug, Clone)]
#[clap(next_help_heading = "Single Input Options")]
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

#[derive(Parser, Debug, Clone)]
#[clap(next_help_heading = "Single BAM Input Options")]
pub struct SingleInputBam {
    /// Input BAM file to process (default=stdin)
    #[clap(short, long)]
    pub input: Option<String>,
}

#[derive(Parser, Debug, Clone)]
#[clap(next_help_heading = "Mixed BAM/Bed Dual Input")]
pub struct MixedInputBam {
    /// Input BAM file to process (default=stdin)
    #[clap(short = 'a', long)]
    pub bam: Option<String>,
    /// Input BED file to process
    #[clap(short = 'b', long)]
    pub bed: String,
}
impl MixedInputBam {
    pub fn get_reader_bed(&self) -> Result<BedReader> {
        // The bed format must always be read as string-based when working with BAM files
        BedReader::from_path(Some(self.bed.clone()), None, Some(FieldFormat::StringBased))
    }

    pub fn get_reader_bam(&self) -> Result<BamReader> {
        match_bam_input(self.bam.clone())
    }
}

#[derive(Parser, Debug, Clone)]
#[clap(next_help_heading = "Mixed BAM/Bed Dual Input")]
pub struct MixedInputVcf {
    /// Input BCF/VCF file to process (default=stdin)
    #[clap(short = 'a', long)]
    pub bcf: Option<String>,
    /// Input BED file to process
    #[clap(short = 'b', long)]
    pub bed: String,
}
impl MixedInputVcf {
    pub fn get_reader_bed(&self) -> Result<BedReader> {
        // The bed format must always be read as string-based when working with BCF files
        BedReader::from_path(Some(self.bed.clone()), None, Some(FieldFormat::StringBased))
    }

    pub fn get_reader_bcf(&self) -> Result<BcfReader> {
        match_bcf_input(self.bcf.clone())
    }
}

#[derive(Parser, Debug, Clone)]
#[clap(next_help_heading = "Dual Input Options")]
pub struct DualInput {
    /// Primary BED file to use (default=stdin)
    #[clap(short, long)]
    pub a: Option<String>,

    /// Secondary BED file(s) to use
    ///
    ///
    /// Multiple BED files can be provided, mixed
    /// format input will be demoted to the lowest rank BED provided.
    #[clap(short, long, num_args = 1..)]
    pub b: Vec<String>,
}
impl DualInput {
    pub fn reader_from_a(&self) -> Result<BedReader> {
        BedReader::from_path(self.a.clone(), None, None)
    }
    pub fn readers_from_b(&self) -> Result<Vec<BedReader>> {
        let mut readers = vec![];
        for b in self.b.clone() {
            readers.push(BedReader::from_path(Some(b), None, None)?);
        }
        let minimum_rank = readers.iter().filter_map(|x| x.input_format().rank()).min();
        readers.clear();
        if let Some(minimum_rank) = minimum_rank {
            if let Some(in_format) = InputFormat::from_rank(minimum_rank) {
                for b in self.b.clone() {
                    readers.push(BedReader::from_path(Some(b), Some(in_format), None)?);
                }
                Ok(readers)
            } else {
                bail!("Cannot properly demote input format type - please merge your BED files manually");
            }
        } else {
            bail!("None of the provided input formats can be properly joined - please merge your input files manually")
        }
    }
    pub fn reader_from_b(&self) -> Result<BedReader> {
        BedReader::from_path(Some(self.b[0].clone()), None, None)
    }
    pub fn get_readers(self) -> Result<(BedReader, BedReader)> {
        let bed_a = self.reader_from_a()?;
        let bed_b = self.reader_from_b()?;
        if bed_a.is_named() != bed_b.is_named() {
            bail!("Input files must both be either named or unnamed");
        }
        Ok((self.reader_from_a()?, self.reader_from_b()?))
    }
    pub fn get_multi_readers(self) -> Result<(BedReader, Vec<BedReader>)> {
        let bed_a = self.reader_from_a()?;
        let bed_b = self.readers_from_b()?;
        for reader in bed_b.iter() {
            if bed_a.is_named() != reader.is_named() {
                bail!("Input files must all be either named or unnamed");
            }
        }
        Ok((self.reader_from_a()?, self.readers_from_b()?))
    }
    pub fn is_multi(&self) -> bool {
        self.b.len() > 1
    }
}

#[derive(Parser, Debug, Clone)]
#[clap(next_help_heading = "Multi Input Options")]
pub struct MultiInput {
    /// Input BED files to process
    #[clap(short, long, num_args=2.., required=true)]
    pub inputs: Vec<String>,
}
impl MultiInput {
    /// Get readers for all input files and ensure they are all named or unnamed
    pub fn get_readers(self) -> Result<Vec<BedReader>> {
        let mut readers = vec![];
        let mut named = None;
        for input in self.inputs {
            let reader = BedReader::from_path(Some(input), None, None)?;
            if let Some(named) = named {
                if named != reader.is_named() {
                    bail!("Input files must all be either named or unnamed");
                }
            } else {
                named = Some(reader.is_named());
            }
            readers.push(reader);
        }
        Ok(readers)
    }
}
