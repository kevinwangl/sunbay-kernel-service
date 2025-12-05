use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub signing_key: String,
    pub cache: CacheConfig,
    pub logging: LoggingConfig,
    pub backend: BackendConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BackendConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CacheConfig {
    pub enabled: bool,
    pub max_size_mb: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let env = env::var("ENV").unwrap_or_else(|_| "development".to_string());
        let config_file = format!("config/{}.yaml", env);

        let config = config::Config::builder()
            .add_source(config::File::with_name(&config_file))
            .add_source(config::Environment::with_prefix("APP"))
            .build()?;

        Ok(config.try_deserialize()?)
    }
}
