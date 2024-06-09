use std::{collections::HashMap, fs::File, io::Read, path::PathBuf};

use anyhow::{anyhow, Result};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    api: Option<String>,
    address: String,
    address_book: HashMap<String, String>,
    log_backend: Option<LogType>,
}

#[derive(Deserialize, Clone)]
pub enum LogType {
    Heap,
    File(String),
}

impl Config {
    pub fn api(&self) -> String {
        let mut endpoint = "127.0.0.1:8000".to_string();
        self.api.as_ref().map(|s| endpoint = s.clone());
        endpoint
    }

    pub fn address(&self) -> String {
        self.address.clone()
    }

    pub fn address_book(&self) -> HashMap<String, String> {
        self.address_book.clone()
    }

    pub fn log_backend(&self) -> LogType {
        let mut log_backend = LogType::Heap;
        self.log_backend.as_ref().map(|s| log_backend = s.clone());
        log_backend
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api: Some("127.0.0.1:8000".to_string()),
            address: "127.0.0.1:18000".to_string(),
            address_book: HashMap::new(),
            log_backend: Some(LogType::Heap),
        }
    }
}

pub fn load_config(path: PathBuf, name: Option<String>) -> Result<Config> {
    match name {
        Some(n) => {
            let mut config_content = String::default();
            let mut file_handler = File::open(path)?;

            file_handler.read_to_string(&mut config_content)?;

            let mut configs: HashMap<String, Config> = serde_yaml::from_str(&config_content)
                .map_err(|_| anyhow!("Parse Config Error. Check your config file"))?;

            configs
                .remove(&n)
                .ok_or(anyhow!("Config `{}` not exists.", n))
        }
        None => Ok(Config::default()),
    }
}
