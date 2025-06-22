use snafu::Snafu;

use crate::app_config::AppConfig;
use crate::traits::query::Query;

#[derive(Debug, Snafu)]
pub enum FuzzyQueryError {
}

pub struct FuzzyQuery {}

impl Query for FuzzyQuery {
    type Error = FuzzyQueryError;

    fn query(keyword: &str) -> Result<String, Self::Error> {
        Ok(String::from(keyword))
    }

    fn sync(config: &AppConfig) -> Result<String, Self::Error> {
        todo!()
    }

}
