use anyhow::{bail, Result};
use clap::ValueEnum;
use std::{
    io::{BufRead, BufReader, Read},
    str::from_utf8,
};

/// Determines the input format of a file or stream.
///
/// Will read the first line of the file and count the number of fields.
///
/// Will *not* consume the first line of the file.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum InputFormat {
    #[default]
    Bed3,
    Bed6,
}
impl InputFormat {
    pub fn predict<R: Read>(reader: &mut BufReader<R>) -> Result<Self> {
        reader.fill_buf()?;
        let internal = reader.buffer();
        let first = internal.split(|b| *b == b'\n').next().unwrap();
        Self::predict_from_bytes(first)
    }

    pub fn predict_from_bytes(line: &[u8]) -> Result<Self> {
        let num_fields = line.split(|b| *b == b'\t').count();
        match num_fields {
            3 => Ok(Self::Bed3),
            6 => Ok(Self::Bed6),
            _ => bail!(
                "Cannot predict input format from line: {}",
                std::str::from_utf8(line)?
            ),
        }
    }
}

/// Determines the field format of a file or stream.
///
/// Will read the first line of the file and try to parse the first and fourth fields as integers.
/// (i.e. the chromosome and name fields of a bed[46] file)
///
/// Will *not* consume the first line of the file.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FieldFormat {
    #[default]
    IntegerBased,
    StringBased,
}
impl FieldFormat {
    pub fn predict<R: Read>(reader: &mut BufReader<R>) -> Result<Self> {
        reader.fill_buf()?;
        let internal = reader.buffer();
        let first = internal.split(|b| *b == b'\n').next().unwrap();
        Self::predict_from_bytes(first)
    }

    pub fn predict_from_bytes(line: &[u8]) -> Result<Self> {
        let input_format = InputFormat::predict_from_bytes(line)?;
        let mut fields = line.split(|b| *b == b'\t');
        match input_format {
            InputFormat::Bed6 => {
                let chr = from_utf8(fields.nth(0).unwrap())?;
                let name = from_utf8(fields.nth(2).unwrap())?;
                if chr.parse::<usize>().is_ok() && name.parse::<usize>().is_ok() {
                    Ok(Self::IntegerBased)
                } else {
                    Ok(Self::StringBased)
                }
            }
            InputFormat::Bed3 => {
                let chr = from_utf8(fields.nth(0).unwrap())?;
                if chr.parse::<usize>().is_ok() {
                    Ok(Self::IntegerBased)
                } else {
                    Ok(Self::StringBased)
                }
            }
        }
    }
}

#[cfg(test)]
mod testing {
    use std::io::BufReader;

    use super::*;

    #[test]
    fn no_consumption_input_format() {
        let lines = "1\t1\t2\n1\t3\t4\n1\t5\t6\n".as_bytes();
        let mut reader = BufReader::new(lines);
        let _input_format = InputFormat::predict(&mut reader).unwrap();
        let num_lines = reader.lines().count();
        assert_eq!(num_lines, 3);
    }

    #[test]
    fn no_consumption_field_format() {
        let lines = "1\t1\t2\n1\t3\t4\n1\t5\t6\n".as_bytes();
        let mut reader = BufReader::new(lines);
        let _field_format = FieldFormat::predict(&mut reader).unwrap();
        let num_lines = reader.lines().count();
        assert_eq!(num_lines, 3);
    }

    #[test]
    fn input_format_bed3() {
        let line = b"chr1\t1\t2";
        let input_format = InputFormat::predict_from_bytes(line).unwrap();
        assert_eq!(input_format, InputFormat::Bed3);
    }

    #[test]
    fn input_format_bed6() {
        let line = b"chr1\t1\t2\tname\t0\t+";
        let input_format = InputFormat::predict_from_bytes(line).unwrap();
        assert_eq!(input_format, InputFormat::Bed6);
    }

    #[test]
    fn input_format_unknown() {
        let line = b"chr1\t1\t2\tname\t0\t+\textra";
        let input_format = InputFormat::predict_from_bytes(line);
        assert!(input_format.is_err());
    }

    #[test]
    fn field_format_integer_based_bed3() {
        let line = b"1\t1\t2";
        let field_format = FieldFormat::predict_from_bytes(line).unwrap();
        assert_eq!(field_format, FieldFormat::IntegerBased);
    }

    #[test]
    fn field_format_string_based_bed3() {
        let line = b"chr1\t1\t2";
        let field_format = FieldFormat::predict_from_bytes(line).unwrap();
        assert_eq!(field_format, FieldFormat::StringBased);
    }

    #[test]
    fn field_format_integer_based_bed6() {
        let line = b"1\t1\t2\t1\t0\t+";
        let field_format = FieldFormat::predict_from_bytes(line).unwrap();
        assert_eq!(field_format, FieldFormat::IntegerBased);
    }

    #[test]
    fn field_format_string_based_bed6_a() {
        let line = b"chr1\t1\t2\tname\t0\t+";
        let field_format = FieldFormat::predict_from_bytes(line).unwrap();
        assert_eq!(field_format, FieldFormat::StringBased);
    }

    #[test]
    fn field_format_string_based_bed6_b() {
        let line = b"1\t1\t2\tname\t0\t+";
        let field_format = FieldFormat::predict_from_bytes(line).unwrap();
        assert_eq!(field_format, FieldFormat::StringBased);
    }

    #[test]
    fn field_format_string_based_bed6_c() {
        let line = b"chr1\t1\t2\t1\t0\t+";
        let field_format = FieldFormat::predict_from_bytes(line).unwrap();
        assert_eq!(field_format, FieldFormat::StringBased);
    }
}
