use anyhow::Result;
use std::io::Write;

fn reverse_complement(shared_buffer: &mut Vec<u8>, seq_buffer: &[u8]) {
    shared_buffer.clear();
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
            _ => c,
        };
        shared_buffer.push(char);
    }
}

pub fn write_sequence<W: Write>(
    shared_buffer: &mut Vec<u8>,
    seq_buffer: &[u8],
    revcomp: bool,
    output: &mut W,
) -> Result<()> {
    if revcomp {
        reverse_complement(shared_buffer, seq_buffer);
        for subseq in shared_buffer.split(|&c| c == b'\n') {
            output.write_all(subseq)?;
        }
    } else {
        for subseq in seq_buffer.split(|&c| c == b'\n') {
            output.write_all(subseq)?;
        }
    }
    output.write_all(b"\n")?;
    Ok(())
}
