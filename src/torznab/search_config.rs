use std::sync::LazyLock;
use crate::torznab::types::SearchInfo;

static QUERY_FIELD: &str = "q";
static IMDBID_FIELD: &str = "imdbid";
static SEASON_FIELD: &str = "season";
static EPISODE_FIELD: &str = "ep";

pub static SEARCH_CONFIG: LazyLock<Vec<SearchInfo>> = LazyLock::new(|| {
    vec![
        SearchInfo {
            search_type: "search".to_string(),
            available: true,
            supported_params: vec![
                QUERY_FIELD.to_string()
            ],
        },

        SearchInfo {
            search_type: "movie-search".to_string(),
            available: true,
            supported_params: vec![
                QUERY_FIELD.to_string(),
                IMDBID_FIELD.to_string(),
            ],
        },

        SearchInfo {
            search_type: "tv-search".to_string(),
            available: true,
            supported_params: vec![
                QUERY_FIELD.to_string(),
                IMDBID_FIELD.to_string(),
                SEASON_FIELD.to_string(),
                EPISODE_FIELD.to_string(),
            ],
        },
    ]
});