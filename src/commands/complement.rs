use std::io::Write;

use crate::{
    cli::ComplementArgs,
    io::{
        build_reader, iter_unnamed, read_bed3_set, write_records_iter, write_records_iter_with,
        BedReader,
    },
    types::NumericBed3,
};
use anyhow::{bail, Result};
use bedrs::{types::iterator::ComplementIter, MergeIter};

fn complement_inplace<W: Write>(reader: BedReader, output: W) -> Result<()> {
    // Check if the input is named
    let named = reader.is_named();

    // Read records into a set
    let (mut iset, translater) = read_bed3_set(reader.reader(), named)?;

    // Sort the set
    iset.sort();

    // Merge the set
    let merged = iset.merge()?;

    // Complement the set
    let complement_iter = merged.complement()?;

    // Write the records
    write_records_iter_with(complement_iter, output, translater.as_ref())?;

    Ok(())
}

fn complement_stream<W: Write>(reader: BedReader, output: W) -> Result<()> {
    // Build the CSV reader
    let mut csv_reader = build_reader(reader.reader());

    // Build the record iterator
    let record_iter: Box<dyn Iterator<Item = NumericBed3>> = iter_unnamed(&mut csv_reader);

    // Pipe the record iterator into the merge iterator
    let merged_iter = MergeIter::new(record_iter);

    // Pipe the merge iterator into the complement iterator
    let comp_iter = ComplementIter::new(merged_iter);

    // Write the records
    write_records_iter(comp_iter, output)?;
    Ok(())
}

pub fn complement(args: ComplementArgs) -> Result<()> {
    let reader = args.input.get_reader()?;
    let output = args.output.get_writer()?;
    if args.stream {
        if reader.is_named() {
            bail!("Cannot currently stream named records in complement - in development");
        }
        complement_stream(reader, output)
    } else {
        complement_inplace(reader, output)
    }
}
