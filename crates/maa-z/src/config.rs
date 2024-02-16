use std::{fs::read_to_string, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::MaaResult;

use self::start_up::StartUpConfig;

pub mod start_up;

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub start_up: StartUpConfig,
}

pub struct ConfigHolder {
    config: Config,
    config_file: PathBuf,
}

impl ConfigHolder {
    pub fn new(config_file: PathBuf) -> MaaResult<Self> {
        if !std::path::Path::new(&config_file).exists() {
            tracing::info!(
                "Config file does not exist, creating a new one at {:?}",
                config_file
            );
            let default_config = Config::default();
            let file_content = toml::to_string(&default_config)?;
            std::fs::write(&config_file, file_content)?;

            return Ok(Self {
                config: default_config,
                config_file,
            });
        }

        let file_content = read_to_string(&config_file)?;
        let config = toml::from_str(&file_content)?;

        Ok(Self {
            config,
            config_file,
        })
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn write<F>(&mut self, f: F) -> MaaResult<()>
    where
        F: FnOnce(&mut Config),
    {
        f(&mut self.config);
        let file_content = toml::to_string(&self.config)?;
        std::fs::write(&self.config_file, file_content)?;

        Ok(())
    }
}
