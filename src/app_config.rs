use std::path::Path;

use colored::Colorize;
use confy::ConfyError;
use log::info;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};

pub static APP_NAME: &str = "memq";

#[derive(Debug, Snafu)]
pub enum AppConfigError {
    #[snafu(display("Failed to load config: {}", source))]
    LoadConfig { source: ConfyError },

    #[snafu(display("Failed to save config: {}", source))]
    SaveConfig { source: ConfyError },
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfig {
    doc_paths: Vec<String>,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfyError> {
        let cfg: AppConfig = confy::load(APP_NAME, None)?;

        Ok(cfg)
    }

    pub fn show_doc_paths() -> Result<(), AppConfigError> {
        let cfg = Self::load().context(LoadConfigSnafu)?;
        if cfg.doc_paths.is_empty() {
            println!("There are currently no paths in the watchlist");
        }
        for path in cfg.doc_paths {
            println!("{}", path);
        }
        Ok(())
    }

    pub fn add_doc_paths(new_paths: &[String]) -> Result<(), AppConfigError> {
        let mut cfg = Self::load().context(LoadConfigSnafu)?;

        for path in new_paths {
            if cfg.doc_paths.contains(path) {
                info!("{} is already in the list", path);
            } else if !Path::new(path).exists() {
                println!("{}", format!("{} does not exist!", path).bright_red());
            } else {
                cfg.doc_paths.push(path.clone());
            }
        }

        confy::store(APP_NAME, None, &cfg).context(SaveConfigSnafu)?;

        Ok(())
    }

    pub fn remove_doc_paths(remove_paths: &[String]) -> Result<(), AppConfigError> {
        let mut cfg = Self::load().context(LoadConfigSnafu)?;

        cfg.doc_paths.retain(|path| !remove_paths.contains(path));

        confy::store(APP_NAME, None, &cfg).context(SaveConfigSnafu)?;

        Ok(())
    }
}
