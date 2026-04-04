use serde::Deserialize;
use std::error::Error;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[allow(dead_code)] // Disable dead code warning for the entire struct
    pub common: CommonConfig,
    #[allow(dead_code)]
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct CommonConfig {
    #[allow(dead_code)] // Disable dead code warning for the entire struct
    pub db: PathBuf,
    #[allow(dead_code)]
    pub bookmarks: PathBuf,
    #[allow(dead_code)]
    pub index: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    #[allow(dead_code)] // Disable dead code warning for the entire struct
    pub port: u32,
    #[allow(dead_code)] // Disable dead code warning for the entire struct
    pub apikey: String,
}

impl Config {
    pub fn load(path: &PathBuf) -> Result<Config, Box<dyn Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
    
        let config_toml: Config = toml::from_str(&contents)?;
        Ok(config_toml)
    }
}
