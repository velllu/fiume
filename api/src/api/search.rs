use axum::extract::Path;
use axum_extra::extract::Query;
use serde::{Deserialize, Serialize};

use crate::lisp::{self, Media};

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    search_term: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub media: Vec<Media>,
    pub next_state: String,
}

// TODO: Handle errors, i am focusing firstly on making a working prototype

pub async fn search(
    Query(params): Query<SearchRequest>,
    Path(source_name): Path<String>,
) -> String {
    let search_results = lisp::search(&source_name, &params.search_term);
    serde_json::to_string(&search_results).unwrap()
}
