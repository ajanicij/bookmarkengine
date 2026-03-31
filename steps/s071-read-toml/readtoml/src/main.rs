use toml;
use std::fs::File;
use std::io::Read;

use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct ConfigToml {
    #[allow(dead_code)] // Disable dead code warning for the entire struct
    server: Server,
    #[allow(dead_code)]
    database: Database,
}

#[derive(Debug, Deserialize)]
struct Server {
    #[allow(dead_code)]
    port: i32,
    #[allow(dead_code)]
    host: String,
    #[allow(dead_code)]
    debug: bool,
}

#[derive(Debug, Deserialize)]
struct Database {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    username: String,
    #[allow(dead_code)]
    password: String,
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
    let mut file = File::open("config.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // At this point, `contents` contains the content of the TOML file
    println!("{}", contents);

    let config_toml: ConfigToml = toml::from_str(&contents)?;
    println!("Config: {:?}", config_toml);
    Ok(())
}
