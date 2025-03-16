use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: ServerSettings,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        
        let s = Config::builder()
            .add_source(File::with_name("config/default").required(true))
            .add_source(File::with_name(&format!("config/{}", run_mode)).required(false))
            .build()?;
            
        s.try_deserialize()
    }
}