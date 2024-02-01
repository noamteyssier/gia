#[cfg(test)]
mod testing {
    use anyhow::Result;
    use assert_cmd::prelude::*;
    use std::process::Command;

    fn build_expected_str(
        expected: &Vec<(u32, u32, u32, Option<u32>, Option<u32>, Option<u32>)>,
    ) -> String {
        expected
            .iter()
            .map(|(c1, s1, e1, c2, s2, e2)| {
                if c2 == &None || s2 == &None || e2 == &None {
                    return format!("{}\t{}\t{}\t\t\t\n", c1, s1, e1);
                } else {
                    return format!(
                        "{}\t{}\t{}\t{}\t{}\t{}\n",
                        c1,
                        s1,
                        e1,
                        c2.unwrap(),
                        s2.unwrap(),
                        e2.unwrap()
                    );
                }
            })
            .collect::<Vec<String>>()
            .join("")
    }

    #[test]
    fn test_closest() -> Result<()> {
        let a = "tests/datasets/closest/closest_a.bed";
        let b = "tests/datasets/closest/closest_b.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("closest")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .output()?;

        let expected = vec![
            (1, 10, 20, Some(1), Some(22), Some(23)),
            (1, 30, 40, Some(1), Some(42), Some(43)),
            (1, 50, 60, Some(1), Some(42), Some(43)),
            // (2, 10, 20, None, None, None),
            // This is no longer returned because there is no
            // closest interval to be found
        ];
        let expected_str = build_expected_str(&expected);

        println!("Expected:\n{}", expected_str);
        println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));

        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_closest_downstream() -> Result<()> {
        let a = "tests/datasets/closest/closest_a.bed";
        let b = "tests/datasets/closest/closest_b.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("closest")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("-d")
            .output()?;

        let expected = vec![
            (1, 10, 20, Some(1), Some(22), Some(23)),
            (1, 30, 40, Some(1), Some(42), Some(43)),
            // (1, 50, 60, None, None, None),
            // (2, 10, 20, None, None, None),
        ];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }

    #[test]
    fn test_closest_upstream() -> Result<()> {
        let a = "tests/datasets/closest/closest_a.bed";
        let b = "tests/datasets/closest/closest_b.bed";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("closest")
            .arg("-a")
            .arg(a)
            .arg("-b")
            .arg(b)
            .arg("-u")
            .output()?;

        let expected = vec![
            // (1, 10, 20, None, None, None),
            (1, 30, 40, Some(1), Some(22), Some(23)),
            (1, 50, 60, Some(1), Some(42), Some(43)),
            // (2, 10, 20, None, None, None),
        ];
        let expected_str = build_expected_str(&expected);
        assert_eq!(output.stdout, expected_str.as_bytes());
        Ok(())
    }
}
