#[cfg(test)]
mod testing {
    use anyhow::Result;
    use assert_cmd::prelude::*;
    use std::process::Command;

    fn get_n_intervals(output: &[u8]) -> usize {
        output.split(|c| *c == b'\n').count() - 1
    }

    fn get_n_fields(output: &[u8]) -> usize {
        output
            .split(|c| *c == b'\n')
            .next()
            .unwrap()
            .split(|c| *c == b'\t')
            .count()
    }

    fn validate_output(output: &[u8], expected: Vec<Vec<&str>>) {
        let output: Vec<Vec<&str>> = output
            .split(|c| *c == b'\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.split(|c| *c == b'\t')
                    .map(|field| std::str::from_utf8(field).unwrap())
                    .collect()
            })
            .collect();
        assert_eq!(output, expected);
    }

    #[test]
    fn test_inner_join_bed3() -> Result<()> {
        let a = "tests/datasets/join/a.bed";
        let b = "tests/datasets/join/b.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("join")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("--how")
            .arg("inner")
            .output()?;

        let num_intervals = get_n_intervals(&output.stdout);
        assert_eq!(num_intervals, 11);

        let num_fields = get_n_fields(&output.stdout);
        assert_eq!(num_fields, 6);

        let expected = vec![
            vec!["1", "72", "222", "1", "55", "205"],
            vec!["1", "72", "222", "1", "69", "219"],
            vec!["1", "72", "222", "1", "93", "243"],
            vec!["1", "72", "222", "1", "156", "306"],
            vec!["1", "257", "407", "1", "156", "306"],
            vec!["1", "268", "418", "1", "156", "306"],
            vec!["1", "467", "617", "1", "603", "753"],
            vec!["1", "819", "969", "1", "837", "987"],
            vec!["2", "174", "324", "2", "39", "189"],
            vec!["2", "174", "324", "2", "71", "221"],
            vec!["2", "587", "737", "2", "672", "822"],
        ];
        validate_output(&output.stdout, expected);
        Ok(())
    }

    #[test]
    fn test_left_join_bed3() -> Result<()> {
        let a = "tests/datasets/join/a.bed";
        let b = "tests/datasets/join/b.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("join")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("--how")
            .arg("left")
            .output()?;

        let num_intervals = get_n_intervals(&output.stdout);
        assert_eq!(num_intervals, 14);

        let num_fields = get_n_fields(&output.stdout);
        assert_eq!(num_fields, 6);

        let expected = vec![
            vec!["1", "72", "222", "1", "55", "205"],
            vec!["1", "72", "222", "1", "69", "219"],
            vec!["1", "72", "222", "1", "93", "243"],
            vec!["1", "72", "222", "1", "156", "306"],
            vec!["1", "257", "407", "1", "156", "306"],
            vec!["1", "268", "418", "1", "156", "306"],
            vec!["1", "467", "617", "1", "603", "753"],
            vec!["1", "819", "969", "1", "837", "987"],
            vec!["2", "174", "324", "2", "39", "189"],
            vec!["2", "174", "324", "2", "71", "221"],
            vec!["2", "587", "737", "2", "672", "822"],
            vec!["3", "395", "545", "0", "0", "0"],
            vec!["3", "554", "704", "0", "0", "0"],
            vec!["3", "653", "803", "0", "0", "0"],
        ];
        validate_output(&output.stdout, expected);
        Ok(())
    }

    #[test]
    fn test_right_join_bed3() -> Result<()> {
        let a = "tests/datasets/join/a.bed";
        let b = "tests/datasets/join/b.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("join")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("--how")
            .arg("right")
            .output()?;

        let num_intervals = get_n_intervals(&output.stdout);
        assert_eq!(num_intervals, 12);

        let num_fields = get_n_fields(&output.stdout);
        assert_eq!(num_fields, 6);

        let expected = vec![
            vec!["1", "72", "222", "1", "55", "205"],
            vec!["1", "72", "222", "1", "69", "219"],
            vec!["1", "72", "222", "1", "93", "243"],
            vec!["1", "72", "222", "1", "156", "306"],
            vec!["1", "257", "407", "1", "156", "306"],
            vec!["1", "268", "418", "1", "156", "306"],
            vec!["1", "467", "617", "1", "603", "753"],
            vec!["1", "819", "969", "1", "837", "987"],
            vec!["2", "174", "324", "2", "39", "189"],
            vec!["2", "174", "324", "2", "71", "221"],
            vec!["2", "587", "737", "2", "672", "822"],
            vec!["0", "0", "0", "3", "138", "288"],
        ];
        validate_output(&output.stdout, expected);
        Ok(())
    }

    #[test]
    fn test_named_left_join_bed3() -> Result<()> {
        let a = "tests/datasets/join/a.named.bed";
        let b = "tests/datasets/join/b.named.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("join")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("--how")
            .arg("left")
            .output()?;

        let num_intervals = get_n_intervals(&output.stdout);
        assert_eq!(num_intervals, 14);

        let num_fields = get_n_fields(&output.stdout);
        assert_eq!(num_fields, 6);

        let expected = vec![
            vec!["chr.1", "72", "222", "chr.1", "55", "205"],
            vec!["chr.1", "72", "222", "chr.1", "69", "219"],
            vec!["chr.1", "72", "222", "chr.1", "93", "243"],
            vec!["chr.1", "72", "222", "chr.1", "156", "306"],
            vec!["chr.1", "257", "407", "chr.1", "156", "306"],
            vec!["chr.1", "268", "418", "chr.1", "156", "306"],
            vec!["chr.1", "467", "617", "chr.1", "603", "753"],
            vec!["chr.1", "819", "969", "chr.1", "837", "987"],
            vec!["chr.2", "174", "324", "chr.2", "39", "189"],
            vec!["chr.2", "174", "324", "chr.2", "71", "221"],
            vec!["chr.2", "587", "737", "chr.2", "672", "822"],
            vec!["chr.3", "395", "545", ".", "0", "0"],
            vec!["chr.3", "554", "704", ".", "0", "0"],
            vec!["chr.3", "653", "803", ".", "0", "0"],
        ];
        validate_output(&output.stdout, expected);
        Ok(())
    }

    #[test]
    fn test_named_right_join_bed3() -> Result<()> {
        let a = "tests/datasets/join/a.named.bed";
        let b = "tests/datasets/join/b.named.bed";

        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("join")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("--how")
            .arg("right")
            .output()?;

        let num_intervals = get_n_intervals(&output.stdout);
        assert_eq!(num_intervals, 12);

        let num_fields = get_n_fields(&output.stdout);
        assert_eq!(num_fields, 6);

        let expected = vec![
            vec!["chr.1", "72", "222", "chr.1", "55", "205"],
            vec!["chr.1", "72", "222", "chr.1", "69", "219"],
            vec!["chr.1", "72", "222", "chr.1", "93", "243"],
            vec!["chr.1", "72", "222", "chr.1", "156", "306"],
            vec!["chr.1", "257", "407", "chr.1", "156", "306"],
            vec!["chr.1", "268", "418", "chr.1", "156", "306"],
            vec!["chr.1", "467", "617", "chr.1", "603", "753"],
            vec!["chr.1", "819", "969", "chr.1", "837", "987"],
            vec!["chr.2", "174", "324", "chr.2", "39", "189"],
            vec!["chr.2", "174", "324", "chr.2", "71", "221"],
            vec!["chr.2", "587", "737", "chr.2", "672", "822"],
            vec![".", "0", "0", "chr.3", "138", "288"],
        ];
        validate_output(&output.stdout, expected);
        Ok(())
    }

    #[test]
    fn test_join_bed3_bed6() -> Result<()> {
        let a = "tests/datasets/join/a.bed";
        let b = "tests/datasets/join/b.bed6";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("join").arg("-a").arg(a).arg("-b").arg(b).output()?;
        let num_intervals = get_n_intervals(&output.stdout);
        assert_eq!(num_intervals, 11);
        let num_fields = get_n_fields(&output.stdout);
        assert_eq!(num_fields, 9);
        Ok(())
    }

    #[test]
    fn test_join_bed6_bed3() -> Result<()> {
        let a = "tests/datasets/join/a.bed6";
        let b = "tests/datasets/join/b.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("join").arg("-a").arg(a).arg("-b").arg(b).output()?;
        let num_intervals = get_n_intervals(&output.stdout);
        assert_eq!(num_intervals, 11);
        let num_fields = get_n_fields(&output.stdout);
        assert_eq!(num_fields, 9);
        Ok(())
    }

    #[test]
    fn test_join_bed6_bed6() -> Result<()> {
        let a = "tests/datasets/join/a.bed6";
        let b = "tests/datasets/join/b.bed6";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("join").arg("-a").arg(a).arg("-b").arg(b).output()?;
        let num_intervals = get_n_intervals(&output.stdout);
        assert_eq!(num_intervals, 11);
        let num_fields = get_n_fields(&output.stdout);
        assert_eq!(num_fields, 12);
        Ok(())
    }
}
