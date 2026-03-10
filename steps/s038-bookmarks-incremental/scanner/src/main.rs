mod scanner;
mod bookmark_token;
mod bookmark_item;
mod utils;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use scanner::*;

fn main() -> std::io::Result<()> {
    let f = File::open("input.html")?;
    let reader = BufReader::new(f);
    let mut scanner = BookmarkScanner::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            // println!("Got line: {}", line);
            scanner.scan(&line);
        }
    }
    // Display all bookmarks.
    println!("Displaing all bookmarks");
    for bookmark in &scanner.bookmarks {
        println!("{:?}", bookmark);
    }

    Ok(())
}
