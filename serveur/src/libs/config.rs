use std::fs;
use lazy_static::lazy_static;

use super::structures::Config;

lazy_static! {
    pub static ref CONFIG: Config = load_config("src/libs/configs/config.json").expect("Failed to load config");
}

pub fn load_config(filename: &str) -> Result<Config, Box<dyn std::error::Error>> 
{
    let contents = fs::read_to_string(filename)?;
    let config: Config = serde_json::from_str(&contents)?;
    Ok(config)
}
