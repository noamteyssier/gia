#[cfg(test)]
mod testing {
    use anyhow::Result;
    use assert_cmd::prelude::*;
    use std::{fmt::Display, process::Command};

    fn count_lines(output: &[u8]) -> usize {
        output.split(|&c| c == b'\n').count() - 1
    }

    fn count_fields(output: &[u8]) -> usize {
        output
            .split(|&c| c == b'\n')
            .next()
            .unwrap()
            .split(|&c| c == b'\t')
            .count()
    }

    fn build_expected_str<T: Display>(expected: &[(T, u32, u32, T)]) -> String {
        expected
            .iter()
            .map(|(chr, start, end, depth)| format!("{}\t{}\t{}\t{}\n", chr, start, end, depth))
            .collect::<Vec<String>>()
            .join("")
    }

    #[test]
    fn test_spacing_bed3() -> Result<()> {
        let filename = "tests/datasets/spacing/test.bed3";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("spacing").arg("-i").arg(filename).output()?;

        let num_intervals = count_lines(&output.stdout);
        assert_eq!(num_intervals, 6);

        let num_fields = count_fields(&output.stdout);
        assert_eq!(num_fields, 4);

        let expected = vec![
            ("chr1", 0, 10, "."),
            ("chr1", 10, 20, "0"),
            ("chr1", 19, 30, "-1"),
            ("chr1", 35, 45, "5"),
            ("chr1", 100, 200, "55"),
            ("chr2", 100, 200, "."),
        ];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());

        Ok(())
    }

    #[test]
    fn test_spacing_bed4() -> Result<()> {
        let filename = "tests/datasets/spacing/test.bed4";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("spacing").arg("-i").arg(filename).output()?;

        let num_intervals = count_lines(&output.stdout);
        assert_eq!(num_intervals, 6);

        let num_fields = count_fields(&output.stdout);
        assert_eq!(num_fields, 5);
        Ok(())
    }

    #[test]
    fn test_spacing_bed6() -> Result<()> {
        let filename = "tests/datasets/spacing/test.bed6";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("spacing").arg("-i").arg(filename).output()?;

        let num_intervals = count_lines(&output.stdout);
        assert_eq!(num_intervals, 6);

        let num_fields = count_fields(&output.stdout);
        assert_eq!(num_fields, 7);
        Ok(())
    }

    #[test]
    fn test_spacing_bed12() -> Result<()> {
        let filename = "tests/datasets/spacing/test.bed12";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("spacing").arg("-i").arg(filename).output()?;

        let num_intervals = count_lines(&output.stdout);
        assert_eq!(num_intervals, 6);

        let num_fields = count_fields(&output.stdout);
        assert_eq!(num_fields, 13);
        Ok(())
    }
}
