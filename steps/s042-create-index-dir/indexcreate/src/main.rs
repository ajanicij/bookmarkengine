use clap::Parser;
use std::path::PathBuf;
use std::error::Error;
use std::fs;

#[derive(Parser, Debug)]
#[command(name = "indexcreate")]
#[command(about = "Write specified index directory", long_about = None)]
struct Args {
    /// Directory containing the index
    #[arg(short = 'i', long, value_name = "PATH")]
    index: PathBuf,
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
    let index = args.index;
    let index_str = index.into_os_string().into_string().unwrap();
    let res = create_if_not_file(&index_str)?;
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
