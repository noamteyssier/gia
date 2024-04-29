use crate::{
    cli::GetFastaArgs,
    io::build_reader,
    types::{Header, InputFormat, NamedBed12, NamedBed3, NamedBed4, NamedBed6},
};
use anyhow::Result;
use bedrs::{Coordinates, Strand};
use csv::ByteRecord;
use faiquery::{FastaIndex, IndexedFasta};
use std::io::{Read, Write};

use super::utils::write_sequence;

fn build_fasta_index(fasta: &str) -> Result<FastaIndex> {
    let index_path = format!("{}.fai", fasta);
    FastaIndex::from_filepath(&index_path)
}

fn write_fasta<'a, I, W>(
    record: &I,
    fasta: &IndexedFasta,
    strandedness: bool,
    shared_buffer: &mut Vec<u8>,
    mut output: W,
) -> Result<()>
where
    I: Coordinates<&'a str, usize> + Header,
    W: Write,
{
    let revcomp = strandedness && matches!(record.strand(), Some(Strand::Reverse));
    if let Ok(buffer) = fasta.query_buffer(record.chr(), record.start(), record.end()) {
        record.write_header(&mut output)?;
        write_sequence(shared_buffer, buffer, revcomp, &mut output)?;
    }
    Ok(())
}

fn dispatch_get_fasta<R: Read, W: Write>(
    format: InputFormat,
    csv_reader: &mut csv::Reader<R>,
    byterecord: &mut ByteRecord,
    fasta: IndexedFasta,
    strandedness: bool,
    mut output: W,
) -> Result<()> {
    let mut shared_buffer = Vec::new();
    while csv_reader.read_byte_record(byterecord)? {
        match format {
            InputFormat::Bed3 => {
                let record: NamedBed3 = byterecord.deserialize(None)?;
                write_fasta(
                    &record,
                    &fasta,
                    strandedness,
                    &mut shared_buffer,
                    &mut output,
                )?;
            }
            InputFormat::Bed4 => {
                let record: NamedBed4 = byterecord.deserialize(None)?;
                write_fasta(
                    &record,
                    &fasta,
                    strandedness,
                    &mut shared_buffer,
                    &mut output,
                )?;
            }
            InputFormat::Bed6 => {
                let record: NamedBed6 = byterecord.deserialize(None)?;
                write_fasta(
                    &record,
                    &fasta,
                    strandedness,
                    &mut shared_buffer,
                    &mut output,
                )?;
            }
            InputFormat::Bed12 => {
                let record: NamedBed12 = byterecord.deserialize(None)?;
                write_fasta(
                    &record,
                    &fasta,
                    strandedness,
                    &mut shared_buffer,
                    &mut output,
                )?;
            }
            _ => anyhow::bail!("Unable to process ambiguous input format"),
        }
    }
    Ok(())
}

pub fn get_text_fasta(args: GetFastaArgs) -> Result<()> {
    let reader = args.input.get_reader()?;
    let writer = args.output.get_writer()?;
    let fasta_index = build_fasta_index(&args.fasta)?;
    let fasta = IndexedFasta::new(fasta_index, &args.fasta)?;
    let format = reader.input_format();
    let mut csv_reader = build_reader(reader.reader());
    let mut byterecord = ByteRecord::new();
    dispatch_get_fasta(
        format,
        &mut csv_reader,
        &mut byterecord,
        fasta,
        args.stranded,
        writer,
    )
}
