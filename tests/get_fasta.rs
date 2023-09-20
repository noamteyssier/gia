#[cfg(test)]
mod testing {
    use anyhow::Result;
    use assert_cmd::prelude::*;
    use std::process::Command;

    #[test]
    fn test_get_fasta_bed3() -> Result<()> {
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
    fn test_get_fasta_named_bed3() -> Result<()> {
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

    #[test]
    fn test_get_fasta_bed6() -> Result<()> {
        let input = "tests/datasets/get_fasta/unnamed.bed6";
        let fasta = "tests/datasets/get_fasta/unnamed.fa";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("get-fasta")
            .arg("-b")
            .arg(input)
            .arg("-f")
            .arg(fasta)
            .arg("--format")
            .arg("bed6")
            .output()?;
        let expected = ">1:20-30::0::0::+\nAGCGACTACG\n>2:30-40::0::0::-\nCGATCGATCG\n";
        assert_eq!(String::from_utf8_lossy(&output.stdout), expected);
        Ok(())
    }

    #[test]
    fn test_get_fasta_named_bed6() -> Result<()> {
        let input = "tests/datasets/get_fasta/named.bed6";
        let fasta = "tests/datasets/get_fasta/named.fa";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("get-fasta")
            .arg("-b")
            .arg(input)
            .arg("-f")
            .arg(fasta)
            .arg("--format")
            .arg("bed6")
            .output()?;
        let expected = ">chr1:20-30::0::0::+\nAGCGACTACG\n>chr2:30-40::0::0::-\nCGATCGATCG\n";
        assert_eq!(String::from_utf8_lossy(&output.stdout), expected);
        Ok(())
    }
}
