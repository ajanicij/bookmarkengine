use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let f = File::open("input.txt")?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    for line in reader.lines() {
        let line = line.unwrap();
        println!("Got line: {}", line);
    }

    Ok(())
}
