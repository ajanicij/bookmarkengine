mod scanner;
mod bookmark_token;
mod bookmark_item;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> std::io::Result<()> {
    let f = File::open("input.html")?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        if let Ok(line) = line {
            // println!("Got line: {}", line);
            let tokens = scanner::scan(&line);
        }
    }

    Ok(())
}
