#[cfg(test)]
mod testing {
    use anyhow::Result;
    use assert_cmd::prelude::*;
    use std::process::Command;

    #[test]
    fn test_windows() -> Result<()> {
        let a = "tests/datasets/windows/windows_a.bed";
        let b = "tests/datasets/windows/windows_b.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("window")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("-w")
            .arg("100")
            .output()?;
        let num_intervals = output.stdout.split(|&c| c == b'\n').count() - 1;
        assert_eq!(num_intervals, 18);

        let num_fields = output
            .stdout
            .split(|&c| c == b'\n')
            .next()
            .unwrap()
            .split(|&c| c == b'\t')
            .count();
        assert_eq!(num_fields, 6);
        Ok(())
    }

    #[test]
    fn test_windows_inv_bed3() -> Result<()> {
        let a = "tests/datasets/windows/windows_a.bed";
        let b = "tests/datasets/windows/windows_b.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("window")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("-w")
            .arg("100")
            .arg("-v")
            .output()?;
        let num_intervals = output.stdout.split(|&c| c == b'\n').count() - 1;
        assert_eq!(num_intervals, 3);

        let num_fields = output
            .stdout
            .split(|&c| c == b'\n')
            .next()
            .unwrap()
            .split(|&c| c == b'\t')
            .count();
        assert_eq!(num_fields, 3);
        Ok(())
    }

    #[test]
    fn test_windows_inv_bed6() -> Result<()> {
        let a = "tests/datasets/windows/windows_a.bed6";
        let b = "tests/datasets/windows/windows_b.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("window")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("-w")
            .arg("100")
            .arg("-v")
            .output()?;
        let num_intervals = output.stdout.split(|&c| c == b'\n').count() - 1;
        assert_eq!(num_intervals, 3);

        let num_fields = output
            .stdout
            .split(|&c| c == b'\n')
            .next()
            .unwrap()
            .split(|&c| c == b'\t')
            .count();
        assert_eq!(num_fields, 6);
        Ok(())
    }

    #[test]
    fn test_windows_inv_bed12() -> Result<()> {
        let a = "tests/datasets/windows/windows_a.bed12";
        let b = "tests/datasets/windows/windows_b.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("window")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("-w")
            .arg("100")
            .arg("-v")
            .output()?;
        let num_intervals = output.stdout.split(|&c| c == b'\n').count() - 1;
        assert_eq!(num_intervals, 3);

        let num_fields = output
            .stdout
            .split(|&c| c == b'\n')
            .next()
            .unwrap()
            .split(|&c| c == b'\t')
            .count();
        assert_eq!(num_fields, 12);
        Ok(())
    }

    #[test]
    fn test_windows_bed3_bed3() -> Result<()> {
        let a = "tests/datasets/windows/windows_a.bed";
        let b = "tests/datasets/windows/windows_b.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("window")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("-w")
            .arg("0")
            .output()?;
        let num_intervals = output.stdout.split(|&c| c == b'\n').count() - 1;
        assert_eq!(num_intervals, 11);

        let num_fields = output
            .stdout
            .split(|&c| c == b'\n')
            .next()
            .unwrap()
            .split(|&c| c == b'\t')
            .count();
        assert_eq!(num_fields, 6);
        Ok(())
    }

    #[test]
    fn test_windows_bed3_bed6() -> Result<()> {
        let a = "tests/datasets/windows/windows_a.bed";
        let b = "tests/datasets/windows/windows_b.bed6";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("window")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("-w")
            .arg("0")
            .output()?;
        let num_intervals = output.stdout.split(|&c| c == b'\n').count() - 1;
        assert_eq!(num_intervals, 11);

        let num_fields = output
            .stdout
            .split(|&c| c == b'\n')
            .next()
            .unwrap()
            .split(|&c| c == b'\t')
            .count();
        assert_eq!(num_fields, 9);
        Ok(())
    }

    #[test]
    fn test_windows_bed3_bed12() -> Result<()> {
        let a = "tests/datasets/windows/windows_a.bed";
        let b = "tests/datasets/windows/windows_b.bed12";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("window")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("-w")
            .arg("0")
            .output()?;
        let num_intervals = output.stdout.split(|&c| c == b'\n').count() - 1;
        assert_eq!(num_intervals, 11);

        let num_fields = output
            .stdout
            .split(|&c| c == b'\n')
            .next()
            .unwrap()
            .split(|&c| c == b'\t')
            .count();
        assert_eq!(num_fields, 15);
        Ok(())
    }
}
