use clap::ValueEnum;

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
    Bed12,
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
