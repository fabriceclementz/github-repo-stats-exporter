use anyhow::{Context, Result};
use log::info;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RepoStats {
    /// Number of stars
    pub stargazers_count: u64,
    /// Open issues
    pub open_issues_count: u64,
}

pub struct StatsFetcher {
    repository: String,
    client: reqwest::Client,
}

impl StatsFetcher {
    pub fn new(repo: String, client: reqwest::Client) -> Self {
        Self {
            repository: repo,
            client,
        }
    }

    pub async fn fetch(&self) -> Result<RepoStats> {
        let url = &format!("https://api.github.com/repos/{}", &self.repository);
        info!("fetching stats at: {}", url);

        let stats = self
            .client
            .get(url)
            .send()
            .await
            .with_context(|| format!("Failed to send request at {}", url))?
            .json::<RepoStats>()
            .await
            .with_context(|| format!("Failed to parse response from {}", url))?;

        info!("stats fetched: {:?}", stats);

        Ok(stats)
    }
}
