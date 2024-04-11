#[cfg(test)]
mod testing {
    use anyhow::Result;
    use assert_cmd::prelude::*;
    use std::{fmt::Display, process::Command};

    type Expected<T> = Vec<(T, u32, u32)>;

    fn build_expected_str<T: Display>(expected: &Expected<T>) -> String {
        expected
            .iter()
            .map(|(chr, start, end)| format!("{}\t{}\t{}\n", chr, start, end))
            .collect::<Vec<String>>()
            .join("")
    }

    #[test]
    fn test_merge_sorted_bed3() -> Result<()> {
        let input = "tests/datasets/merge/sorted.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("merge")
            .arg("-i")
            .arg(input)
            .arg("--demote")
            .output()?;

        let expected = vec![(1, 10, 45), (1, 100, 300), (2, 105, 301)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_merge_sorted_bed6() -> Result<()> {
        let input = "tests/datasets/merge/sorted.bed6";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("merge")
            .arg("-i")
            .arg(input)
            .arg("--demote")
            .output()?;

        let expected = vec![(1, 10, 45), (1, 100, 300), (2, 105, 301)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_merge_sorted_bed12() -> Result<()> {
        let input = "tests/datasets/merge/sorted.bed12";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("merge")
            .arg("-i")
            .arg(input)
            .arg("--demote")
            .output()?;

        let expected = vec![(1, 10, 45), (1, 100, 300), (2, 105, 301)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_merge_sorted_skip_sort() -> Result<()> {
        let input = "tests/datasets/merge/sorted.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("merge")
            .arg("-s")
            .arg("-i")
            .arg(input)
            .arg("--demote")
            .output()?;

        let expected = vec![(1, 10, 45), (1, 100, 300), (2, 105, 301)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_merge_unsorted_bed3() -> Result<()> {
        let input = "tests/datasets/merge/unsorted.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("merge")
            .arg("-i")
            .arg(input)
            .arg("--demote")
            .output()?;

        let expected = vec![(1, 10, 45), (1, 100, 300), (2, 105, 301)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_merge_unsorted_bed6() -> Result<()> {
        let input = "tests/datasets/merge/unsorted.bed6";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("merge")
            .arg("-i")
            .arg(input)
            .arg("--demote")
            .output()?;

        let expected = vec![(1, 10, 45), (1, 100, 300), (2, 105, 301)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_merge_unsorted_bed12() -> Result<()> {
        let input = "tests/datasets/merge/unsorted.bed12";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("merge")
            .arg("-i")
            .arg(input)
            .arg("--demote")
            .output()?;

        let expected = vec![(1, 10, 45), (1, 100, 300), (2, 105, 301)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_merge_unsorted_named_bed3() -> Result<()> {
        let input = "tests/datasets/merge/unsorted_named.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("merge")
            .arg("-i")
            .arg(input)
            .arg("--demote")
            .output()?;

        let expected = vec![("chr1", 10, 45), ("chr1", 100, 300), ("chr2", 105, 301)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_merge_unsorted_named_bed6() -> Result<()> {
        let input = "tests/datasets/merge/unsorted_named.bed6";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("merge")
            .arg("-i")
            .arg(input)
            .arg("--demote")
            .output()?;

        let expected = vec![("chr1", 10, 45), ("chr1", 100, 300), ("chr2", 105, 301)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_merge_unsorted_named_bed12() -> Result<()> {
        let input = "tests/datasets/merge/unsorted_named.bed12";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("merge")
            .arg("-i")
            .arg(input)
            .arg("--demote")
            .output()?;

        let expected = vec![("chr1", 10, 45), ("chr1", 100, 300), ("chr2", 105, 301)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_merge_sorted_named_bed3() -> Result<()> {
        let input = "tests/datasets/merge/sorted_named.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("merge")
            .arg("-i")
            .arg(input)
            .arg("--demote")
            .output()?;

        let expected = vec![("chr1", 10, 45), ("chr1", 100, 300), ("chr2", 105, 301)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_merge_sorted_named_bed6() -> Result<()> {
        let input = "tests/datasets/merge/sorted_named.bed6";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("merge")
            .arg("-i")
            .arg(input)
            .arg("--demote")
            .output()?;

        let expected = vec![("chr1", 10, 45), ("chr1", 100, 300), ("chr2", 105, 301)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_merge_sorted_named_bed12() -> Result<()> {
        let input = "tests/datasets/merge/sorted_named.bed12";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("merge")
            .arg("-i")
            .arg(input)
            .arg("--demote")
            .output()?;

        let expected = vec![("chr1", 10, 45), ("chr1", 100, 300), ("chr2", 105, 301)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_merge_stream() -> Result<()> {
        let input = "tests/datasets/merge/sorted.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("merge")
            .arg("-S")
            .arg("-i")
            .arg(input)
            .arg("--demote")
            .output()?;

        let expected = vec![(1, 10, 45), (1, 100, 300), (2, 105, 301)];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }
}
