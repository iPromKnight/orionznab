use std::borrow::Cow;
use anyhow::{anyhow, Result};
use crate::request_clients::orionoid_client::search_endpoints::SearchService;
use crate::request_clients::rate_limited_client::{Executor, RateLimitedClient};
use crate::request_clients::request_errors::error::Error;

#[derive(Debug)]
pub struct OrionoidRequestClient(
    pub Client<RateLimitedClient>
);

const BASE_URL: &str = "https://api.orionoid.com";

pub struct ClientBuilder<E: Executor> {
    base_url: Cow<'static, str>,
    executor: Option<E>,
}

impl<E: Executor> Default for ClientBuilder<E> {
    fn default() -> Self {
        Self {
            base_url: Cow::Borrowed(BASE_URL),
            executor: None,
        }
    }
}

impl<E: Executor> ClientBuilder<E> {
    pub fn with_executor(mut self, executor: E) -> Self {
        self.executor = Some(executor);
        self
    }

    pub fn build(self) -> Result<Client<E>> {
        let base_url = self.base_url;
        let executor = self.executor.ok_or_else(|| anyhow!("missing executor"))?;

        Ok(Client {
            executor,
            base_url
        })
    }
}

pub struct Client<E> {
    executor: E,
    base_url: Cow<'static, str>,
}

impl<E: std::fmt::Debug> std::fmt::Debug for Client<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(Client))
            .field("executor", &self.executor)
            .field("base_url", &self.base_url)
            .finish()
    }
}

impl OrionoidRequestClient {
    pub async fn execute_raw<P>(&self, path: &str) -> Result<reqwest::Response, Error>
    where
        P: serde::Serialize + Send + Sync + 'static,
    {
        let url = format!("{}{}", self.0.base_url, path);
        self.0.executor
            .execute_raw(&url)
            .await
    }

    pub fn search_endpoints(&self) -> SearchService<'_> { SearchService { client: self } }
}