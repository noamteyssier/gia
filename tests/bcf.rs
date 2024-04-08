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
            .skip_while(|line| line.starts_with(b"#"))
            .next()
            .unwrap()
            .split(|&c| c == b'\t')
            .count()
    }

    fn get_num_header_lines(output: &[u8]) -> usize {
        output
            .split(|&c| c == b'\n')
            .take_while(|line| line.starts_with(b"#"))
            .count()
    }

    #[test]
    fn test_bcf_filter() -> Result<()> {
        let a_set = "tests/datasets/bcf/chr22.bcf";
        let b_set = "tests/datasets/bcf/filter.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("bcf")
            .arg("filter")
            .arg("-a")
            .arg(a_set)
            .arg("-b")
            .arg(b_set)
            .arg("-O")
            .arg("v") // show VCF output for easier time counting lines
            .output()?;
        assert!(output.status.success());
        assert_eq!(output.stderr, b"");
        let num_lines = get_num_lines(&output.stdout);
        let num_header_lines = get_num_header_lines(&output.stdout);
        let num_cols = get_num_cols(&output.stdout);
        assert_eq!(num_lines, 1037);
        assert_eq!(num_header_lines, 32);
        assert_eq!(num_cols, 14);
        Ok(())
    }

    #[test]
    fn test_bcf_filter_invert() -> Result<()> {
        let a_set = "tests/datasets/bcf/chr22.bcf";
        let b_set = "tests/datasets/bcf/filter.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("bcf")
            .arg("filter")
            .arg("-a")
            .arg(a_set)
            .arg("-b")
            .arg(b_set)
            .arg("-O")
            .arg("v") // show VCF output for easier time counting lines
            .arg("--invert")
            .output()?;
        assert!(output.status.success());
        assert_eq!(output.stderr, b"");
        let num_lines = get_num_lines(&output.stdout);
        let num_header_lines = get_num_header_lines(&output.stdout);
        let num_cols = get_num_cols(&output.stdout);
        assert_eq!(num_lines, 9405);
        assert_eq!(num_header_lines, 32);
        assert_eq!(num_cols, 14);
        Ok(())
    }

    #[test]
    fn test_bcf_filter_types() -> Result<()> {
        let inputs = vec![
            "tests/datasets/bcf/chr22.vcf",
            "tests/datasets/bcf/chr22.vcf.gz",
        ];
        let b_set = "tests/datasets/bcf/filter.bed";
        for a_set in inputs {
            let mut cmd = Command::cargo_bin("gia")?;
            let output = cmd
                .arg("bcf")
                .arg("filter")
                .arg("-a")
                .arg(a_set)
                .arg("-b")
                .arg(b_set)
                .arg("-O")
                .arg("v") // show VCF output for easier time counting lines
                .output()?;
            assert!(output.status.success());
            assert_eq!(output.stderr, b"");
            let num_lines = get_num_lines(&output.stdout);
            let num_header_lines = get_num_header_lines(&output.stdout);
            let num_cols = get_num_cols(&output.stdout);
            assert_eq!(num_header_lines, 33); // one extra header line for the VCF version
            assert_eq!(num_cols, 14);
            assert_eq!(num_lines, 1038);
        }
        Ok(())
    }

    #[test]
    fn test_bcf_filter_invert_types() -> Result<()> {
        let inputs = vec![
            "tests/datasets/bcf/chr22.vcf",
            "tests/datasets/bcf/chr22.vcf.gz",
        ];
        let b_set = "tests/datasets/bcf/filter.bed";
        for a_set in inputs {
            let mut cmd = Command::cargo_bin("gia")?;
            let output = cmd
                .arg("bcf")
                .arg("filter")
                .arg("-a")
                .arg(a_set)
                .arg("-b")
                .arg(b_set)
                .arg("-O")
                .arg("v") // show VCF output for easier time counting lines
                .arg("--invert")
                .output()?;
            assert!(output.status.success());
            assert_eq!(output.stderr, b"");
            let num_lines = get_num_lines(&output.stdout);
            let num_header_lines = get_num_header_lines(&output.stdout);
            let num_cols = get_num_cols(&output.stdout);
            assert_eq!(num_header_lines, 33); // one extra header line for the VCF version
            assert_eq!(num_cols, 14);
            assert_eq!(num_lines, 9406);
        }
        Ok(())
    }
}
