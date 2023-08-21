#[cfg(test)]
mod testing {
    use anyhow::Result;
    use assert_cmd::prelude::*;
    use std::process::Command;

    fn build_expected_str(expected: &Vec<(u8, u32, u32)>) -> String {
        expected
            .iter()
            .map(|(chr, start, end)| format!("{}\t{}\t{}\n", chr, start, end))
            .collect::<Vec<String>>()
            .join("")
    }

    #[test]
    fn test_complement() -> Result<()> {
        let input = "tests/datasets/complement_a.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("complement").arg("-i").arg(input).output()?;

        let expected = vec![(1, 200, 300), (1, 400, 500), (2, 400, 600)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }
}
