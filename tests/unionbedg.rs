#[cfg(test)]
mod testing {
    use anyhow::Result;
    use assert_cmd::prelude::*;
    use std::process::Command;

    fn format_6col(data: &[(usize, usize, usize, f64, f64, f64)]) -> String {
        let mut out = String::new();
        for (a, b, c, d, e, f) in data {
            out.push_str(&format!("{a}\t{b}\t{c}"));
            for score in [d, e, f].iter() {
                if **score < f64::EPSILON {
                    out.push_str("\t0.0");
                } else {
                    out.push_str(&format!("\t{score}"));
                }
            }
            out.push('\n');
        }
        out
    }

    fn format_5col(data: &[(usize, usize, usize, f64, f64)]) -> String {
        let mut out = String::new();
        for (a, b, c, d, e) in data {
            out.push_str(&format!("{a}\t{b}\t{c}"));
            for score in [d, e].iter() {
                if **score < f64::EPSILON {
                    out.push_str("\t0.0");
                } else {
                    out.push_str(&format!("\t{score}"));
                }
            }
            out.push('\n');
        }
        out
    }

    #[test]
    fn test_union_abc() -> Result<()> {
        let a = "tests/datasets/unionbedg/set_a.bg";
        let b = "tests/datasets/unionbedg/set_b.bg";
        let c = "tests/datasets/unionbedg/set_c.bg";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd
            .arg("unionbedg")
            .arg("-i")
            .arg(a)
            .arg(b)
            .arg(c)
            .output()?;
        let expected = vec![
            (0, 0, 100, 0.13609125163867486, 0.0, 0.0),
            (0, 100, 200, 0.9324660871019884, 0.0, 0.0),
            (0, 200, 300, 0.2676953573001045, 0.07970120821650206, 0.0),
            (0, 300, 400, 0.10039781237551093, 0.19211244407634076, 0.0),
            (0, 400, 500, 0.0, 0.0009236532951610865, 0.32625953468180857),
            (0, 500, 600, 0.0, 0.7542068747269175, 0.8104941293677845),
            (0, 600, 700, 0.0, 0.0, 0.761750021822635),
            (0, 700, 800, 0.0, 0.0, 0.11991722896448598),
        ];
        let expected_str = format_6col(&expected);
        let observed_str = String::from_utf8(output.stdout)?;
        println!("Expected:\n{expected_str}");
        println!("Observed:\n{observed_str}");
        assert_eq!(observed_str, expected_str);
        Ok(())
    }

    #[test]
    fn test_union_ab() -> Result<()> {
        let a = "tests/datasets/unionbedg/set_a.bg";
        let b = "tests/datasets/unionbedg/set_b.bg";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("unionbedg").arg("-i").arg(a).arg(b).output()?;
        let expected = vec![
            (0, 0, 100, 0.13609125163867486, 0.0),
            (0, 100, 200, 0.9324660871019884, 0.0),
            (0, 200, 300, 0.2676953573001045, 0.07970120821650206),
            (0, 300, 400, 0.10039781237551093, 0.19211244407634076),
            (0, 400, 500, 0.0, 0.0009236532951610865),
            (0, 500, 600, 0.0, 0.7542068747269175),
        ];
        let expected_str = format_5col(&expected);
        let observed_str = String::from_utf8(output.stdout)?;
        println!("Expected:\n{expected_str}");
        println!("Observed:\n{observed_str}");
        assert_eq!(observed_str, expected_str);
        Ok(())
    }

    #[test]
    fn test_union_ac() -> Result<()> {
        let a = "tests/datasets/unionbedg/set_a.bg";
        let c = "tests/datasets/unionbedg/set_c.bg";
        let mut cmd = Command::cargo_bin("gia")?;
        let output = cmd.arg("unionbedg").arg("-i").arg(a).arg(c).output()?;
        let expected = vec![
            (0, 0, 100, 0.13609125163867486, 0.0),
            (0, 100, 200, 0.9324660871019884, 0.0),
            (0, 200, 300, 0.2676953573001045, 0.0),
            (0, 300, 400, 0.10039781237551093, 0.0),
            (0, 400, 500, 0.0, 0.32625953468180857),
            (0, 500, 600, 0.0, 0.8104941293677845),
            (0, 600, 700, 0.0, 0.761750021822635),
            (0, 700, 800, 0.0, 0.11991722896448598),
        ];
        let expected_str = format_5col(&expected);
        let observed_str = String::from_utf8(output.stdout)?;
        println!("Expected:\n{expected_str}");
        println!("Observed:\n{observed_str}");
        assert_eq!(observed_str, expected_str);
        Ok(())
    }
}
