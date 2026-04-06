use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::fs;
use std::io::BufRead;

mod scanner;
mod item;
mod token;
mod utils;
mod db;
mod config;
use crate::BookmarkScanner;

use crate::scanner::*;
use crate::utils::*;

const DEFAULT_CONFIG: &str = ".bookmarkengine.cfg";

#[derive(Parser)]
#[command(name = "write")]
struct Args {
    #[command(subcommand)]
    command: Command
}

#[derive(Subcommand, Debug)]
enum Command {
    Write {
        #[arg(long, short, default_value = None)]
        index: Option<PathBuf>,

        #[arg(long, short)]
        bookmarks: Option<PathBuf>,

        #[arg(long)]
        db: Option<PathBuf>,

        #[arg(long, short, default_value = "")]
        max_age: String,

        #[arg(long, default_value = "0")]
        commit_period: u32,

        #[arg(long, default_value = "50MB")]
        memory_budget: String,

        #[arg(long, default_value = "20")]
        threads: usize,

        #[arg(long, short)]
        config: Option<PathBuf>,
    },

    Search {
        #[arg(long, short)]
        index: Option<PathBuf>,

        #[arg(long)]
        db: Option<PathBuf>,

        #[arg(long, short)]
        query: String,

        #[arg(long, short = 'n', help = "Number of results (0 for all)", default_value = "0")]
        num_results: u32,

        #[arg(long, short)]
        config: Option<PathBuf>,
    }
}

fn main() {
    match run() {
        Ok(_msg) => std::process::exit(0),
        Err(error) => {
            println!("Error: {}", error);
            std::process::exit(-1);
        }
    }
}

fn run() -> Result<String, Box<dyn Error>> {
    let args = Args::parse();
    match args.command {
        Command::Write{
            index,
            bookmarks,
            db,
            max_age,
            commit_period,
            memory_budget, 
            threads,
            config} => {
            // let index_str = index.into_os_string().into_string().unwrap();

            // println!("write(index={:?}, bookmark={:?}, max_age={:?}, commit_period={:?}, memory_budget={:?}",
            //     index, bookmark, max_age, commit_period, memory_budget);
            cmd_write(index, bookmarks, db, max_age, commit_period, memory_budget, threads, config)?;
        },
        Command::Search{
            index,
            db,
            query,
            num_results,
            config} => {
            // println!("search(index={:?}, query={:?}", index, query);
            cmd_search(index, db, query, num_results, config)?;
        },
    }
    Ok("Done".to_string())
}

fn cmd_search(index_opt: Option<PathBuf>, db_opt: Option<PathBuf>, query: String, num_results: u32, config_opt: Option<PathBuf>) -> Result<String, Box<dyn Error>> {
    let mut default_config: Option<PathBuf> = None;
    if let Some(home_dir) = std::env::home_dir() {
        default_config = Some(home_dir.join(DEFAULT_CONFIG));
    }

    let mut config: Option<config::Config> = None;
    if let Some(config_file) = config_opt {
        match config::Config::load(&config_file) {
            Ok(config_obj) => {
                println!("Reading from configuration file: {}", config_file.display());
                config = Some(config_obj)
            },
            Err(err) => return Err(Box::from(format!(
                "Error reading {}: {}", config_file.display(), err)))
        }
    } else if let Some(default_config_file) = default_config {
        match config::Config::load(&default_config_file) {
            Ok(config_obj) => {
                println!("Reading from default configuration file: {}", default_config_file.display());
                config = Some(config_obj)
            },
            Err(err) => return Err(Box::from(format!(
                "Error reading {}: {}", default_config_file.display(), err)))
        }
    }

    // Process configuration.
    // For each configuration parameter, the rule is the same: command-line parameter
    // has a higher precedence than the parameter from the configuration file.
    // If neither is provided, we exit with an error.
    let mut index = index_opt;
    let mut db = db_opt;
    if let Some(config_obj) = config {
        index = index.or(Some(config_obj.common.index));
        db = db.or(Some(config_obj.common.db));
    }
    if index.is_none() {
        return Err(Box::from("index argument missing"));
    }
    if db.is_none() {
        return Err(Box::from("db argument missing"));
    }
    let index = index.unwrap();
    let db = db.unwrap();

    println!("index: {}", index.display());
    println!("db: {}", db.display());
    println!("");

    if !directory_exists(&index) {
        return Err(Box::from(format!("{} doesn't exist or is not a directory", index.display())));
    }

    let mut bookmark_db = db::Db::new(&db)?;

    let _ = utils::search(&index, &mut bookmark_db, &query, num_results)?;
    Ok("Done".to_string())
}

fn cmd_write(index_opt: Option<PathBuf>, bookmarks_opt: Option<PathBuf>, db_opt: Option<PathBuf>, max_age: String, commit_period: u32,
    memory_budget: String, threads: usize, config_opt: Option<PathBuf>) ->
Result<String, Box<dyn Error>> {
    let mut default_config: Option<PathBuf> = None;
    if let Some(home_dir) = std::env::home_dir() {
        default_config = Some(home_dir.join(DEFAULT_CONFIG));
    }

    let mut config: Option<config::Config> = None;
    if let Some(config_file) = config_opt {
        match config::Config::load(&config_file) {
            Ok(config_obj) => {
                println!("Reading from configuration file: {}", config_file.display());
                config = Some(config_obj)
            },
            Err(err) => return Err(Box::from(format!(
                "Error reading {}: {}", config_file.display(), err)))
        }
    } else if let Some(default_config_file) = default_config {
        match config::Config::load(&default_config_file) {
            Ok(config_obj) => {
                println!("Reading from default configuration file: {}", default_config_file.display());
                config = Some(config_obj)
            },
            Err(err) => return Err(Box::from(format!(
                "Error reading {}: {}", default_config_file.display(), err)))
        }
    }

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

    // Process configuration.
    // For each configuration parameter, the rule is the same: command-line parameter
    // has a higher precedence than the parameter from the configuration file.
    // If neither is provided, we exit with an error.
    let mut index = index_opt.clone();
    let mut db = db_opt.clone();
    let mut bookmarks = bookmarks_opt.clone();
    if let Some(ref config_obj) = config {
        index = index.or(Some(config_obj.common.index.clone()));
        db = db.or(Some(config_obj.common.db.clone()));
        bookmarks = bookmarks.or(Some(config_obj.common.bookmarks.clone()));
    }
    if index.is_none() {
        return Err(Box::from("index argument missing"));
    }
    if db.is_none() {
        return Err(Box::from("db argument missing"));
    }
    if bookmarks.is_none() {
        return Err(Box::from("bookmarks argument missing"));
    }
    let index = index.unwrap();
    let db = db.unwrap();
    let bookmarks = bookmarks.unwrap();

    println!("index: {}", index.display());
    println!("db: {}", db.display());
    println!("bookmarks: {}", bookmarks.display());
    println!("");

    let _ = create_if_not_file(&index)?;

    let f = File::open(bookmarks)?;
    let reader = BufReader::new(f);
    let mut scanner = BookmarkScanner::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            // println!("Got line: {}", line);
            scanner.scan(&line, max_age_val);
        }
    }

    let mut bookmark_db = db::Db::new(&db)?;
    bookmark_db.create_db()?;

    // Filter the bookmarks. Only those that are not in the database should
    // be processed.
    let filtered_bookmarks = filter_bookmarks(&scanner.bookmarks, &mut bookmark_db);
    
    // Count the bookmarks.
    let total_count = filtered_bookmarks.len();

    println!("Indexing {} bookmarks", total_count);

    let mut indexer = utils::Indexer::new(&index, memory, bookmark_db)
        .expect("Failed to create indexer");
    indexer.index(filtered_bookmarks, commit_period, threads)?;

    Ok("".to_string())
}

fn create_if_not_file(entry: &PathBuf) -> Result<String, String> {
    if let Ok(metadata) = fs::metadata(entry) {
        let file_type = metadata.file_type();
        if file_type.is_file() {
            return Err(format!("{} is a regular file", entry.display()));
        } else if file_type.is_dir() {
            return Ok(format!("{} already exists", entry.display()));
        }
    } else {
        // Try to create directory.
        match fs::create_dir(entry) {
            Ok(()) => return Ok(format!("created directory {}", entry.display())),
            Err(err) => {
                return Err(format!("couldn't create directory {}: {}", entry.display(), err));
            }
        }
    }
    Err("unknown".to_string())
}

fn directory_exists(path: &PathBuf) -> bool {
    return path.exists() && path.is_dir();
}

fn filter_bookmarks(bookmarks: &Vec<item::Item>, bookmark_db: &mut db::Db)
-> Vec<item::Item> {
    let mut result: Vec<item::Item> = vec![];
    for bookmark in bookmarks {
        if let item::Item::Bookmark { path, href, .. } = bookmark {
            let bookmark_db_record = db::Bookmark {
                description: None,
                path: path.clone(),
                href: href.clone(),
                last_modified: 0,
            };
            if let Ok(exists) = bookmark_db.exists(&bookmark_db_record) {
                if exists {
                    continue;
                }
            }
        }
        result.push(bookmark.clone());
    }
    result
}
