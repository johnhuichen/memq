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

#[derive(Debug, Serialize, Deserialize)]
pub struct Doc {
    path: String,
    is_indexed: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfig {
    docs: Vec<Doc>,
}

impl AppConfig {
    pub fn load() -> Result<Self, AppConfigError> {
        let cfg: AppConfig = confy::load(APP_NAME, None).context(LoadConfigSnafu)?;

        Ok(cfg)
    }

    pub fn print_docs(&self) -> Result<(), AppConfigError> {
        if self.docs.is_empty() {
            println!("There are currently no paths in the watchlist");
        }
        for doc in &self.docs {
            println!("{:?}", doc);
        }
        Ok(())
    }

    pub fn add_docs(&mut self, new_paths: &[String]) -> Result<(), AppConfigError> {
        let doc_paths: Vec<String> = self.docs.iter().map(|doc| doc.path.clone()).collect();

        for path in new_paths {
            if doc_paths.contains(path) {
                info!("{} is already in the list", path);
            } else if !Path::new(path).exists() {
                println!("{}", format!("{} does not exist!", path).bright_red());
            } else {
                self.docs.push(Doc {
                    path: String::from(path),
                    is_indexed: false,
                });
            }
        }

        confy::store(APP_NAME, None, &self).context(SaveConfigSnafu)?;

        Ok(())
    }

    pub fn remove_doc_paths(&mut self, remove_paths: &[String]) -> Result<(), AppConfigError> {
        self.docs.retain(|doc| !remove_paths.contains(&doc.path));

        confy::store(APP_NAME, None, &self).context(SaveConfigSnafu)?;

        Ok(())
    }
}
