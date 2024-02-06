#[cfg(test)]
mod testing {
    use anyhow::Result;
    use assert_cmd::prelude::*;
    use std::{fmt::Display, process::Command};

    fn build_expected_str<T: Display>(expected: &[(T, u32, u32, u32)]) -> String {
        expected
            .iter()
            .map(|(chr, start, end, depth)| format!("{}\t{}\t{}\t{}\n", chr, start, end, depth))
            .collect::<Vec<String>>()
            .join("")
    }

    #[test]
    fn test_coverage() -> Result<()> {
        let a = "tests/datasets/coverage/coverage_a.bed";
        let b = "tests/datasets/coverage/coverage_b.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("coverage")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .output()?;
        println!("{:?}", output);
        let num_intervals = output.stdout.split(|&c| c == b'\n').count() - 1;
        assert_eq!(num_intervals, 10);

        let num_fields = output
            .stdout
            .split(|&c| c == b'\n')
            .next()
            .unwrap()
            .split(|&c| c == b'\t')
            .count();
        assert_eq!(num_fields, 4);

        let expected = vec![
            (1, 72, 222, 4),
            (1, 257, 407, 1),
            (1, 268, 418, 1),
            (1, 467, 617, 1),
            (1, 819, 969, 1),
            (2, 174, 324, 2),
            (2, 587, 737, 1),
            (3, 395, 545, 0),
            (3, 554, 704, 0),
            (3, 653, 803, 0),
        ];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_coverage_bed3_bed3() -> Result<()> {
        let a = "tests/datasets/coverage/coverage_a.bed";
        let b = "tests/datasets/coverage/coverage_b.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("coverage")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .output()?;
        println!("{:?}", output);
        let num_intervals = output.stdout.split(|&c| c == b'\n').count() - 1;
        assert_eq!(num_intervals, 10);

        let num_fields = output
            .stdout
            .split(|&c| c == b'\n')
            .next()
            .unwrap()
            .split(|&c| c == b'\t')
            .count();
        assert_eq!(num_fields, 4);
        Ok(())
    }

    #[test]
    fn test_coverage_bed6_bed3() -> Result<()> {
        let a = "tests/datasets/coverage/coverage_a.bed6";
        let b = "tests/datasets/coverage/coverage_b.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("coverage")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .output()?;
        println!("{:?}", output);
        let num_intervals = output.stdout.split(|&c| c == b'\n').count() - 1;
        assert_eq!(num_intervals, 10);

        let num_fields = output
            .stdout
            .split(|&c| c == b'\n')
            .next()
            .unwrap()
            .split(|&c| c == b'\t')
            .count();
        assert_eq!(num_fields, 7);
        Ok(())
    }

    #[test]
    fn test_coverage_bed12_bed3() -> Result<()> {
        let a = "tests/datasets/coverage/coverage_a.bed12";
        let b = "tests/datasets/coverage/coverage_b.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("coverage")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .output()?;
        println!("{:?}", output);
        let num_intervals = output.stdout.split(|&c| c == b'\n').count() - 1;
        assert_eq!(num_intervals, 10);

        let num_fields = output
            .stdout
            .split(|&c| c == b'\n')
            .next()
            .unwrap()
            .split(|&c| c == b'\t')
            .count();
        assert_eq!(num_fields, 13);
        Ok(())
    }
}
