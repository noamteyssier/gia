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

    fn calculate_n_fields(output: &[u8]) -> usize {
        output
            .split(|&c| c == b'\n')
            .next()
            .unwrap()
            .split(|&c| c == b'\t')
            .count()
    }

    #[test]
    fn test_cluster_bed3() -> Result<()> {
        let input = "tests/datasets/cluster/test.bed3";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("cluster").arg("-i").arg(input).output()?;
        let expected = vec![
            (1, 83, 233, 0),
            (1, 142, 292, 0),
            (1, 349, 499, 1),
            (1, 437, 587, 1),
            (1, 704, 854, 2),
        ];
        let expected_str = build_expected_str(&expected);
        let observed_str = String::from_utf8(output.stdout)?;
        assert_eq!(observed_str, expected_str);
        Ok(())
    }

    #[test]
    fn test_cluster_bed6() -> Result<()> {
        let input = "tests/datasets/cluster/test.bed6";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("cluster").arg("-i").arg(input).output()?;
        let n_fields = calculate_n_fields(&output.stdout);
        assert_eq!(n_fields, 7);
        Ok(())
    }

    #[test]
    fn test_cluster_bed12() -> Result<()> {
        let input = "tests/datasets/cluster/test.bed12";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("cluster").arg("-i").arg(input).output()?;
        let n_fields = calculate_n_fields(&output.stdout);
        assert_eq!(n_fields, 13);
        Ok(())
    }
}
