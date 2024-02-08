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
    fn test_extend_left() -> Result<()> {
        let input = "tests/datasets/extend/extend.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("extend")
            .arg("-i")
            .arg(input)
            .arg("-l")
            .arg("20")
            .output()?;
        let expected = vec![(1, 0, 20), (1, 10, 40), (2, 10, 40)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_extend_right() -> Result<()> {
        let input = "tests/datasets/extend/extend.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("extend")
            .arg("-i")
            .arg(input)
            .arg("-r")
            .arg("20")
            .output()?;
        let expected = vec![(1, 10, 40), (1, 30, 60), (2, 30, 60)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_extend_right_genome() -> Result<()> {
        let input = "tests/datasets/extend/extend.bed";
        let genome = "tests/datasets/extend/extend.genome";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("extend")
            .arg("-i")
            .arg(input)
            .arg("-r")
            .arg("20")
            .arg("-g")
            .arg(genome)
            .output()?;
        let expected = vec![(1, 10, 40), (1, 30, 60), (2, 30, 50)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_extend_unequal_both() -> Result<()> {
        let input = "tests/datasets/extend/extend.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("extend")
            .arg("-i")
            .arg(input)
            .arg("-l")
            .arg("20")
            .arg("-r")
            .arg("5")
            .output()?;
        let expected = vec![(1, 0, 25), (1, 10, 45), (2, 10, 45)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_extend_unequal_both_genome() -> Result<()> {
        let input = "tests/datasets/extend/extend.bed";
        let genome = "tests/datasets/extend/extend.genome";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("extend")
            .arg("-i")
            .arg(input)
            .arg("-l")
            .arg("20")
            .arg("-r")
            .arg("25")
            .arg("-g")
            .arg(genome)
            .output()?;
        let expected = vec![(1, 0, 45), (1, 10, 65), (2, 10, 50)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_extend_both() -> Result<()> {
        let input = "tests/datasets/extend/extend.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("extend")
            .arg("-i")
            .arg(input)
            .arg("-t")
            .arg("20")
            .output()?;
        let expected = vec![(1, 0, 40), (1, 10, 60), (2, 10, 60)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_extend_both_genome() -> Result<()> {
        let input = "tests/datasets/extend/extend.bed";
        let genome = "tests/datasets/extend/extend.genome";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("extend")
            .arg("-i")
            .arg(input)
            .arg("-t")
            .arg("20")
            .arg("-g")
            .arg(genome)
            .output()?;
        let expected = vec![(1, 0, 40), (1, 10, 60), (2, 10, 50)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }
}
