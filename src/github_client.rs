use std::time::Duration;

use anyhow::Result;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};

pub fn build(connect_timeout: u64, timeout: u64) -> Result<Client> {
    let mut headers = HeaderMap::new();
    headers.append(
        reqwest::header::USER_AGENT,
        HeaderValue::from_static("github-stats-fetcher"),
    );

    let client = Client::builder()
        .connect_timeout(Duration::from_secs(connect_timeout))
        .timeout(Duration::from_secs(timeout))
        .default_headers(headers)
        .build()?;

    Ok(client)
}
