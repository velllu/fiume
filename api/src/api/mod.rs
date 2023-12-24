use axum::extract::State;
use axum_extra::extract::Query;
use reqwest::Client;
use serde::Deserialize;

mod search;

/// This holds in immutable shared data between the routes, we have this because the
/// `reqwest` library tells us to only have one client
#[derive(Clone)]
pub struct Api {
    client: Client,
}

impl Api {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

// Search
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    search_term: String,
    sources_name: Vec<String>,
}

pub async fn search_handler(State(state): State<Api>, Query(params): Query<SearchQuery>) -> String {
    state.search(params).await
}
