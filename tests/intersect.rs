#[cfg(test)]
mod testing {
    use anyhow::Result;
    use assert_cmd::prelude::*;
    use std::{fmt::Display, process::Command};

    fn build_expected_str<T: Display>(expected: &Vec<(T, u32, u32)>) -> String {
        expected
            .iter()
            .map(|(chr, start, end)| format!("{}\t{}\t{}\n", chr, start, end))
            .collect::<Vec<String>>()
            .join("")
    }

    #[test]
    fn test_intersect_bed3() -> Result<()> {
        let a = "tests/datasets/intersect/intersect_a.bed";
        let b = "tests/datasets/intersect/intersect_b.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("intersect")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .output()?;

        let num_intervals = output.stdout.split(|&c| c == b'\n').count() - 1;
        assert_eq!(num_intervals, 11);

        let num_fields = output
            .stdout
            .split(|&c| c == b'\n')
            .next()
            .unwrap()
            .split(|&c| c == b'\t')
            .count();
        assert_eq!(num_fields, 3);
        Ok(())
    }

    #[test]
    fn test_intersect_bed6() -> Result<()> {
        let a = "tests/datasets/intersect/intersect_a.bed6";
        let b = "tests/datasets/intersect/intersect_b.bed6";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("intersect")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("--format")
            .arg("bed6")
            .output()?;

        let num_intervals = output.stdout.split(|&c| c == b'\n').count() - 1;
        assert_eq!(num_intervals, 11);

        let num_fields = output
            .stdout
            .split(|&c| c == b'\n')
            .next()
            .unwrap()
            .split(|&c| c == b'\t')
            .count();
        assert_eq!(num_fields, 6);
        Ok(())
    }

    #[test]
    fn test_intersect_bed12() -> Result<()> {
        let a = "tests/datasets/intersect/intersect_a.bed12";
        let b = "tests/datasets/intersect/intersect_b.bed12";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("intersect")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("--format")
            .arg("bed12")
            .output()?;

        let num_intervals = output.stdout.split(|&c| c == b'\n').count() - 1;
        assert_eq!(num_intervals, 11);

        let num_fields = output
            .stdout
            .split(|&c| c == b'\n')
            .next()
            .unwrap()
            .split(|&c| c == b'\t')
            .count();
        assert_eq!(num_fields, 12);
        Ok(())
    }

    #[test]
    fn test_intersect_fractional_query() -> Result<()> {
        let a = "tests/datasets/intersect/intersect_a.bed";
        let b = "tests/datasets/intersect/intersect_b.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("intersect")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("-f")
            .arg("0.5")
            .output()?;

        let num_intervals = output.stdout.split(|&c| c == b'\n').count() - 1;
        assert_eq!(num_intervals, 4);
        Ok(())
    }

    #[test]
    fn test_intersect_fractional_target() -> Result<()> {
        let a = "tests/datasets/intersect/intersect_a.bed";
        let b = "tests/datasets/intersect/intersect_b.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("intersect")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("-F")
            .arg("0.5")
            .output()?;

        let num_intervals = output.stdout.split(|&c| c == b'\n').count() - 1;
        assert_eq!(num_intervals, 4);
        Ok(())
    }

    #[test]
    fn test_intersect_reciprocal() -> Result<()> {
        let a = "tests/datasets/intersect/intersect_a.bed";
        let b = "tests/datasets/intersect/intersect_b.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("intersect")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("-f")
            .arg("0.5")
            .arg("-r")
            .output()?;

        let num_intervals = output.stdout.split(|&c| c == b'\n').count() - 1;
        assert_eq!(num_intervals, 4);
        Ok(())
    }

    #[test]
    fn test_intersect_with_query() -> Result<()> {
        let a = "tests/datasets/intersect/intersect_a.bed";
        let b = "tests/datasets/intersect/intersect_b.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("intersect")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("-q")
            .output()?;

        let expected = vec![
            (1, 72, 222),
            (1, 72, 222),
            (1, 72, 222),
            (1, 72, 222),
            (1, 257, 407),
            (1, 268, 418),
            (1, 467, 617),
            (1, 819, 969),
            (2, 174, 324),
            (2, 174, 324),
            (2, 587, 737),
        ];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_intersect_with_query_unique() -> Result<()> {
        let a = "tests/datasets/intersect/intersect_a.bed";
        let b = "tests/datasets/intersect/intersect_b.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("intersect")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("-q")
            .arg("-u")
            .output()?;

        let expected = vec![
            (1, 72, 222),
            (1, 257, 407),
            (1, 268, 418),
            (1, 467, 617),
            (1, 819, 969),
            (2, 174, 324),
            (2, 587, 737),
        ];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_intersect_with_target() -> Result<()> {
        let a = "tests/datasets/intersect/intersect_a.bed";
        let b = "tests/datasets/intersect/intersect_b.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("intersect")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("-t")
            .output()?;

        let expected = vec![
            (1, 55, 205),
            (1, 69, 219),
            (1, 93, 243),
            (1, 156, 306),
            (1, 156, 306),
            (1, 156, 306),
            (1, 603, 753),
            (1, 837, 987),
            (2, 39, 189),
            (2, 71, 221),
            (2, 672, 822),
        ];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_intersect_inverse() -> Result<()> {
        let a = "tests/datasets/intersect/intersect_a.bed";
        let b = "tests/datasets/intersect/intersect_b.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("intersect")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("-v")
            .output()?;

        let expected = vec![(3, 395, 545), (3, 554, 704), (3, 653, 803)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_intersect_streamed() -> Result<()> {
        let a = "tests/datasets/intersect/intersect_a.bed";
        let b = "tests/datasets/intersect/intersect_b.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("intersect")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("-S")
            .output()?;
        let expected = vec![
            (1, 72, 222),
            (1, 257, 306),
            (1, 603, 617),
            (1, 837, 969),
            (2, 174, 221),
            (2, 672, 737),
        ];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }
}
