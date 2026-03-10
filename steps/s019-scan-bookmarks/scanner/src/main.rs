use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

mod scanner;

fn main() -> std::io::Result<()> {
    let f = File::open("input.html")?;
    let reader = BufReader::new(f);

/*
    while let Ok(len) = reader.read_line(&mut buffer) {
        if len == 0 {
            println!("Reached EOF");
            break;
        }
        println!("Read line: len={}: line={}", len, buffer.trim_end());
        buffer.clear();
    }
*/

    for line in reader.lines() {
        if let Ok(line) = line {
            println!("Got line: {}", line);
            scanner::scan(&line);
        }
    }

    Ok(())
}
