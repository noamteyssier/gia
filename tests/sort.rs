#[cfg(test)]
mod testing {
    use anyhow::Result;
    use assert_cmd::prelude::*;
    use bedrs::{Coordinates, GenomicInterval};
    use std::process::Command;

    #[test]
    fn test_sort() -> Result<()> {
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
}
