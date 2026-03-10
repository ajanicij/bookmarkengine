use scanner::utils::*;
use std::error::Error;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "search")]
#[command(about = "Search for a query in Tantivy index", long_about = None)]
struct Args {
    /// Directory containing the index
    #[arg(short = 'i', long, value_name = "PATH")]
    index: PathBuf,

    /// Search term
    #[arg(short = 'q', long, value_name = "QUERY")]
    query: String,
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
    println!("Query: {:?}", args.query);

    let search_term = &args.query;

    println!("Search term: {}", search_term);

    if let Some(index_str) = args.index.as_path().to_str() {
        search(index_str, search_term)?;
    }

    Ok(())
}
