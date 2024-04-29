use super::utils::write_sequence;
use crate::{
    cli::{GetFastaArgs, GetFastaParams},
    io::build_reader,
    types::{Header, InputFormat, NamedBed12, NamedBed3, NamedBed4, NamedBed6},
};
use anyhow::Result;
use bedrs::{Coordinates, Strand};
use csv::ByteRecord;
use hashbrown::HashSet;
use rust_htslib::faidx::Reader;
use std::io::{Read, Write};

fn write_fasta_gzip<'a, I, W>(
    seq_names: &HashSet<Vec<u8>>,
    record: &I,
    fasta: &Reader,
    params: GetFastaParams,
    shared_buffer: &mut Vec<u8>,
    mut output: W,
) -> Result<()>
where
    I: Coordinates<&'a str, usize> + Header,
    W: Write,
{
    if !seq_names.contains(record.chr().as_bytes()) {
        return Ok(());
    }
    let revcomp = params.stranded && matches!(record.strand(), Some(Strand::Reverse));
    // The BED format is 0-based, inclusive
    if let Ok(buffer) = fasta.fetch_seq(record.chr(), record.start(), record.end() - 1) {
        record.write_header(&mut output)?;
        write_sequence(shared_buffer, buffer, revcomp, params.rna, &mut output)?;
    }
    Ok(())
}
fn get_all_seqname(fasta: &Reader) -> HashSet<Vec<u8>> {
    (0..fasta.n_seqs())
        .map(|i| {
            fasta
                .seq_name(i as i32)
                .expect("Could not find sequence, bug in rust_htslib")
        })
        .map(|name| name.as_bytes().to_owned())
        .collect()
}

fn dispatch_get_fasta_gzip<R: Read, W: Write>(
    format: InputFormat,
    csv_reader: &mut csv::Reader<R>,
    byterecord: &mut ByteRecord,
    fasta_reader: &Reader,
    seq_names: &HashSet<Vec<u8>>,
    params: GetFastaParams,
    mut output: W,
) -> Result<()> {
    let mut shared_buffer = Vec::new();
    while csv_reader.read_byte_record(byterecord)? {
        match format {
            InputFormat::Bed3 => {
                let record: NamedBed3 = byterecord.deserialize(None)?;
                write_fasta_gzip(
                    seq_names,
                    &record,
                    fasta_reader,
                    params,
                    &mut shared_buffer,
                    &mut output,
                )?;
            }
            InputFormat::Bed4 => {
                let record: NamedBed4 = byterecord.deserialize(None)?;
                write_fasta_gzip(
                    seq_names,
                    &record,
                    fasta_reader,
                    params,
                    &mut shared_buffer,
                    &mut output,
                )?;
            }
            InputFormat::Bed6 => {
                let record: NamedBed6 = byterecord.deserialize(None)?;
                write_fasta_gzip(
                    seq_names,
                    &record,
                    fasta_reader,
                    params,
                    &mut shared_buffer,
                    &mut output,
                )?;
            }
            InputFormat::Bed12 => {
                let record: NamedBed12 = byterecord.deserialize(None)?;
                write_fasta_gzip(
                    seq_names,
                    &record,
                    fasta_reader,
                    params,
                    &mut shared_buffer,
                    &mut output,
                )?;
            }
            _ => anyhow::bail!("Unable to process ambiguous input format"),
        }
    }
    Ok(())
}

pub fn get_gzip_fasta(args: GetFastaArgs) -> Result<()> {
    let reader = Reader::from_path(args.fasta)?;
    let writer = args.output.get_writer()?;
    let bed_reader = args.input.get_reader()?;
    let format = bed_reader.input_format();
    let mut csv_reader = build_reader(bed_reader.reader());
    let mut byterecord = ByteRecord::new();
    let seq_names = get_all_seqname(&reader);
    dispatch_get_fasta_gzip(
        format,
        &mut csv_reader,
        &mut byterecord,
        &reader,
        &seq_names,
        args.params,
        writer,
    )
}
