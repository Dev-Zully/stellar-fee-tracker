//! Minimal Horizon HTTP client with a configurable base URL and timeout.
use std::time::Duration;

pub struct HorizonClient {
    base_url: String,
    timeout: Duration,
    client: reqwest::Client,
}

impl HorizonClient {
    pub fn new(base_url: impl Into<String>, timeout: Duration) -> Self {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .expect("failed to build reqwest client");

        Self { base_url: base_url.into(), timeout, client }
    }

    pub async fn fee_stats_raw(&self) -> Result<String, reqwest::Error> {
        let url = format!("{}/fee_stats", self.base_url);
        self.client.get(url).timeout(self.timeout).send().await?.text().await
    }
}
