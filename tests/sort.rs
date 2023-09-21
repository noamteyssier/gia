#[cfg(test)]
mod testing {
    use anyhow::Result;
    use assert_cmd::prelude::*;
    use bedrs::{Coordinates, GenomicInterval};
    use std::process::Command;

    #[test]
    fn test_sort_bed3() -> Result<()> {
        let input = "tests/datasets/sort/unsorted.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("sort").arg("-i").arg(input).output()?;
        let output_str = String::from_utf8(output.stdout)?;
        let rows = output_str
            .split("\n")
            .filter(|row| !row.is_empty())
            .collect::<Vec<&str>>();

        let mut last_interval = GenomicInterval::new(0, 0, 0);
        for row in rows {
            let fields = row
                .split("\t")
                .map(|field| field.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let interval = GenomicInterval::new(fields[0], fields[1], fields[2]);
            assert!(interval.gt(&last_interval) || interval == last_interval);
            last_interval = interval;
        }
        Ok(())
    }

    #[test]
    fn test_lex_sort_bed3() -> Result<()> {
        let input = "tests/datasets/sort/unsorted_named.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("sort").arg("-i").arg(input).arg("-N").output()?;
        let output_str = String::from_utf8(output.stdout)?;
        let rows = output_str
            .split("\n")
            .filter(|row| !row.is_empty())
            .collect::<Vec<&str>>();

        let mut last_interval = GenomicInterval::new(0, 0, 0);
        for row in rows {
            let fields = row
                .split("\t")
                .map(|field| field.replace("chr", ""))
                .map(|field| field.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let interval = GenomicInterval::new(fields[0], fields[1], fields[2]);
            assert!(interval.gt(&last_interval) || interval == last_interval);
            last_interval = interval;
        }
        Ok(())
    }

    #[test]
    fn test_sort_bed6() -> Result<()> {
        let input = "tests/datasets/sort/unsorted.bed6";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("sort")
            .arg("-i")
            .arg(input)
            .arg("--format")
            .arg("bed6")
            .output()?;
        let output_str = String::from_utf8(output.stdout)?;
        let rows = output_str
            .split("\n")
            .filter(|row| !row.is_empty())
            .collect::<Vec<&str>>();

        let mut last_interval = GenomicInterval::new(0, 0, 0);
        for row in rows {
            let fields = row.split("\t").collect::<Vec<&str>>();
            assert_eq!(fields.len(), 6);
            let numeric_fields = fields
                .iter()
                .take(3)
                .map(|field| field.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let interval =
                GenomicInterval::new(numeric_fields[0], numeric_fields[1], numeric_fields[2]);
            assert!(interval.gt(&last_interval) || interval == last_interval);
            last_interval = interval;
        }
        Ok(())
    }

    #[test]
    fn test_lex_sort_bed6() -> Result<()> {
        let input = "tests/datasets/sort/unsorted_named.bed6";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("sort")
            .arg("-i")
            .arg(input)
            .arg("--format")
            .arg("bed6")
            .output()?;
        let output_str = String::from_utf8(output.stdout)?;
        let rows = output_str
            .split("\n")
            .filter(|row| !row.is_empty())
            .collect::<Vec<&str>>();

        let mut last_interval = GenomicInterval::new(0, 0, 0);
        for row in rows {
            let fields = row.split("\t").collect::<Vec<&str>>();
            assert_eq!(fields.len(), 6);
            let numeric_fields = fields
                .iter()
                .take(3)
                .map(|field| field.replace("chr", ""))
                .map(|field| field.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let interval =
                GenomicInterval::new(numeric_fields[0], numeric_fields[1], numeric_fields[2]);
            assert!(interval.gt(&last_interval) || interval == last_interval);
            last_interval = interval;
        }
        Ok(())
    }

    #[test]
    fn test_lex_sort_bed6_correct_name() -> Result<()> {
        let input = "tests/datasets/sort/unsorted_named.bed6";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("sort")
            .arg("-i")
            .arg(input)
            .arg("--format")
            .arg("bed6")
            .output()?;
        let output_str = String::from_utf8(output.stdout)?;
        let rows = output_str
            .split("\n")
            .filter(|row| !row.is_empty())
            .collect::<Vec<&str>>();

        let mut last_interval = GenomicInterval::new(0, 0, 0);
        for row in rows {
            let fields = row.split("\t").collect::<Vec<&str>>();
            assert_eq!(fields.len(), 6);
            let numeric_fields = fields
                .iter()
                .take(3)
                .map(|field| field.replace("chr", ""))
                .map(|field| field.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let interval =
                GenomicInterval::new(numeric_fields[0], numeric_fields[1], numeric_fields[2]);
            assert!(interval.gt(&last_interval) || interval == last_interval);
            last_interval = interval;
            assert_eq!(fields[3], "0");
            assert_eq!(fields[4], "0.0");
            assert_eq!(fields[5], "+");
        }
        Ok(())
    }
}
