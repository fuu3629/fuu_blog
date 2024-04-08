use config::{self, ConfigError};
use dotenvy::dotenv;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
    pub app_key: String,
    pub qiita_api_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();
        Ok(Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            app_key: env::var("APP_KEY").expect("APP_KEY must be set"),
            qiita_api_key: env::var("QIITA_API_KEY").expect("QIITA_API must be set"),
        })
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::from_env().expect("Failed to read .env file");
}
