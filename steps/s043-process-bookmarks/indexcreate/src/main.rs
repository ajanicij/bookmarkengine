use clap::Parser;
use std::path::PathBuf;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

mod scanner;

#[derive(Parser, Debug)]
#[command(name = "indexcreate")]
#[command(about = "Write specified index directory", long_about = None)]
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
        Ok(msg) => println!("Success: {}", msg),
        Err(error) => println!("Error: {}", error),
    }
}

fn run() -> Result<String, Box<dyn Error>> {
    let args = Args::parse();

    println!("Index directory: {:?}", args.index);
    println!("Bookmark file: {:?}", args.bookmarks);
    let index = args.index;
    let index_str = index.into_os_string().into_string().unwrap();
    let res = create_if_not_file(&index_str)?;

    let f = File::open(args.bookmarks)?;
    let reader = BufReader::new(f);
    // let mut scanner = BookmarkScanner::new();
    println!("here");
    for line in reader.lines() {
        if let Ok(line) = line {
            println!("Got line: {}", line);
            scanner::scan(&line);
        }
    }

    Ok(res)
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
