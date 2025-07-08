use std::num::NonZeroU32;
use std::sync::Arc;
use anyhow::anyhow;
use futures::future::BoxFuture;
use governor::clock::DefaultClock;
use governor::{Quota, RateLimiter};
use governor::state::NotKeyed;
use crate::request_clients::request_errors::error::Error;

#[derive(Clone, Debug)]
pub struct RateLimitedClient {
    inner: Arc<reqwest::Client>,
    limiter: Arc<RateLimiter<NotKeyed, governor::state::InMemoryState, DefaultClock>>,
}

pub trait Executor: Send + Sync {
    fn execute_raw(&self, url: &str) -> BoxFuture<Result<reqwest::Response, Error>>;
}

impl RateLimitedClient {
    pub fn from_config(user_agent: &str, rate_limit: &str) -> anyhow::Result<Self> {
        let client = Arc::new(
            reqwest::Client::builder()
                .user_agent(user_agent)
                .build()
                .map_err(|e| anyhow::anyhow!("Failed to build reqwest client: {e}"))?,
        );

        let limiter = Arc::new(RateLimiter::direct(Self::parse_quota(rate_limit)?));

        Ok(Self { inner: client, limiter })
    }

    fn parse_quota(s: &str) -> anyhow::Result<Quota> {
        let parts: Vec<&str> = s.trim().split('/').collect();
        if parts.len() != 2 {
            anyhow::bail!("Rate limit must be in format 'N/second', 'N/minute', etc.");
        }

        let amount: u32 = parts[0].parse()?;
        let nonzero = NonZeroU32::new(amount)
            .ok_or_else(|| anyhow!("Rate must be > 0"))?;

        match parts[1] {
            "second" | "sec" => Ok(Quota::per_second(nonzero)),
            "minute" | "min" => Ok(Quota::per_minute(nonzero)),
            "hour" => Ok(Quota::per_hour(nonzero)),
            other => anyhow::bail!("Invalid rate unit: {}", other),
        }
    }
}

impl Executor for RateLimitedClient {
    fn execute_raw(&self, url: &str) -> BoxFuture<Result<reqwest::Response, Error>> {
        let client = self.inner.clone();
        let limiter = self.limiter.clone();
        let url = url.to_string();

        Box::pin(async move {
            limiter.until_ready().await;
            client
                .get(&url)
                .send()
                .await
                .map_err(|e| Error::Request {
                    source: Box::new(e),
                })
        })
    }
}