use super::{
    read_bed12_set, read_bed12_set_with, read_bed3_set, read_bed3_set_with, read_bed6_set,
    read_bed6_set_with,
};
use crate::types::{Bed12Set, Bed3Set, Bed6Set, FieldFormat, InputFormat, Translater};
use anyhow::Result;
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
    input_format: InputFormat,
    field_format: FieldFormat,
}
impl BedReader {
    pub fn reader(self) -> BufReader<Box<dyn Read>> {
        self.reader
    }

    pub fn input_format(&self) -> InputFormat {
        self.input_format
    }

    pub fn is_named(&self) -> bool {
        self.field_format == FieldFormat::StringBased
    }

    /// Reads a BED file from a path and autodetects the compression and format
    pub fn from_path(
        input: Option<String>,
        input_format: Option<InputFormat>,
        field_format: Option<FieldFormat>,
    ) -> Result<Self> {
        let mut reader = Self::build_reader(input)?;
        reader.fill_buf()?;
        let input_format = match input_format {
            Some(f) => f,
            None => InputFormat::predict(&reader)?,
        };
        let field_format = match field_format {
            Some(f) => f,
            None => FieldFormat::predict(&reader, input_format)?,
        };
        Ok(Self {
            reader,
            input_format,
            field_format,
        })
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

    /// Returns a Bed3Set from the reader with an Option<Translater>
    pub fn bed3_set(self) -> Result<(Bed3Set, Option<Translater>)> {
        let is_named = self.is_named();
        read_bed3_set(self.reader(), is_named)
    }

    /// Returns a Bed6Set from the reader with an Option<Translater>
    pub fn bed6_set(self) -> Result<(Bed6Set, Option<Translater>)> {
        let is_named = self.is_named();
        read_bed6_set(self.reader(), is_named)
    }

    /// Returns a Bed6Set from the reader with an Option<Translater>
    pub fn bed12_set(self) -> Result<(Bed12Set, Option<Translater>)> {
        let is_named = self.is_named();
        read_bed12_set(self.reader(), is_named)
    }

    /// Returns a Bed3Set from the reader
    pub fn bed3_set_with(self, translater: Option<&mut Translater>) -> Result<Bed3Set> {
        read_bed3_set_with(self.reader(), translater)
    }

    /// Returns a Bed6Set from the reader
    pub fn bed6_set_with(self, translater: Option<&mut Translater>) -> Result<Bed6Set> {
        read_bed6_set_with(self.reader(), translater)
    }

    /// Returns a Bed6Set from the reader
    pub fn bed12_set_with(self, translater: Option<&mut Translater>) -> Result<Bed12Set> {
        read_bed12_set_with(self.reader(), translater)
    }
}
