use clap::Parser;
use std::path::PathBuf;
use url::Url;

#[derive(Parser, Debug)]
#[command(name = "cmdlineexample")]
#[command(about = "Example program using clap", long_about = None)]
struct Args {
    /// Directory containing the index
    #[arg(short = 'i', long, value_name = "PATH")]
    index: PathBuf,

    /// URL to process (can be provided multiple times)
    #[arg(short = 'u', long, value_name = "URL", required = true)]
    url: Vec<Url>,
}

fn main() {
    let args = Args::parse();

    println!("Index directory: {:?}", args.index);

    println!("URLs:");
    for url in &args.url {
        println!("  - {}", url);
    }
}
