use std::path::Path;

use confy::ConfyError;
use log::info;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};

pub static APP_NAME: &str = "memq";

#[derive(Debug, Snafu)]
pub enum AppConfigError {
    #[snafu(display("Path does not exist: {}", path))]
    InvalidPath { path: String },

    #[snafu(display("Failed to load config: {}", source))]
    LoadConfig { source: ConfyError },

    #[snafu(display("Failed to save config: {}", source))]
    SaveConfig { source: ConfyError },
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfig {
    paths: Vec<String>,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfyError> {
        let cfg: AppConfig = confy::load(APP_NAME, None)?;
        info!("Config {:#?}", cfg);

        Ok(cfg)
    }

    // pub fn get_paths(&self) -> Vec<String> {
    //     self.paths.clone()
    // }

    pub fn add_paths(new_paths: &[String]) -> Result<(), AppConfigError> {
        if !Self::validate_paths(new_paths) {
            return Err(AppConfigError::InvalidPath);
        }

        let mut cfg = Self::load().context(LoadConfigSnafu)?;

        for path in new_paths {
            if !cfg.paths.contains(path) {
                cfg.paths.push(path.clone());
            }
        }

        confy::store(APP_NAME, None, &cfg).context(SaveConfigSnafu)?;

        Ok(())
    }

    fn validate_paths(paths: &[String]) -> bool {
        paths.iter().map(Path::new).all(|p| p.exists())
    }
}
