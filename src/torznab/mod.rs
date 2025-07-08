mod types;
mod responses;
mod categories;
mod search_config;
mod limits;
mod search_handler;

use axum::extract::{State, Query};
use axum::response::IntoResponse;
use axum::{Router, routing::get};
use std::sync::Arc;
use once_cell::sync::OnceCell;
use reqwest::StatusCode;
use crate::request_clients::orionoid_client::orionoid_request_client::OrionoidRequestClient;
use crate::torznab::types::{Config, SearchFunc};

static TORZNAB_API: OnceCell<Router> = OnceCell::new();
static SEARCH_HANDLER: OnceCell<Arc<search_handler::TorznabSearchHandler>> = OnceCell::new();

async fn api_dispatch(
    State(conf): State<Arc<Config>>,
    Query(query): Query<responses::SearchForm>,
) -> impl IntoResponse {
    match query.t.as_deref() {
        Some("caps") => responses::caps(State(conf)).await.into_response(),
        Some("search") => responses::search(State(conf), Query(query)).await.into_response(),
        Some("tvsearch") => responses::tv_search(State(conf), Query(query)).await.into_response(),
        Some("movie") => responses::movie_search(State(conf), Query(query)).await.into_response(),
        _ => (StatusCode::NOT_FOUND, "Unknown or missing `t` parameter").into_response(),
    }
}

fn setup_torznab_config() -> Config {
    let search_handler: SearchFunc = Arc::new(|params| {
        Box::pin(async move {
            let handler = SEARCH_HANDLER.get().expect("Handler not initialized");
            handler.search_orionoid(params).await
        })
    });

    let caps = types::Caps {
        server_info: Some(std::collections::HashMap::from([
            ("version".to_string(), "1.0.0".to_string()),
            ("url".to_string(), "https://github.com/ipromknight/orionznab".to_string()),
            ("title".to_string(), "Orionznab by iPromKnight".to_string()),
        ])),
        limits: limits::SEARCH_LIMITS.clone(),
        searching: search_config::SEARCH_CONFIG.to_vec(),
        categories: categories::CATEGORIES.to_vec(),
        genres: None,
        tags: None,
    };

    Config {
        search_handler,
        caps,
    }
}

pub fn initialize_torznab_api(orionoid_client: Arc<OrionoidRequestClient>) {
    let search_handler = Arc::new(search_handler::TorznabSearchHandler::new(orionoid_client.clone()));
    let state_config = setup_torznab_config();

    let state = Arc::new(state_config);

    let torznab_api = Router::new()
        .route("/api", get(api_dispatch))
        .with_state(state);

    SEARCH_HANDLER
        .set(search_handler)
        .expect("Search Handler already initialized");

    TORZNAB_API
        .set(torznab_api)
        .expect("Torznab API already initialized");
}

pub fn get_torznab_api() -> Router {
    TORZNAB_API.get().expect("Torznab Api not initialized").clone()
}