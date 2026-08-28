use std::{env, sync::LazyLock};

use serde::Deserialize;

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| Config::new());

#[derive(Deserialize, Debug)]
pub struct Config {
    name: String,
    host: String,
    port: u64,
    database_url: String,
    database_pool_maxsize: usize,
}

impl Config {
    pub fn new() -> Self {
        Self {
            name: env::var("APP_NAME").unwrap_or(String::from("authy")),
            host: env::var("APP_HOST").expect("APP_HOST must be set"),
            port: env::var("APP_PORT")
                .expect("APP_PORT must be set")
                .parse()
                .expect("Could not parse APP_PORT to number::<u64>"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            database_pool_maxsize: env::var("DATABASE_POOL_MAXSIZE")
                .unwrap_or(String::from("8"))
                .parse()
                .unwrap(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u64 {
        self.port
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    pub fn database_pool_maxsize(&self) -> usize {
        self.database_pool_maxsize
    }
}
