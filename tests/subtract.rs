#[cfg(test)]
mod testing {
    use anyhow::Result;
    use assert_cmd::prelude::*;
    use std::{fmt::Display, process::Command};

    type Expected3<T> = Vec<(T, u32, u32)>;
    type Expected6<T, S, F, C> = Vec<(S, T, T, S, F, C)>;
    type Expected12<T, S, F, C> = Vec<(S, T, T, S, F, C, T, T, T, T, T, T)>;

    fn build_expected_str<T: Display>(expected: &Expected3<T>) -> String {
        expected
            .iter()
            .map(|(chr, start, end)| format!("{}\t{}\t{}\n", chr, start, end))
            .collect::<Vec<String>>()
            .join("")
    }

    fn build_expected_str_bed6<T: Display, S: Display, F: Display, C: Display>(
        expected: &Expected6<T, S, F, C>,
    ) -> String {
        expected
            .iter()
            .map(|(chr, start, end, name, score, strand)| {
                format!(
                    "{}\t{}\t{}\t{}\t{:.3}\t{}\n",
                    chr, start, end, name, score, strand
                )
            })
            .collect::<Vec<String>>()
            .join("")
    }

    fn build_expected_str_bed12<T: Display, S: Display, F: Display, C: Display>(
        expected: &Expected12<T, S, F, C>,
    ) -> String {
        expected
            .iter()
            .map(
                |(
                    chr,
                    start,
                    end,
                    name,
                    score,
                    strand,
                    thick_start,
                    thick_end,
                    rgb,
                    block_count,
                    block_sizes,
                    block_starts,
                )| {
                    format!(
                        "{}\t{}\t{}\t{}\t{:.3}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
                        chr,
                        start,
                        end,
                        name,
                        score,
                        strand,
                        thick_start,
                        thick_end,
                        rgb,
                        block_count,
                        block_sizes,
                        block_starts
                    )
                },
            )
            .collect::<Vec<String>>()
            .join("")
    }

    #[test]
    fn test_subtract_merged_bed3() -> Result<()> {
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
    fn test_subtract_merged_bed6() -> Result<()> {
        let a = "tests/datasets/subtract/subtract_a.bed6";
        let b = "tests/datasets/subtract/subtract_b.bed6";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("subtract")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .output()?;

        let expected = vec![
            (1, 100, 120, 0, '.', '+'),
            (1, 125, 150, 0, '.', '+'),
            (1, 160, 300, 0, '.', '+'),
            (1, 400, 460, 0, '.', '+'),
            (1, 470, 475, 0, '.', '+'),
            (1, 500, 550, 0, '.', '+'),
        ];
        let expected_str = build_expected_str_bed6(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_subtract_merged_bed12() -> Result<()> {
        let a = "tests/datasets/subtract/subtract_a.bed12";
        let b = "tests/datasets/subtract/subtract_b.bed12";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("subtract")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .output()?;

        let expected = vec![
            (1, 100, 120, 0, ".", '+', 0, 0, 0, 0, 0, 0),
            (1, 125, 150, 0, ".", '+', 0, 0, 0, 0, 0, 0),
            (1, 160, 300, 0, ".", '+', 0, 0, 0, 0, 0, 0),
            (1, 400, 460, 0, ".", '+', 0, 0, 0, 0, 0, 0),
            (1, 470, 475, 0, ".", '+', 0, 0, 0, 0, 0, 0),
            (1, 500, 550, 0, ".", '+', 0, 0, 0, 0, 0, 0),
        ];
        let expected_str = build_expected_str_bed12(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_subtract_unmerged_bed3() -> Result<()> {
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
    fn test_subtract_unmerged_bed6() -> Result<()> {
        let a = "tests/datasets/subtract/subtract_a.bed6";
        let b = "tests/datasets/subtract/subtract_b.bed6";

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
            (1, 100, 120, 0, ".", '+'),
            (1, 125, 150, 0, ".", '+'),
            (1, 160, 200, 0, ".", '+'),
            (1, 200, 300, 0, "0", '+'),
            (1, 400, 460, 0, ".", '+'),
            (1, 470, 475, 0, ".", '+'),
            (1, 500, 550, 0, "0", '+'),
        ];
        let expected_str = build_expected_str_bed6(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_subtract_unmerged_bed12() -> Result<()> {
        let a = "tests/datasets/subtract/subtract_a.bed12";
        let b = "tests/datasets/subtract/subtract_b.bed12";

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
            (1, 100, 120, 0, ".", '+', 0, 0, 0, 0, 0, 0),
            (1, 125, 150, 0, ".", '+', 0, 0, 0, 0, 0, 0),
            (1, 160, 200, 0, ".", '+', 0, 0, 0, 0, 0, 0),
            (1, 200, 300, 0, "0", '+', 0, 0, 0, 0, 0, 0),
            (1, 400, 460, 0, ".", '+', 0, 0, 0, 0, 0, 0),
            (1, 470, 475, 0, ".", '+', 0, 0, 0, 0, 0, 0),
            (1, 500, 550, 0, "0", '+', 0, 0, 0, 0, 0, 0),
        ];
        let expected_str = build_expected_str_bed12(&expected);

        println!("{}", std::str::from_utf8(&output.stdout).unwrap());
        println!("{}", expected_str);

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
