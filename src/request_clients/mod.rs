use std::sync::Arc;
use once_cell::sync::OnceCell;
use crate::configuration::configuration_provider::{AppConfig};
use crate::request_clients::orionoid_client::orionoid_request_client;
use crate::request_clients::orionoid_client::orionoid_request_client::OrionoidRequestClient;
use crate::request_clients::rate_limited_client::RateLimitedClient;
pub mod orionoid_client;
pub mod rate_limited_client;
pub mod request_errors;

static ORIONOID_REQUEST_CLIENT: OnceCell<Arc<OrionoidRequestClient>> = OnceCell::new();

pub fn initialize_request_clients(app_config: Arc<AppConfig>) {
    initialize_orionoid_request_client(app_config.clone());
}

pub fn initialize_orionoid_request_client(app_config: Arc<AppConfig>) {
    let executor = RateLimitedClient::from_config(
        &app_config.user_agent,
        &app_config.orionoid_rate_limit,
    ).expect("Failed to create Orionoid executor");

    let inner_client = orionoid_request_client::ClientBuilder::default()
        .with_executor(executor)
        .build()
        .expect("Failed to build Orionoid request client");

    let client = OrionoidRequestClient(inner_client);

    ORIONOID_REQUEST_CLIENT
        .set(Arc::new(client))
        .expect("Orionoid request client already initialized");
}
pub fn get_orionoid_client() -> Arc<OrionoidRequestClient> {
    ORIONOID_REQUEST_CLIENT.get().expect("Orionoid Client not initialized").clone()
}