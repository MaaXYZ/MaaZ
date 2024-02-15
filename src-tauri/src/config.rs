use std::{collections::HashMap, path::PathBuf};

use serde_json::Value;

use crate::error::MaaResult;

pub type Config = HashMap<String, Value>;

pub struct ConfigHolder {
    config: Config,
    path: PathBuf,
}

impl ConfigHolder {
    pub fn new(config_file: PathBuf) -> MaaResult<Self> {
        if !config_file.exists() {
            let default_config = Config::new();
            let default_config_json = serde_json::to_string_pretty(&default_config)?;
            std::fs::write(&config_file, default_config_json)?;

            return Ok(Self {
                config: default_config,
                path: config_file,
            });
        }

        let config_str = std::fs::read_to_string(&config_file)?;
        let config: Config = serde_json::from_str(&config_str)?;

        Ok(Self {
            config,
            path: config_file,
        })
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
