#[cfg(test)]
mod testing {
    use anyhow::Result;
    use assert_cmd::prelude::*;
    use std::process::Command;

    fn get_num_lines(output: &[u8]) -> usize {
        output.split(|&c| c == b'\n').count()
    }

    fn get_num_cols(output: &[u8]) -> usize {
        output
            .split(|&c| c == b'\n')
            .next()
            .unwrap()
            .split(|&c| c == b'\t')
            .count()
    }

    fn get_num_header_lines(output: &[u8]) -> usize {
        output
            .split(|&c| c == b'\n')
            .take_while(|line| line.starts_with(b"@"))
            .count()
    }

    #[test]
    fn test_bam_convert() -> Result<()> {
        let input = "tests/datasets/bam/tiny.bam";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("bam")
            .arg("convert")
            .arg("-i")
            .arg(input)
            .output()?;
        assert!(output.status.success());
        assert_eq!(output.stderr, b"");
        let num_lines = get_num_lines(&output.stdout);
        let num_cols = get_num_cols(&output.stdout);
        assert_eq!(num_lines, 99);
        assert_eq!(num_cols, 6);
        Ok(())
    }

    #[test]
    fn test_bam_convert_cigar() -> Result<()> {
        let input = "tests/datasets/bam/tiny.bam";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("bam")
            .arg("convert")
            .arg("-i")
            .arg(input)
            .arg("--cigar")
            .output()?;
        assert!(output.status.success());
        assert_eq!(output.stderr, b"");
        let num_lines = get_num_lines(&output.stdout);
        let num_cols = get_num_cols(&output.stdout);
        assert_eq!(num_lines, 99);
        assert_eq!(num_cols, 7);
        Ok(())
    }

    #[test]
    fn test_bam_filter() -> Result<()> {
        let a_set = "tests/datasets/bam/tiny.bam";
        let b_set = "tests/datasets/bam/filter.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("bam")
            .arg("filter")
            .arg("-a")
            .arg(a_set)
            .arg("-b")
            .arg(b_set)
            .arg("-O")
            .arg("sam") // show SAM output for easier time counting lines
            .output()?;
        assert!(output.status.success());
        assert_eq!(output.stderr, b"");
        let num_lines = get_num_lines(&output.stdout);
        let num_header_lines = get_num_header_lines(&output.stdout);
        assert_eq!(num_lines, 215);
        assert_eq!(num_header_lines, 200);
        Ok(())
    }

    #[test]
    fn test_bam_filter_invert() -> Result<()> {
        let a_set = "tests/datasets/bam/tiny.bam";
        let b_set = "tests/datasets/bam/filter.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("bam")
            .arg("filter")
            .arg("-a")
            .arg(a_set)
            .arg("-b")
            .arg(b_set)
            .arg("-O")
            .arg("sam") // show SAM output for easier time counting lines
            .arg("-v")
            .output()?;
        assert!(output.status.success());
        assert_eq!(output.stderr, b"");
        let num_lines = get_num_lines(&output.stdout);
        let num_header_lines = get_num_header_lines(&output.stdout);
        assert_eq!(num_lines, 285);
        assert_eq!(num_header_lines, 200);
        Ok(())
    }
}
