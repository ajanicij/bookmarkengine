use scanner::utils::*;
use clap::Parser;
use std::path::PathBuf;
use std::error::Error;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use scanner::scanner::*;
use scanner::bookmark_item::*;

use chrono::{DateTime, Utc, TimeZone};

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

    /// Maximum age of a bookmark - if the bookmark is older than this it will be skipped
    #[arg(short = 'm', long, value_name = "MAX_AGE", default_value = "")]
    max_age: String,
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

    let max_age_str = args.max_age;
    let max_age: Option<i64>;
    max_age = match max_age_str.parse::<i64>() {
        Ok(value) => Some(value),
        Err(_err) => {
            None
        }
    };

    match max_age {
        Some(value) => println!("Maximum age: {}", value),
        None        => println!("No maximum age"),
    }

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
        if let Item::Bookmark{ description: _, path: _, href: _, last_modified: _ } = bookmark {
            count += 1;
        }
    }

    let max_count = 100;

    println!("Indexing {} bookmarks", count);

    println!("--- max_age: {:?}", max_age);

    let mut current_count = 0;
    if let Some(index_str) = args.index.as_path().to_str() {
        for bookmark in &scanner.bookmarks {
            if let Some(max_age_num) = max_age {
                if let Item::Bookmark{ description: _, path: _, href: _, last_modified } = bookmark {
                    if days_from(*last_modified) > max_age_num {
                        // Too old; skip.
                        continue;
                    }
                }
            }

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

pub fn days_from_epoch(epoch_seconds: u32) -> u32 {
    let then: DateTime<Utc> = Utc.timestamp_opt(epoch_seconds as i64, 0)
        .single()
        .expect("Invalid epoch timestamp");

    let now: DateTime<Utc> = Utc::now();

    now.signed_duration_since(then).num_days() as u32
}
