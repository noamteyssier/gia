use crate::io::match_output;
use anyhow::Result;
use clap::Parser;
use std::io::Write;

#[derive(Parser, Debug, Clone)]
#[clap(next_help_heading = "Output Options")]
pub struct Output {
    /// Output BED file to write to (default=stdout)
    #[clap(short, long)]
    pub output: Option<String>,

    /// Compression threads to use for output files if applicable
    #[clap(global = true, short = 'j', long, default_value = "1")]
    pub compression_threads: usize,

    /// Compression level to use for output files if applicable
    #[clap(global = true, long, default_value = "6")]
    pub compression_level: u32,
}
impl Output {
    pub fn get_writer(&self) -> Result<Box<dyn Write>> {
        match_output(
            self.output.clone(),
            self.compression_threads,
            self.compression_level,
        )
    }
}
