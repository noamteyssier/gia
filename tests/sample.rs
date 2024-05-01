#[cfg(test)]
mod testing {
    use anyhow::Result;
    use assert_cmd::prelude::*;
    use std::process::Command;

    #[test]
    fn test_sample_int() -> Result<()> {
        let input = "tests/datasets/sample/sample.bed";
        let num_samples = 10;
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("sample")
            .arg("-i")
            .arg(input)
            .arg("-n")
            .arg(num_samples.to_string())
            .output()?;
        let num_intervals = output.stdout.split(|&c| c == b'\n').count() - 1;
        assert_eq!(num_intervals, num_samples);
        Ok(())
    }

    #[test]
    fn test_sample_int_bed6() -> Result<()> {
        let input = "tests/datasets/sample/sample.bed6";
        let num_samples = 10;
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("sample")
            .arg("-i")
            .arg(input)
            .arg("-n")
            .arg(num_samples.to_string())
            .output()?;
        let num_intervals = output.stdout.split(|&c| c == b'\n').count() - 1;
        assert_eq!(num_intervals, num_samples);
        Ok(())
    }

    #[test]
    fn test_sample_int_bed12() -> Result<()> {
        let input = "tests/datasets/sample/sample.bed12";
        let num_samples = 10;
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("sample")
            .arg("-i")
            .arg(input)
            .arg("-n")
            .arg(num_samples.to_string())
            .output()?;
        let num_intervals = output.stdout.split(|&c| c == b'\n').count() - 1;
        assert_eq!(num_intervals, num_samples);
        Ok(())
    }

    #[test]
    fn test_sample_float() -> Result<()> {
        let input = "tests/datasets/sample/sample.bed";
        let frac_samples = 0.1;
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("sample")
            .arg("-i")
            .arg(input)
            .arg("-f")
            .arg(frac_samples.to_string())
            .output()?;
        let num_intervals = output.stdout.split(|&c| c == b'\n').count() - 1;
        assert_eq!(num_intervals, 10);
        Ok(())
    }

    #[test]
    fn test_sample_seeded() -> Result<()> {
        let input = "tests/datasets/sample/sample.bed";
        let num_samples = 10;
        let seed = 42;
        let mut cmd = Command::cargo_bin("gia")?;
        let output_a = cmd
            .arg("sample")
            .arg("-i")
            .arg(input)
            .arg("-n")
            .arg(num_samples.to_string())
            .arg("-s")
            .arg(seed.to_string())
            .output()?;
        let mut cmd = Command::cargo_bin("gia")?;
        let output_b = cmd
            .arg("sample")
            .arg("-i")
            .arg(input)
            .arg("-n")
            .arg(num_samples.to_string())
            .arg("-s")
            .arg(seed.to_string())
            .output()?;
        assert_eq!(output_a.stdout, output_b.stdout);
        Ok(())
    }

    #[test]
    fn test_sample_multiple_seeds() -> Result<()> {
        let input = "tests/datasets/sample/sample.bed";
        let num_samples = 10;
        let seed_a = 42;
        let seed_b = 43;
        let mut cmd = Command::cargo_bin("gia")?;
        let output_a = cmd
            .arg("sample")
            .arg("-i")
            .arg(input)
            .arg("-n")
            .arg(num_samples.to_string())
            .arg("-s")
            .arg(seed_a.to_string())
            .output()?;
        let mut cmd = Command::cargo_bin("gia")?;
        let output_b = cmd
            .arg("sample")
            .arg("-i")
            .arg(input)
            .arg("-n")
            .arg(num_samples.to_string())
            .arg("-s")
            .arg(seed_b.to_string())
            .output()?;
        assert_ne!(output_a.stdout, output_b.stdout);
        Ok(())
    }

    #[test]
    fn test_sample_unseeded() -> Result<()> {
        let input = "tests/datasets/sample/sample.bed";
        let num_samples = 50;
        let mut cmd = Command::cargo_bin("gia")?;
        let output_a = cmd
            .arg("sample")
            .arg("-i")
            .arg(input)
            .arg("-n")
            .arg(num_samples.to_string())
            .output()?;
        let mut cmd = Command::cargo_bin("gia")?;
        let output_b = cmd
            .arg("sample")
            .arg("-i")
            .arg(input)
            .arg("-n")
            .arg(num_samples.to_string())
            .output()?;
        assert_ne!(output_a.stdout, output_b.stdout);
        Ok(())
    }
}
