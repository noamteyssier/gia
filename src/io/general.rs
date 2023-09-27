use anyhow::Result;
use gzp::deflate::Bgzf;
use gzp::{Compression, ZBuilder};
use niffler::get_reader;
use std::ffi::OsStr;
use std::path::Path;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};
const COMPRESSION_THREADS: usize = 4;
const COMPRESSION_LEVEL: u32 = 6;

fn compression_aware_read_buffer(file: File) -> Result<Box<dyn BufRead>> {
    let buffer = BufReader::new(file);
    let (reader, _compression) = get_reader(Box::new(buffer))?;
    Ok(Box::new(BufReader::new(reader)))
}

pub fn match_input(input: Option<String>) -> Result<Box<dyn BufRead>> {
    match input {
        Some(filename) => {
            let file = File::open(filename)?;
            compression_aware_read_buffer(file)
        }
        None => {
            let stdin = std::io::stdin();
            let handle = stdin.lock();
            let buffer = BufReader::new(handle);
            Ok(Box::new(buffer))
        }
    }
}

fn compression_aware_write_buffer(filename: String) -> Result<Box<dyn Write>> {
    let file = File::create(filename.clone())?;
    let buffer = BufWriter::new(file);
    let ext = Path::new(&filename).extension();
    if ext == Some(OsStr::new("gz")) || ext == Some(OsStr::new("bgz")) {
        let writer = ZBuilder::<Bgzf, _>::new()
            .num_threads(COMPRESSION_THREADS)
            .compression_level(Compression::new(COMPRESSION_LEVEL))
            .from_writer(buffer);
        Ok(Box::new(writer))
    } else {
        Ok(Box::new(buffer))
    }
}

pub fn match_output(output: Option<String>) -> Result<Box<dyn Write>> {
    match output {
        Some(filename) => compression_aware_write_buffer(filename),
        None => {
            let stdout = std::io::stdout();
            let buffer = BufWriter::new(stdout);
            Ok(Box::new(buffer))
        }
    }
}
