use std::fs;

use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Data {
    config: Config,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub keys: Vec<String>,
    pub port: u16,
    pub host: String,
}

pub fn read_config() -> Config {
    let filename = "config.toml";

    let contents = match fs::read_to_string(filename) {
        Ok(contents) => contents,
        Err(_) => {
            panic!("Could not read config file {}", filename);
        }
    };

    let data: Data = match toml::from_str(&contents) {
        Ok(data) => data,
        Err(_) => {
            panic!("Could not parse config file {}", filename);
        }
    };

    data.config
}
