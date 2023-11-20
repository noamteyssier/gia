#[cfg(test)]
mod testing {
    use anyhow::Result;
    use assert_cmd::prelude::*;
    use std::process::Command;

    #[test]
    fn test_random_defaults() -> Result<()> {
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("random").output()?;
        let num_intervals = output.stdout.iter().filter(|&&c| c == b'\n').count();

        let string_out = String::from_utf8(output.stdout)?.trim().to_string();
        let rows = string_out.split("\n");
        for row in rows {
            let cols = row.split("\t").collect::<Vec<&str>>();
            let chr = cols[0].parse::<i32>()?;
            let x = cols[1].parse::<i32>()?;
            let y = cols[2].parse::<i32>()?;

            assert!(chr <= 23);
            assert!(x <= y);
            assert!(x >= 0);
            assert!(y <= 250_000_000);
            assert_eq!(y - x, 150);
            assert_eq!(cols.len(), 3);
        }
        assert_eq!(num_intervals, 10000);
        Ok(())
    }

    #[test]
    fn test_random_params() -> Result<()> {
        let num_intervals = 10_000;
        let len_interval = 500;
        let num_chroms = 10;
        let max_chrom_len = 100_000;

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("random")
            .arg("-n")
            .arg(num_intervals.to_string())
            .arg("-l")
            .arg(len_interval.to_string())
            .arg("-c")
            .arg(num_chroms.to_string())
            .arg("-m")
            .arg(max_chrom_len.to_string())
            .output()?;
        let obs_intervals = output.stdout.iter().filter(|&&c| c == b'\n').count();

        let string_out = String::from_utf8(output.stdout)?.trim().to_string();
        let rows = string_out.split("\n");
        for row in rows {
            let cols = row.split("\t").collect::<Vec<&str>>();
            let chr = cols[0].parse::<i32>()?;
            let x = cols[1].parse::<i32>()?;
            let y = cols[2].parse::<i32>()?;

            assert!(chr <= num_chroms);
            assert!(x <= y);
            assert!(x >= 0);
            assert!(y <= max_chrom_len);
            assert_eq!(y - x, len_interval);
            assert_eq!(cols.len(), 3);
        }
        assert_eq!(obs_intervals, num_intervals);
        Ok(())
    }

    #[test]
    fn test_random_defaults_genome() -> Result<()> {
        let genome = "tests/datasets/random/example.genome";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("random").arg("-g").arg(genome).output()?;
        let num_intervals = output.stdout.iter().filter(|&&c| c == b'\n').count();

        let string_out = String::from_utf8(output.stdout)?.trim().to_string();
        let rows = string_out.split("\n");
        for row in rows {
            let cols = row.split("\t").collect::<Vec<&str>>();
            let chr = cols[0].parse::<i32>()?;
            let x = cols[1].parse::<i32>()?;
            let y = cols[2].parse::<i32>()?;

            assert!(chr <= 3);
            assert!(x <= y);
            assert!(x >= 0);
            if chr == 1 {
                assert!(y <= 1000);
            } else if chr == 2 {
                assert!(y <= 2000);
            } else if chr == 3 {
                assert!(y <= 5000);
            }
            assert_eq!(y - x, 150);
            assert_eq!(cols.len(), 3);
        }
        assert_eq!(num_intervals, 10000);
        Ok(())
    }

    #[test]
    fn test_random_seeded() -> Result<()> {
        for s in 0..5 {
            let mut cmd = Command::cargo_bin("gia")?;
            let run_a = cmd.arg("random").arg("-s").arg(format!("{s}")).output()?;
            let mut cmd = Command::cargo_bin("gia")?;
            let run_b = cmd.arg("random").arg("-s").arg(format!("{s}")).output()?;
            assert_eq!(run_a.stdout, run_b.stdout);
        }
        Ok(())
    }

    #[test]
    fn test_random_bed6() -> Result<()> {
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("random").arg("--format").arg("bed6").output()?;
        let string_out = String::from_utf8(output.stdout)?.trim().to_string();
        let rows = string_out.split("\n");
        for row in rows {
            let cols = row.split("\t").collect::<Vec<&str>>();
            assert_eq!(cols.len(), 6);
        }
        Ok(())
    }

    #[test]
    fn test_random_bed12() -> Result<()> {
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("random").arg("--format").arg("bed12").output()?;
        let string_out = String::from_utf8(output.stdout)?.trim().to_string();
        let rows = string_out.split("\n");
        for row in rows {
            let cols = row.split("\t").collect::<Vec<&str>>();
            assert_eq!(cols.len(), 12);
        }
        Ok(())
    }
}
