use anyhow::{bail, Result};
use clap::ValueEnum;
use std::{io::BufReader, str::from_utf8};

/// Determines the input format of a file or stream.
///
/// Will read the first line of the file and count the number of fields.
///
/// Will *not* consume the first line of the file.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum InputFormat {
    #[default]
    Bed3,
    Bed4,
    Bed6,
    Bed12,
    Ambiguous,
}
impl InputFormat {
    pub fn predict<R>(bufreader: &BufReader<R>) -> Result<InputFormat> {
        let internal = bufreader.buffer();
        if internal.is_empty() {
            bail!("Empty file or stream or buffer")
        }
        let first = if let Some(first) = internal.split(|b| *b == b'\n').next() {
            first
        } else {
            bail!("File missing newline, cannot predict input format")
        };
        let num_fields = first.split(|b| *b == b'\t').count();
        match num_fields {
            1..=2 => bail!("Too few fields in line: {}", from_utf8(first)?),
            3 => Ok(InputFormat::Bed3),
            4 => Ok(InputFormat::Bed4),
            6 => Ok(InputFormat::Bed6),
            12 => Ok(InputFormat::Bed12),
            _ => Ok(InputFormat::Ambiguous),
        }
    }
}

/// Determines the field format of a file or stream.
///
/// Will read the first line of the file and try to parse the first and fourth fields as integers.
/// (i.e. the chromosome and name fields of a bed[46] file)
///
/// Will *not* consume the first line of the file.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum FieldFormat {
    #[default]
    IntegerBased,
    StringBased,
}
impl FieldFormat {
    pub fn predict<R>(bufreader: &BufReader<R>, input_format: InputFormat) -> Result<FieldFormat> {
        let internal = bufreader.buffer();
        let first = if let Some(first) = internal.split(|b| *b == b'\n').next() {
            first
        } else {
            bail!("Empty file or stream")
        };
        let fields = first.split(|b| *b == b'\t').collect::<Vec<_>>();
        match input_format {
            InputFormat::Bed3 => {
                let chr = from_utf8(fields[0])?;
                if chr.parse::<u32>().is_err() {
                    Ok(FieldFormat::StringBased)
                } else {
                    Ok(FieldFormat::IntegerBased)
                }
            }
            InputFormat::Bed4 | InputFormat::Bed6 | InputFormat::Bed12 => {
                let chr = from_utf8(fields[0])?;
                let name = from_utf8(fields[3])?;
                if chr.parse::<u32>().is_err() || name.parse::<u32>().is_err() {
                    Ok(FieldFormat::StringBased)
                } else {
                    Ok(FieldFormat::IntegerBased)
                }
            }
            InputFormat::Ambiguous => {
                let all_int = fields
                    .iter()
                    .filter_map(|f| from_utf8(f).ok())
                    .all(|f| f.parse::<u32>().is_ok());
                if all_int {
                    Ok(FieldFormat::IntegerBased)
                } else {
                    Ok(FieldFormat::StringBased)
                }
            }
        }
    }
}

#[cfg(test)]
mod testing {
    use std::io::{BufRead, BufReader};

    use super::*;

    #[test]
    fn no_consumption_input_format() {
        let lines = "1\t1\t2\n1\t3\t4\n1\t5\t6\n".as_bytes();
        let mut reader = BufReader::new(lines);
        reader.fill_buf().unwrap();
        let _input_format = InputFormat::predict(&reader).unwrap();
        let num_lines = reader.lines().count();
        assert_eq!(num_lines, 3);
    }

    #[test]
    fn no_consumption_field_format() {
        let lines = "1\t1\t2\n1\t3\t4\n1\t5\t6\n".as_bytes();
        let reader = BufReader::new(lines);
        let _field_format = FieldFormat::predict(&reader, InputFormat::Bed3).unwrap();
        let num_lines = reader.lines().count();
        assert_eq!(num_lines, 3);
    }

    #[test]
    fn input_format_bed3() {
        let line = b"chr1\t1\t2";
        let mut buffer = BufReader::new(line.as_slice());
        buffer.fill_buf().unwrap();
        let input_format = InputFormat::predict(&buffer).unwrap();
        assert_eq!(input_format, InputFormat::Bed3);
    }

    #[test]
    fn input_format_bed6() {
        let line = b"chr1\t1\t2\tname\t0\t+";
        let mut buffer = BufReader::new(line.as_slice());
        buffer.fill_buf().unwrap();
        let input_format = InputFormat::predict(&buffer).unwrap();
        assert_eq!(input_format, InputFormat::Bed6);
    }

    #[test]
    fn input_format_unknown() {
        let line = b"chr1\t1\t2\tname\t0\t+\textra";
        let mut buffer = BufReader::new(line.as_slice());
        buffer.fill_buf().unwrap();
        let input_format = InputFormat::predict(&buffer);
        assert!(input_format.is_err());
    }

    #[test]
    fn field_format_integer_based_bed3() {
        let line = b"1\t1\t2";
        let mut buffer = BufReader::new(line.as_slice());
        buffer.fill_buf().unwrap();
        let field_format = FieldFormat::predict(&buffer, InputFormat::Bed3).unwrap();
        assert_eq!(field_format, FieldFormat::IntegerBased);
    }

    #[test]
    fn field_format_string_based_bed3() {
        let line = b"chr1\t1\t2";
        let mut buffer = BufReader::new(line.as_slice());
        buffer.fill_buf().unwrap();
        let field_format = FieldFormat::predict(&buffer, InputFormat::Bed3).unwrap();
        assert_eq!(field_format, FieldFormat::StringBased);
    }

    #[test]
    fn field_format_integer_based_bed6() {
        let line = b"1\t1\t2\t1\t0\t+";
        let mut buffer = BufReader::new(line.as_slice());
        buffer.fill_buf().unwrap();
        let field_format = FieldFormat::predict(&buffer, InputFormat::Bed6).unwrap();
        assert_eq!(field_format, FieldFormat::IntegerBased);
    }

    #[test]
    fn field_format_string_based_bed6_a() {
        let line = b"chr1\t1\t2\tname\t0\t+";
        let mut buffer = BufReader::new(line.as_slice());
        buffer.fill_buf().unwrap();
        let field_format = FieldFormat::predict(&buffer, InputFormat::Bed6).unwrap();
        assert_eq!(field_format, FieldFormat::StringBased);
    }

    #[test]
    fn field_format_string_based_bed6_b() {
        let line = b"1\t1\t2\tname\t0\t+";
        let mut buffer = BufReader::new(line.as_slice());
        buffer.fill_buf().unwrap();
        let field_format = FieldFormat::predict(&buffer, InputFormat::Bed6).unwrap();
        assert_eq!(field_format, FieldFormat::StringBased);
    }

    #[test]
    fn field_format_string_based_bed6_c() {
        let line = b"chr1\t1\t2\t1\t0\t+";
        let mut buffer = BufReader::new(line.as_slice());
        buffer.fill_buf().unwrap();
        let field_format = FieldFormat::predict(&buffer, InputFormat::Bed6).unwrap();
        assert_eq!(field_format, FieldFormat::StringBased);
    }
}
