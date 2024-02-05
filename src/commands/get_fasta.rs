use crate::{
    io::{build_reader, match_output, BedReader},
    types::{InputFormat, NamedBed12, NamedBed3, NamedBed6},
};
use anyhow::Result;
use bedrs::Coordinates;
use bstr::ByteSlice;
use csv::ByteRecord;
use faiquery::{FastaIndex, IndexedFasta};
use std::io::{Read, Write};

fn build_fasta_index(fasta: &str) -> Result<FastaIndex> {
    let index_path = format!("{}.fai", fasta);
    FastaIndex::from_filepath(&index_path)
}

fn get_fasta_bed3<R: Read, W: Write>(
    csv_reader: &mut csv::Reader<R>,
    byterecord: &mut ByteRecord,
    fasta: IndexedFasta,
    output: &mut W,
) -> Result<()> {
    while csv_reader.read_byte_record(byterecord)? {
        let record: NamedBed3 = byterecord.deserialize(None)?;
        match fasta.query_buffer(record.chr(), record.start(), record.end()) {
            Ok(buffer) => {
                writeln!(
                    output,
                    ">{}:{}-{}",
                    record.chr(),
                    record.start(),
                    record.end()
                )?;
                for subseq in buffer.split_str("\n") {
                    output.write_all(subseq)?;
                }
                output.write_all(b"\n")?;
            }
            Err(_) => continue,
        }
    }
    Ok(())
}

fn get_fasta_bed6<R: Read, W: Write>(
    csv_reader: &mut csv::Reader<R>,
    byterecord: &mut ByteRecord,
    fasta: IndexedFasta,
    output: &mut W,
) -> Result<()> {
    while csv_reader.read_byte_record(byterecord)? {
        let record: NamedBed6 = byterecord.deserialize(None)?;
        match fasta.query_buffer(record.chr(), record.start(), record.end()) {
            Ok(buffer) => {
                writeln!(
                    output,
                    ">{}:{}-{}::{}::{}::{}",
                    record.chr(),
                    record.start(),
                    record.end(),
                    record.name(),
                    record.score(),
                    record.strand().unwrap_or_default(),
                )?;
                for subseq in buffer.split_str("\n") {
                    output.write_all(subseq)?;
                }
                output.write_all(b"\n")?;
            }
            Err(_) => continue,
        }
    }
    Ok(())
}

fn get_fasta_bed12<R: Read, W: Write>(
    csv_reader: &mut csv::Reader<R>,
    byterecord: &mut ByteRecord,
    fasta: IndexedFasta,
    output: &mut W,
) -> Result<()> {
    while csv_reader.read_byte_record(byterecord)? {
        let record: NamedBed12 = byterecord.deserialize(None)?;
        match fasta.query_buffer(record.chr(), record.start(), record.end()) {
            Ok(buffer) => {
                writeln!(
                    output,
                    ">{}:{}-{}::{}::{}::{}::{}::{}::{}::{}::{}::{}",
                    record.chr(),
                    record.start(),
                    record.end(),
                    record.name(),
                    record.score(),
                    record.strand().unwrap_or_default(),
                    record.thick_start(),
                    record.thick_end(),
                    record.item_rgb(),
                    record.block_count(),
                    record.block_sizes(),
                    record.block_starts(),
                )?;
                for subseq in buffer.split_str("\n") {
                    output.write_all(subseq)?;
                }
                output.write_all(b"\n")?;
            }
            Err(_) => continue,
        }
    }
    Ok(())
}

pub fn get_fasta(
    bed: Option<String>,
    fasta: &str,
    output: Option<String>,
    input_format: Option<InputFormat>,
    compression_threads: usize,
    compression_level: u32,
) -> Result<()> {
    let bed_reader = BedReader::from_path(bed, input_format, None)?;
    let fasta_index = build_fasta_index(fasta)?;
    let fasta = IndexedFasta::new(fasta_index, fasta)?;
    let input_format = bed_reader.input_format();
    let mut csv_reader = build_reader(bed_reader.reader());
    let mut byterecord = ByteRecord::new();
    let mut output = match_output(output, compression_threads, compression_level)?;
    match input_format {
        InputFormat::Bed3 => get_fasta_bed3(&mut csv_reader, &mut byterecord, fasta, &mut output),
        InputFormat::Bed6 => get_fasta_bed6(&mut csv_reader, &mut byterecord, fasta, &mut output),
        InputFormat::Bed12 => get_fasta_bed12(&mut csv_reader, &mut byterecord, fasta, &mut output),
    }
}
