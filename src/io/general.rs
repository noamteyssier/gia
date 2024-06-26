use anyhow::{bail, Result};
use gzp::deflate::Bgzf;
use gzp::{Compression, ZBuilder};
use niffler::get_reader;
use rust_htslib::{
    bam::{Format as SamFormat, Header, HeaderView, Reader as BamReader, Writer as BamWriter},
    bcf::{
        header::HeaderView as BcfHeaderView, Format as BcfFormat, Header as BcfHeader,
        Reader as BcfReader, Writer as BcfWriter,
    },
};
use std::ffi::OsStr;
use std::path::Path;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

fn compression_aware_read_buffer(file: File) -> Result<Box<dyn BufRead>> {
    let (reader, _compression) = get_reader(Box::new(file))?;
    let mut buffer = BufReader::new(reader);
    buffer.fill_buf()?;
    Ok(Box::new(buffer))
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
            let mut buffer = BufReader::new(handle);
            buffer.fill_buf()?;
            Ok(Box::new(buffer))
        }
    }
}

pub fn match_bam_input(input: Option<String>) -> Result<BamReader> {
    match input {
        Some(filename) => Ok(BamReader::from_path(filename)?),
        None => Ok(BamReader::from_stdin()?),
    }
}

pub fn match_bcf_input(input: Option<String>) -> Result<BcfReader> {
    match input {
        Some(filename) => Ok(BcfReader::from_path(filename)?),
        None => Ok(BcfReader::from_stdin()?),
    }
}

fn compression_aware_write_buffer(
    filename: String,
    compression_threads: usize,
    compression_level: u32,
) -> Result<Box<dyn Write>> {
    let file = File::create(filename.clone())?;
    let buffer = BufWriter::new(file);
    let ext = Path::new(&filename).extension();
    if ext == Some(OsStr::new("gz")) || ext == Some(OsStr::new("bgz")) {
        let writer = ZBuilder::<Bgzf, _>::new()
            .num_threads(compression_threads)
            .compression_level(Compression::new(compression_level))
            .from_writer(buffer);
        Ok(Box::new(writer))
    } else {
        Ok(Box::new(buffer))
    }
}

pub fn match_output(
    output: Option<String>,
    compression_threads: usize,
    compression_level: u32,
) -> Result<Box<dyn Write>> {
    match output {
        Some(filename) => {
            compression_aware_write_buffer(filename, compression_threads, compression_level)
        }
        None => {
            let stdout = std::io::stdout();
            let buffer = BufWriter::new(stdout);
            Ok(Box::new(buffer))
        }
    }
}

pub fn match_output_mt(output: Option<String>) -> Result<Box<dyn Write + Send + Sync>> {
    match output {
        Some(filename) => {
            let file = File::create(filename.clone())?;
            let buffer = BufWriter::new(file);
            let ext = Path::new(&filename).extension();
            if ext == Some(OsStr::new("gz")) || ext == Some(OsStr::new("bgz")) {
                bail!("Multithreaded compression is not supported for bgzip output files currently")
            } else {
                Ok(Box::new(buffer))
            }
        }
        None => {
            let stdout = std::io::stdout();
            let buffer = BufWriter::new(stdout);
            Ok(Box::new(buffer))
        }
    }
}

pub fn match_bam_output(
    path: Option<String>,
    header: &HeaderView,
    format: SamFormat,
    n_threads: usize,
) -> Result<BamWriter> {
    let mut writer = if let Some(filename) = path {
        BamWriter::from_path(filename, &Header::from_template(header), format)
    } else {
        BamWriter::from_stdout(&Header::from_template(header), format)
    }?;
    writer.set_threads(n_threads)?;
    Ok(writer)
}

pub fn match_bcf_output(
    path: Option<String>,
    header: &BcfHeaderView,
    format: BcfFormat,
    compressed: bool,
    n_threads: usize,
) -> Result<BcfWriter> {
    let mut writer = if let Some(filename) = path {
        BcfWriter::from_path(
            filename,
            &BcfHeader::from_template(header),
            !compressed,
            format,
        )
    } else {
        BcfWriter::from_stdout(&BcfHeader::from_template(header), !compressed, format)
    }?;
    writer.set_threads(n_threads)?;
    Ok(writer)
}

#[cfg(test)]
mod testing {

    use std::fs::remove_file;

    use super::*;

    const TEST_BED3: &str = "tests/datasets/io/bed3.bed";
    const TEST_BED3_GZ: &str = "tests/datasets/io/bed3.bed.gz";
    const TEST_BED3_BGZ: &str = "tests/datasets/io/bed3.bed.bgz";

    const TEST_BED6: &str = "tests/datasets/io/bed6.bed";
    const TEST_BED6_GZ: &str = "tests/datasets/io/bed6.bed.gz";
    const TEST_BED6_BGZ: &str = "tests/datasets/io/bed6.bed.bgz";

    const EXP_SIZE: usize = 10;

    #[test]
    fn test_read_bed3_plaintext() {
        let filename = TEST_BED3.to_string();
        let handle = match_input(Some(filename)).unwrap();
        let lines = handle.lines().count();
        assert_eq!(lines, EXP_SIZE);
    }

    #[test]
    fn test_read_bed3_gz() {
        let filename = TEST_BED3_GZ.to_string();
        let handle = match_input(Some(filename)).unwrap();
        let lines = handle.lines().count();
        assert_eq!(lines, EXP_SIZE);
    }

    #[test]
    fn test_read_bed3_bgz() {
        let filename = TEST_BED3_BGZ.to_string();
        let handle = match_input(Some(filename)).unwrap();
        let lines = handle.lines().count();
        assert_eq!(lines, EXP_SIZE);
    }

    #[test]
    fn test_read_bed6_plaintext() {
        let filename = TEST_BED6.to_string();
        let handle = match_input(Some(filename)).unwrap();
        let lines = handle.lines().count();
        assert_eq!(lines, EXP_SIZE);
    }

    #[test]
    fn test_read_bed6_gz() {
        let filename = TEST_BED6_GZ.to_string();
        let handle = match_input(Some(filename)).unwrap();
        let lines = handle.lines().count();
        assert_eq!(lines, EXP_SIZE);
    }

    #[test]
    fn test_read_bed6_bgz() {
        let filename = TEST_BED6_BGZ.to_string();
        let handle = match_input(Some(filename)).unwrap();
        let lines = handle.lines().count();
        assert_eq!(lines, EXP_SIZE);
    }

    #[test]
    fn test_write_bed3_plaintext() {
        let filename = "null.bed3".to_string();
        let handle = match_output(Some(filename), 1, 1).unwrap();
        let mut writer = BufWriter::new(handle);
        for _ in 0..EXP_SIZE {
            writer.write_all(b"chr1\t1\t2\n").unwrap();
        }
        remove_file("null.bed3").unwrap();
    }

    #[test]
    fn test_write_bed3_gz() {
        let filename = "null.bed3.gz".to_string();
        let handle = match_output(Some(filename), 1, 1).unwrap();
        let mut writer = BufWriter::new(handle);
        for _ in 0..EXP_SIZE {
            writer.write_all(b"chr1\t1\t2\n").unwrap();
        }
        remove_file("null.bed3.gz").unwrap();
    }

    #[test]
    fn test_write_bed3_bgz() {
        let filename = "null.bed3.bgz".to_string();
        let handle = match_output(Some(filename), 1, 1).unwrap();
        let mut writer = BufWriter::new(handle);
        for _ in 0..EXP_SIZE {
            writer.write_all(b"chr1\t1\t2\n").unwrap();
        }
        remove_file("null.bed3.bgz").unwrap();
    }

    #[test]
    fn test_write_bed6_plaintext() {
        let filename = "null.bed6".to_string();
        let handle = match_output(Some(filename), 1, 1).unwrap();
        let mut writer = BufWriter::new(handle);
        for _ in 0..EXP_SIZE {
            writer.write_all(b"chr1\t1\t2\tname\t0\t+\n").unwrap();
        }
        remove_file("null.bed6").unwrap();
    }

    #[test]
    fn test_write_bed6_gz() {
        let filename = "null.bed6.gz".to_string();
        let handle = match_output(Some(filename), 1, 1).unwrap();
        let mut writer = BufWriter::new(handle);
        for _ in 0..EXP_SIZE {
            writer.write_all(b"chr1\t1\t2\tname\t0\t+\n").unwrap();
        }
        remove_file("null.bed6.gz").unwrap();
    }

    #[test]
    fn test_write_bed6_bgz() {
        let filename = "null.bed6.bgz".to_string();
        let handle = match_output(Some(filename), 1, 1).unwrap();
        let mut writer = BufWriter::new(handle);
        for _ in 0..EXP_SIZE {
            writer.write_all(b"chr1\t1\t2\tname\t0\t+\n").unwrap();
        }
        remove_file("null.bed6.bgz").unwrap();
    }
}
