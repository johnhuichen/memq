use crate::app_config::AppConfig;

pub trait Query {
    type Error;

    fn query(keyword: &str) -> Result<String, Self::Error>;

    fn sync(config: &AppConfig) -> Result<String, Self::Error>;
}
