use reqwest::{blocking::Client, redirect};
use crate::config::settings::ScanConfig;

pub fn build_client() -> Result<Client, anyhow::Error>{
    build_client_with_config(&ScanConfig::default())
}

pub fn build_client_with_config(config: &ScanConfig) -> Result<Client, anyhow::Error>{
    let client = Client::builder()
        .redirect(redirect::Policy::limited(config.max_redirects))
        .timeout(config.http_timeout)
        .build()?;
    Ok(client)
}
