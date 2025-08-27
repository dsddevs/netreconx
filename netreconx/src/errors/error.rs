use thiserror::*;
#[derive(Debug, Error, Clone)]
pub enum Errors {
    #[error("Usage: cargo run -- <domain>")]
    CliErr,
    #[error("Reqwest: {0}")]
    ReqwestErr(String),
    #[error("Invalid domain format: {0}")]
    InvalidDomain(String),
}
impl From<reqwest::Error> for Errors {
    fn from(err: reqwest::Error) -> Self {
        Errors::ReqwestErr(err.to_string())
    }
}