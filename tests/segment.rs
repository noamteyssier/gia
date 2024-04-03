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
    fn test_segment_bed3() -> Result<()> {
        let input = "tests/datasets/segment/test.bed3";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("segment").arg("-i").arg(input).output()?;
        let expected = vec![
            (1, 83, 142),
            (1, 142, 233),
            (1, 233, 292),
            (1, 349, 437),
            (1, 437, 499),
            (1, 499, 587),
            (1, 704, 854),
        ];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_segment_bed6() -> Result<()> {
        let input = "tests/datasets/segment/test.bed6";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("segment").arg("-i").arg(input).output()?;
        let expected = vec![
            (1, 83, 142, 0, ".", "-"),
            (1, 142, 233, 0, ".", "-"),
            (1, 233, 292, 0, ".", "-"),
            (1, 349, 437, 0, ".", "+"),
            (1, 437, 499, 0, ".", "+"),
            (1, 499, 587, 0, ".", "+"),
            (1, 704, 854, 0, ".", "+"),
        ];
        let expected_str = build_expected_str_bed6(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_segment_bed12() -> Result<()> {
        let input = "tests/datasets/segment/test.bed12";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("segment").arg("-i").arg(input).output()?;
        let expected = vec![
            (1, 83, 142, 0, ".", "-", 0, 0, 0, 0, 0, 0),
            (1, 142, 233, 0, ".", "-", 0, 0, 0, 0, 0, 0),
            (1, 233, 292, 0, ".", "-", 0, 0, 0, 0, 0, 0),
            (1, 349, 437, 0, ".", "+", 0, 0, 0, 0, 0, 0),
            (1, 437, 499, 0, ".", "+", 0, 0, 0, 0, 0, 0),
            (1, 499, 587, 0, ".", "+", 0, 0, 0, 0, 0, 0),
            (1, 704, 854, 0, ".", "+", 0, 0, 0, 0, 0, 0),
        ];
        let expected_str = build_expected_str_bed12(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }
}
