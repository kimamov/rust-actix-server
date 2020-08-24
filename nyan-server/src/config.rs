use config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

/* #[derive(Deserialize)]
pub struct RustMailConfig {
    pub user: String,
    pub password: String,
} */

#[derive(Deserialize)]
pub struct AdminConfig {
    pub name: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Directory {
    pub templates: String,
    pub static_files: String,
}

/* impl Directory {
    fn new(directory_path: String) -> Directory {
        Directory {
            templates: format!("{}/templates", directory_path),
            static_files: format!("{}/static", directory_path),
        }
    }
} */

impl Default for Directory {
    fn default() -> Directory {
        Directory {
            templates: "./public/templates".to_string(),
            static_files: "./files".to_string(),
        }
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub pg: deadpool_postgres::Config,
    //pub rustmail: RustMailConfig,
    pub admin: AdminConfig,
    pub directory: Directory,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(config::Environment::new())?;
        cfg.try_into()
    }
}
