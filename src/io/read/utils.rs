use super::{read_bed3_set_named, read_bed3_set_unnamed, SetFormat};
use crate::types::{FieldFormat, InputFormat, Translater};
use anyhow::Result;
use std::io::{BufReader, Read};

pub fn build_reader<R: Read>(reader: R) -> csv::Reader<R> {
    csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .comment(Some(b'#'))
        .from_reader(reader)
}

#[allow(dead_code)]
pub fn read_format_set_with<R: Read>(reader: R) -> Result<(SetFormat, Option<Translater>)> {
    let mut buffer = BufReader::new(reader);
    let input_format = InputFormat::predict(&mut buffer)?;
    let field_format = FieldFormat::predict(&mut buffer)?;
    match input_format {
        InputFormat::Bed3 => match field_format {
            FieldFormat::StringBased => {
                let (set, translater) = read_bed3_set_named(buffer.into_inner())?;
                Ok((SetFormat::Bed3(set), Some(translater)))
            }
            FieldFormat::IntegerBased => {
                let set = read_bed3_set_unnamed(buffer.into_inner())?;
                Ok((SetFormat::Bed3(set), None))
            }
        },
        _ => unimplemented!("Only BED3 is currently supported"),
    }
}
