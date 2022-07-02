use error_stack::{IntoReport, Result, ResultExt, bail};
use serde::{Deserialize, Serialize};
use crate::errors::{AccountError, ConfigError};

mod errors;

fn main() {
    match parse_config() {
        Ok(config_map) => println!("{:?}", config_map),
        Err(e) => println!("{:?}", e),
    }

    match find_nick_by_id(1) {
        Ok(nick) => println!("{}", nick),
        Err(e) => println!("{:?}", e),
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigMap {
    name: String,
}

fn parse_config() -> Result<ConfigMap, ConfigError> {
    let json_file = "config.json";
    let config = std::fs::read_to_string(json_file)
        .report()
        .change_context(ConfigError::NotFound(json_file.to_string()))?;
    let map: ConfigMap = serde_json::from_str(&config)
        .report()
        .change_context(ConfigError::Invalid(json_file.to_string()))?;
    Ok(map)
}

fn find_nick_by_id(id: u64) -> Result<String, AccountError> {
    bail!(AccountError::NotFound(id.to_string()))
}
