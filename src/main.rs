mod utils;
mod configuration;
mod request_clients;
mod torznab;

use std::sync::Arc;
use tracing::{debug};
use tracing_subscriber::EnvFilter;
use crate::configuration::configuration_provider::{AppConfig,ConfigurationProvider};
use crate::request_clients::initialize_request_clients;
use crate::torznab::initialize_torznab_api;

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}

#[tokio::main]
async fn main() {
    init_tracing();

    let app_config = ConfigurationProvider::load_config().expect("Failed to load configuration");

    initialize_services(&app_config);

    let router = torznab::get_torznab_api();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

fn initialize_services(app_config: &Arc<AppConfig>) {
    initialize_request_clients(app_config.clone());
    initialize_torznab_api(request_clients::get_orionoid_client().clone());
    debug!("Services initialized successfully");
}
