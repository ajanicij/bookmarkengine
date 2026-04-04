use std::path::PathBuf;
use std::error::Error;

use clap::Parser;

mod config;

#[derive(Parser, Debug)]
#[command(name = "readconfig")]
#[command(about = "Program that reads and parses a config file", long_about = None)]
struct Args {
    /// Directory containing the index
    #[arg(short = 'c', long, value_name = "PATH")]
    config: PathBuf,
}

fn main() {
    match run() {
        Ok(()) => (),
        Err(error) => {
            eprintln!("Error: {}", error)
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("Config file: {:?}", args.config);

    let config_file = args.config;

    let config = config::Config::load(&config_file)?;
    println!("Config: {:?}", config);
    Ok(())
}
