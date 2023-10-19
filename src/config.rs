use config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u32,
}

impl ServerConfig {
    pub fn server_url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Deserialize, Clone)]

pub struct AppConfig {
    pub server: ServerConfig,
    pub access_secret: String,
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let cfg =
            config::Config::builder().add_source(config::Environment::default().separator("."));
        let config = cfg.build()?;
        let config = config.try_deserialize()?;
        Ok(config)
    }
}
