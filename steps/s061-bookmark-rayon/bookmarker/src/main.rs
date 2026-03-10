use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::fs;
use std::io::BufRead;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::mpsc;
use std::thread;
use rayon::prelude::*;

mod scanner;
mod bookmark_item;
mod bookmark_token;
mod utils;

use crate::BookmarkScanner;

use crate::scanner::*;
use crate::utils::*;
use crate::bookmark_item::*;

const NUM_WORKERS: usize = 16;

#[derive(Parser)]
#[command(name = "write")]
struct Args {
    #[command(subcommand)]
    command: Command
}

#[derive(Subcommand, Debug)]
enum Command {
    Write {
        #[arg(long, short)]
        index: PathBuf,

        #[arg(long, short)]
        bookmark: PathBuf,

        #[arg(long, short, default_value = "")]
        max_age: String,

        #[arg(long, short, default_value = "")]
        commit_period: u32,

        #[arg(long, default_value = "50MB")]
        memory_budget: String,
    },

    Search {
        #[arg(long, short)]
        index: PathBuf,

        #[arg(long, short)]
        query: String,
    }
}

fn main() {
    match run() {
        Ok(msg) => println!("Success: {}", msg),
        Err(error) => println!("Error: {}", error),
    }
}

fn run() -> Result<String, Box<dyn Error>> {
    let args = Args::parse();
    // println!("command: {:?}", args.command);
    match args.command {
        Command::Write{index, bookmark, max_age, commit_period, memory_budget} => {
            // let index_str = index.into_os_string().into_string().unwrap();

            // println!("write(index={:?}, bookmark={:?}, max_age={:?}, commit_period={:?}, memory_budget={:?}",
            //     index, bookmark, max_age, commit_period, memory_budget);
            cmd_write(index, bookmark, max_age, commit_period, memory_budget)?;
        },
        Command::Search{index, query} => {
            // println!("search(index={:?}, query={:?}", index, query);
            cmd_search(index, query)?;
        },
    }
    Ok("Done".to_string())
}

fn cmd_search(index: PathBuf, query: String) -> Result<String, Box<dyn Error>> {
    if !directory_exists(&index) {
        return Err(Box::from(format!("{:?} doesn't exist or is not a directory", index)));
    }
    let _ = utils::search(&index, &query)?;
    Ok("Done".to_string())
}

fn cmd_write(index: PathBuf, bookmarks: PathBuf, max_age: String, commit_period: u32,
    memory_budget: String) ->
Result<String, Box<dyn Error>> {
    let max_age_val: Option<u32> = if max_age == "" {
        None
    } else {
        match max_age.parse::<u32>() {
            Ok(value) => Some(value),
            Err(_err) => {
                let res = format!("Maximum age should be a number: {}", max_age);
                return Err(Box::from(res));
            }
        }
    };

    // let res = parse_size(size);
    let memory;
    if let Ok(memory_budget) = parse_size(&memory_budget) {
        memory = memory_budget;
    } else {
        let res = format!("Bad memory budget: {}", memory_budget);
        return Err(Box::from(res));
    }

    // println!("Maximum age: {:?}", max_age_val);
    // println!("Memory budget: {}", memory);

    let index = index;
    let index_str = index.into_os_string().into_string().unwrap();
    let _ = create_if_not_file(&index_str)?;

    let f = File::open(bookmarks)?;
    let reader = BufReader::new(f);
    let mut scanner = BookmarkScanner::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            // println!("Got line: {}", line);
            scanner.scan(&line, max_age_val);
        }
    }

    // Count the bookmarks.
    let total_count = scanner.bookmarks.len();

    println!("Indexing {} bookmarks", total_count);

    let (tx, rx) = mpsc::channel::<utils::BookmarkMessage>();

    // let s = std::time::Instant::now();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(NUM_WORKERS) // use one thread per work slice
        .build()
        .unwrap();

    // Run the receiving thread.
    let handle = thread::spawn(move || {
        // Process bookmarks.
        let pb = ProgressBar::new(total_count as u64);

        pb.set_style(
            ProgressStyle::with_template("({pos}/{len}) [{elapsed_precise}] ETA {eta} {msg}")
                .unwrap()
        );

        let mut indexer = utils::Indexer::new(&index_str, memory)
            .expect("Failed to create indexer");
        let mut counter = 1;
    
        for message in rx {
            // println!("Got message (counter={})", counter);
            counter += 1;
            pb.set_message(format!("Indexing page (counter={})", counter));
            pb.inc(1);
            if let Err(_) = indexer.write(message) {
                continue;
            }
            if commit_period > 0 && counter == commit_period {
                indexer.commit().expect("Failed to commit");
                counter = 0;
            }
        }
        indexer.commit().expect("Failed to commit");
    });

    // Run the sending functions in the thread pool.
    pool.install(|| {
        scanner.bookmarks
            .into_par_iter()
            .for_each_with(tx.clone(), |tx, bookmark| {
                match bookmark {
                    Item::Bookmark{ description: _, path: _, href: _, last_modified: _, } => {
                        // println!("Fetching page: {:?}", bookmark);
                        let result = get_page(&bookmark);
                        let mut text = "".to_string();
                        if let Ok(result) = result {
                            text = result;
                        }
                        let message = utils::BookmarkMessage {
                            bookmark,
                            text,
                        };
                        // println!("Sending result to the channel");
                        if let Err(err) = tx.send(message) {
                            eprintln!("Error sending bookmark: {}", err);
                        }
                    },
                    _ => {
                        let message = utils::BookmarkMessage {
                            bookmark,
                            text: "".to_string(),
                        };
                        if let Err(err) = tx.send(message) {
                            eprintln!("Error sending bookmark: {}", err);
                        }
                    }
                }
                // TODO: do something with error result.
            });
    });

    // println!("Before drop");
    drop(tx);
    let _ = handle.join();

    Ok("".to_string())
}

fn create_if_not_file(entry: &str) -> Result<String, String> {
    if let Ok(metadata) = fs::metadata(entry) {
        let file_type = metadata.file_type();
        if file_type.is_file() {
            return Err(format!("{} is a regular file", entry));
        } else if file_type.is_dir() {
            return Ok(format!("{} already exists", entry));
        }
    } else {
        // Try to create directory.
        match fs::create_dir(entry) {
            Ok(()) => return Ok(format!("created directory {}", entry)),
            Err(err) => {
                return Err(format!("couldn't create directory {}: {}", entry, err));
            }
        }
    }
    Err("unknown".to_string())
}

fn directory_exists(path: &PathBuf) -> bool {
    return path.exists() && path.is_dir();
}