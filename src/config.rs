use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

static APP_NAME: &str = "snitch";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SMTPInfo {
    pub smtp_user: String,
    pub smtp_password: String,
    pub smtp_server: String,
    pub recipient: String,
    pub email_subject: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    smtp_info: SMTPInfo,
    tcp_targets: HashMap<String, String>,
    proc_targets: HashMap<String, String>,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            smtp_info: SMTPInfo {
                smtp_user: String::new(),
                smtp_password: String::new(),
                smtp_server: String::new(),
                recipient: String::new(),
                email_subject: String::new(),
            },
            tcp_targets: HashMap::from([(
                String::from("www.youtube.com"),
                String::from("John is being lazy and watching Youtube"),
            )]),
            proc_targets: HashMap::from([(
                String::from("lutris"),
                String::from("John is being lazy and playing lutris"),
            )]),
        }
    }
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let cfg: Config = confy::load(APP_NAME, None)?;
        log::debug!("Config {:#?}", cfg);

        Ok(cfg)
    }

    pub fn get_tcp_targets(&self) -> HashMap<String, String> {
        self.tcp_targets.clone()
    }

    pub fn get_proc_targets(&self) -> HashMap<String, String> {
        self.proc_targets.clone()
    }

    pub fn get_smtp_info(&self) -> SMTPInfo {
        self.smtp_info.clone()
    }
}
