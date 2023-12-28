use axum_extra::extract::Query;
use serde::{Deserialize, Serialize};

use crate::lisp::{self, Media};

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    search_term: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    media: Vec<Media>,
}

// TODO: Handle errors, i am focusing firstly on making a working prototype

pub async fn search(Query(params): Query<SearchRequest>) -> String {
    let search_results = lisp::search(&params.search_term);

    serde_json::to_string(&SearchResponse {
        media: search_results,
    })
    .unwrap()
}
