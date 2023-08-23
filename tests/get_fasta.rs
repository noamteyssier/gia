#[cfg(test)]
mod testing {
    use anyhow::Result;
    use assert_cmd::prelude::*;
    use std::process::Command;

    #[test]
    fn test_get_fasta() -> Result<()> {
        let input = "tests/datasets/get_fasta/unnamed.bed";
        let fasta = "tests/datasets/get_fasta/unnamed.fa";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("get-fasta")
            .arg("-b")
            .arg(input)
            .arg("-f")
            .arg(fasta)
            .output()?;
        let expected = ">1:20-30\nAGCGACTACG\n>2:30-40\nCGATCGATCG\n";
        assert_eq!(String::from_utf8_lossy(&output.stdout), expected);
        Ok(())
    }

    #[test]
    fn test_get_fasta_named() -> Result<()> {
        let input = "tests/datasets/get_fasta/named.bed";
        let fasta = "tests/datasets/get_fasta/named.fa";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("get-fasta")
            .arg("-b")
            .arg(input)
            .arg("-f")
            .arg(fasta)
            .output()?;
        let expected = ">chr1:20-30\nAGCGACTACG\n>chr2:30-40\nCGATCGATCG\n";
        assert_eq!(String::from_utf8_lossy(&output.stdout), expected);
        Ok(())
    }
}
