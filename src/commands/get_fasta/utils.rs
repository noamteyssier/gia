use anyhow::Result;
use std::io::Write;

fn reverse_complement(shared_buffer: &mut Vec<u8>, seq_buffer: &[u8]) {
    for &c in seq_buffer.iter().rev() {
        let char = match c {
            b'A' => b'T',
            b'C' => b'G',
            b'G' => b'C',
            b'T' => b'A',
            b'a' => b't',
            b'c' => b'g',
            b'g' => b'c',
            b't' => b'a',
            b'\n' => continue,
            _ => c,
        };
        shared_buffer.push(char);
    }
}

/// Revese Complement and Transcribe a DNA sequence to RNA
fn rc_transcribe(shared_buffer: &mut Vec<u8>, seq_buffer: &[u8]) {
    for &c in seq_buffer.iter().rev() {
        let char = match c {
            b'A' => b'U',
            b'C' => b'G',
            b'G' => b'C',
            b'T' => b'A',
            b'a' => b'u',
            b'c' => b'g',
            b'g' => b'c',
            b't' => b'a',
            b'\n' => continue,
            _ => c,
        };
        shared_buffer.push(char);
    }
}

fn remove_newline(shared_buffer: &mut Vec<u8>, seq_buffer: &[u8]) {
    for &c in seq_buffer {
        if c != b'\n' {
            shared_buffer.push(c);
        }
    }
}

pub fn write_sequence<W: Write>(
    shared_buffer: &mut Vec<u8>,
    seq_buffer: &[u8],
    revcomp: bool,
    rna: bool,
    output: &mut W,
) -> Result<()> {
    shared_buffer.clear();
    match (revcomp, rna) {
        (true, true) => rc_transcribe(shared_buffer, seq_buffer),
        (true, false) => reverse_complement(shared_buffer, seq_buffer),
        (false, true) | (false, false) => remove_newline(shared_buffer, seq_buffer),
    }
    shared_buffer.push(b'\n');
    output.write_all(shared_buffer)?;
    Ok(())
}

#[cfg(test)]
mod testing {

    use super::*;

    #[test]
    fn test_reverse_complement() {
        let mut shared_buffer = Vec::new();
        let seq_buffer = b"ATCGatcg";
        reverse_complement(&mut shared_buffer, seq_buffer);
        assert_eq!(shared_buffer, b"cgatCGAT");
    }

    #[test]
    fn test_reverse_complement_with_embedded_newline() {
        let mut shared_buffer = Vec::new();
        let seq_buffer = b"ATCG\natcg";
        reverse_complement(&mut shared_buffer, seq_buffer);
        assert_eq!(shared_buffer, b"cgatCGAT");
    }

    #[test]
    fn test_rc_transcribe() {
        let mut shared_buffer = Vec::new();
        let seq_buffer = b"ATCGatcg";
        rc_transcribe(&mut shared_buffer, seq_buffer);
        assert_eq!(shared_buffer, b"cgauCGAU");
    }

    #[test]
    fn test_rc_transcribe_with_embedded_newline() {
        let mut shared_buffer = Vec::new();
        let seq_buffer = b"ATCG\natcg";
        rc_transcribe(&mut shared_buffer, seq_buffer);
        assert_eq!(shared_buffer, b"cgauCGAU");
    }

    #[test]
    fn test_remove_newline() {
        let mut shared_buffer = Vec::new();
        let seq_buffer = b"ATCG\natcg";
        remove_newline(&mut shared_buffer, seq_buffer);
        assert_eq!(shared_buffer, b"ATCGatcg");
    }

    #[test]
    // Test writing a sequence with no flags set
    fn test_write_sequence_ff() {
        let mut shared_buffer = Vec::new();
        let seq_buffer = b"ATCG\natcg";
        let mut output = Vec::new();
        write_sequence(&mut shared_buffer, seq_buffer, false, false, &mut output).unwrap();
        assert_eq!(output, b"ATCGatcg\n");
    }

    #[test]
    // Test writing a sequence with revcomp flag set
    fn test_write_sequence_tf() {
        let mut shared_buffer = Vec::new();
        let seq_buffer = b"ATCG\natcg";
        let mut output = Vec::new();
        write_sequence(&mut shared_buffer, seq_buffer, true, false, &mut output).unwrap();
        assert_eq!(output, b"cgatCGAT\n");
    }

    #[test]
    // Test writing a sequence with rna flag set but not revcomp
    // This should not change the sequence
    fn test_write_sequence_ft() {
        let mut shared_buffer = Vec::new();
        let seq_buffer = b"ATCG\natcg";
        let mut output = Vec::new();
        write_sequence(&mut shared_buffer, seq_buffer, false, true, &mut output).unwrap();
        assert_eq!(output, b"ATCGatcg\n");
    }

    #[test]
    // Test writing a sequence with rna and revcomp flags set
    fn test_write_sequence_tt() {
        let mut shared_buffer = Vec::new();
        let seq_buffer = b"ATCG\natcg";
        let mut output = Vec::new();
        write_sequence(&mut shared_buffer, seq_buffer, true, true, &mut output).unwrap();
        assert_eq!(output, b"cgauCGAU\n");
    }
}
