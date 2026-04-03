use toml;
use std::fs::File;
use std::io::Read;

use serde::Deserialize;
use std::error::Error;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "readconfig")]
#[command(about = "Program that reads and parses a config file", long_about = None)]
struct Args {
    /// Directory containing the index
    #[arg(short = 'c', long, value_name = "PATH")]
    config: PathBuf,
}

#[derive(Debug, Deserialize)]
struct Config {
    #[allow(dead_code)] // Disable dead code warning for the entire struct
    common: CommonConfig,
    #[allow(dead_code)]
    server: ServerConfig,
}

#[derive(Debug, Deserialize)]
struct CommonConfig {
    #[allow(dead_code)] // Disable dead code warning for the entire struct
    db: PathBuf,
    #[allow(dead_code)]
    bookmark: PathBuf,
    #[allow(dead_code)]
    index: PathBuf,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    #[allow(dead_code)] // Disable dead code warning for the entire struct
    port: u32,
    #[allow(dead_code)] // Disable dead code warning for the entire struct
    apikey: String,
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

    let config = args.config;

    let mut file = File::open(config)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // At this point, `contents` contains the content of the TOML file
    println!("{}", contents);

    let config_toml: Config = toml::from_str(&contents)?;
    println!("Config: {:?}", config_toml);
    Ok(())
}
