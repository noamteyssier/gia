use crate::types::InputFormat;
use anyhow::{bail, Result};
use flate2::read::MultiGzDecoder;
use gzp::BgzfSyncReader;
use std::{
    ffi::OsStr,
    fs::File,
    io::{BufRead, BufReader, Read},
    path::Path,
};
const DEFAULT_BUFFER_SIZE: usize = 8 * 1024;

/// The main module for reading BED files and interpreting them as a stream of records
///
/// This will autodetect the compression of the input file as well as the format of the file
/// and provide a stream of records that can be used to process the file.
pub struct BedReader {
    reader: BufReader<Box<dyn Read>>,
    format: InputFormat,
}
impl BedReader {
    pub fn reader(self) -> BufReader<Box<dyn Read>> {
        self.reader
    }

    pub fn format(&self) -> InputFormat {
        self.format
    }

    /// Reads a BED file from a path and autodetects the compression and format
    pub fn from_path(input: Option<String>, format: Option<InputFormat>) -> Result<Self> {
        let mut reader = Self::build_reader(input)?;
        reader.fill_buf()?;
        let format = match format {
            Some(f) => f,
            None => predict_format(&reader)?,
        };
        Ok(Self { reader, format })
    }

    /// Builds the reader from a path or stdin
    fn build_reader(input: Option<String>) -> Result<BufReader<Box<dyn Read>>> {
        if let Some(path_name) = input {
            Self::compression_aware_read_buffer(Path::new(&path_name))
        } else {
            Self::stdin_read_buffer()
        }
    }

    /// Creates the decrompression reader based on the file extension
    fn get_reader(path: &Path) -> Result<Box<dyn Read>> {
        let file = File::open(path)?;
        if path.extension() == Some(OsStr::new("gz")) {
            let gzip = MultiGzDecoder::new(file);
            Ok(Box::new(gzip))
        } else if path.extension() == Some(OsStr::new("bgz")) {
            let bgzf = BgzfSyncReader::new(file);
            Ok(Box::new(bgzf))
        } else {
            Ok(Box::new(file))
        }
    }

    /// Creates a buffer reader that is compression aware
    fn compression_aware_read_buffer(path: &Path) -> Result<BufReader<Box<dyn Read>>> {
        let reader = Self::get_reader(path)?;
        let buffer = BufReader::with_capacity(DEFAULT_BUFFER_SIZE, reader);
        Ok(buffer)
    }

    /// Creates a buffer reader for stdin
    fn stdin_read_buffer() -> Result<BufReader<Box<dyn Read>>> {
        let stdin = std::io::stdin();
        let handle = stdin.lock();
        Ok(BufReader::with_capacity(
            DEFAULT_BUFFER_SIZE,
            Box::new(handle),
        ))
    }
}

pub fn predict_format<R>(bufreader: &BufReader<R>) -> Result<InputFormat> {
    let internal = bufreader.buffer();
    let first = if let Some(first) = internal.split(|b| *b == b'\n').next() {
        first
    } else {
        bail!("Empty file or stream")
    };
    let num_fields = first.split(|b| *b == b'\t').count();
    match num_fields {
        3 => Ok(InputFormat::Bed3),
        6 => Ok(InputFormat::Bed6),
        12 => Ok(InputFormat::Bed12),
        _ => bail!(
            "Cannot predict input format from line: {}",
            std::str::from_utf8(first)?
        ),
    }
}
