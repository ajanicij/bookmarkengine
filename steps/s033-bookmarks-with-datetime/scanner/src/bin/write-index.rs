use scanner::utils::*;
use clap::Parser;
use std::path::PathBuf;
use std::error::Error;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

// use scanner::*;

use scanner::scanner::*;
use scanner::bookmark_item::*;

#[derive(Parser, Debug)]
#[command(name = "write-index")]
#[command(about = "Write search index for specified URLs", long_about = None)]
struct Args {
    /// Directory containing the index
    #[arg(short = 'i', long, value_name = "PATH")]
    index: PathBuf,

    /// Bookmark file (bookmarks.html)
    #[arg(short = 'b', long, value_name = "BOOKMARKS")]
    bookmarks: String,
}

fn main() {
    match run() {
        Ok(())     => (),
        Err(error) => println!("Error: {}", error),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("Index directory: {:?}", args.index);
    println!("Bookmark file: {:?}", args.bookmarks);

    let f = File::open(args.bookmarks)?;
    let reader = BufReader::new(f);
    let mut scanner = BookmarkScanner::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            // println!("Got line: {}", line);
            scanner.scan(&line);
        }
    }

    // Count bookmarks.
    let mut count = 0;
    for bookmark in &scanner.bookmarks {
        if let Item::Bookmark{ description: _, path: _, href: _, add_date: _ } = bookmark {
            count += 1;
        }
    }

    let max_count = 100;

    println!("Indexing {} bookmarks", count);

    // Display all bookmarks.
    println!("Displaing all bookmarks");
    let mut current_count = 0;
    for bookmark in &scanner.bookmarks {
        // println!("{:?}", bookmark);

        if let Some(index_str) = args.index.as_path().to_str() {
            current_count += 1;
            if current_count % 10 == 0 {
                println!("-- Bookmark # {}", current_count);
            }
            let _ = write_index(index_str, bookmark);
            if current_count == max_count {
                break;
            }
        }
    }

    Ok(())
}
