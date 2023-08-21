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
    fn test_subtract_merged() -> Result<()> {
        let a = "tests/datasets/subtract/subtract_a.bed";
        let b = "tests/datasets/subtract/subtract_b.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("subtract")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .output()?;

        let expected = vec![
            (1, 100, 120),
            (1, 125, 150),
            (1, 160, 300),
            (1, 400, 460),
            (1, 470, 475),
            (1, 500, 550),
        ];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_subtract_unmerged() -> Result<()> {
        let a = "tests/datasets/subtract/subtract_a.bed";
        let b = "tests/datasets/subtract/subtract_b.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("subtract")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("-u")
            .output()?;

        let expected = vec![
            (1, 100, 120),
            (1, 125, 150),
            (1, 160, 200),
            (1, 200, 300),
            (1, 400, 460),
            (1, 470, 475),
            (1, 500, 550),
        ];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_subtract_fractional_query() -> Result<()> {
        let a = "tests/datasets/subtract/subtract_a.bed";
        let b = "tests/datasets/subtract/subtract_b.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("subtract")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("-f")
            .arg("0.5")
            .arg("-u")
            .output()?;

        let expected = vec![(1, 100, 200), (1, 200, 300), (1, 400, 475), (1, 500, 550)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }
}
