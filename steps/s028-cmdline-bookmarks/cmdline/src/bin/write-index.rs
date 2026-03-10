use cmdline::utils::*;
use clap::Parser;
use std::path::PathBuf;
use url::Url;
use std::error::Error;

#[derive(Parser, Debug)]
#[command(name = "write-index")]
#[command(about = "Write search index for specified URLs", long_about = None)]
struct Args {
    /// Directory containing the index
    #[arg(short = 'i', long, value_name = "PATH")]
    index: PathBuf,

    /// URL to process (can be provided multiple times)
    #[arg(short = 'u', long, value_name = "URL", required = true)]
    url: Vec<Url>,
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

    println!("URLs:");
    for url in &args.url {
        println!("  - {}", url);
        if let Some(index_str) = args.index.as_path().to_str() {
            write_index(index_str, url.as_str())?;
        }
    }

    Ok(())
}
