use anyhow::Result;
use bedrs::{Container, GenomicInterval, GenomicIntervalSet};
use csv::Writer;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Read, Write},
};

pub fn match_input(input: Option<String>) -> Result<Box<dyn BufRead>> {
    match input {
        Some(filename) => {
            let file = File::open(filename)?;
            let buffer = BufReader::new(file);
            Ok(Box::new(buffer))
        }
        None => {
            let stdin = std::io::stdin();
            let handle = stdin.lock();
            let buffer = BufReader::new(handle);
            Ok(Box::new(buffer))
        }
    }
}

pub fn match_output(output: Option<String>) -> Result<Box<dyn Write>> {
    match output {
        Some(filename) => {
            let file = File::create(filename)?;
            let buffer = BufWriter::new(file);
            Ok(Box::new(buffer))
        }
        None => {
            let stdout = std::io::stdout();
            let handle = stdout.lock();
            let buffer = BufWriter::new(handle);
            Ok(Box::new(buffer))
        }
    }
}

pub fn read_set<R: Read>(reader: R) -> Result<GenomicIntervalSet<usize>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .comment(Some(b'#'))
        .from_reader(reader);
    let set = reader
        .deserialize()
        .map(|record| {
            let record: GenomicInterval<usize> = record?;
            Ok(record)
        })
        .collect::<Result<GenomicIntervalSet<usize>>>()?;
    Ok(set)
}

pub fn write_set<W: Write>(set: &GenomicIntervalSet<usize>, writer: W) -> Result<()> {
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_writer(writer);
    write_internal(set.records(), &mut wtr)?;
    wtr.flush()?;
    Ok(())
}

pub fn write_records<W: Write>(records: &[GenomicInterval<usize>], writer: W) -> Result<()> {
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_writer(writer);
    write_internal(records, &mut wtr)?;
    wtr.flush()?;
    Ok(())
}

fn write_internal<W: Write>(records: &[GenomicInterval<usize>], wtr: &mut Writer<W>) -> Result<()> {
    for interval in records.iter() {
        wtr.serialize(interval)?;
    }
    Ok(())
}
