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
