use anyhow::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

pub fn match_input(input: Option<String>) -> Result<Box<dyn BufRead>> {
    match input {
        Some(filename) => {
            let file = File::open(filename)?;
            let buffer = BufReader::new(file);
            Ok(Box::new(buffer))
        }
        None => {
            let stdin = std::io::stdin();
            let handle = stdin.lock();
            let buffer = BufReader::new(handle);
            Ok(Box::new(buffer))
        }
    }
}

pub fn match_output(output: Option<String>) -> Result<Box<dyn Write + Send>> {
    match output {
        Some(filename) => {
            let file = File::create(filename)?;
            let buffer = BufWriter::new(file);
            Ok(Box::new(buffer))
        }
        None => {
            let stdout = std::io::stdout();
            let buffer = BufWriter::new(stdout);
            Ok(Box::new(buffer))
        }
    }
}
