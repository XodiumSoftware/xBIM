/*
 * Copyright (c) 2025. Xodium.
 * All rights reserved.
 */

use serde::{Deserialize, Serialize};
use std::{env, error, fs, io};

/// Configuration settings for the application.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub server_port: u16,
    pub database_url: String,
    pub database_username: String,
    pub database_password: String,
    pub api_key: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_port: 8001,
            database_url: "db.xodium.org".to_string(),
            database_username: String::new(),
            database_password: String::new(),
            api_key: String::new(),
        }
    }
}

impl Config {
    pub fn init() -> Result<Self, Box<dyn error::Error>> {
        let config_path = env::current_exe()?
            .parent()
            .ok_or("Failed to get executable directory")?
            .join("config.toml");

        match fs::read_to_string(&config_path) {
            Ok(content) => Ok(toml::from_str(&content)?),
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                let config = Self::default();
                fs::write(&config_path, toml::to_string_pretty(&config)?)?;
                println!("Created new config file at: {}", config_path.display());
                Ok(config)
            }
            Err(e) => Err(e.into()),
        }
    }
}
