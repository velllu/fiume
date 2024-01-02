use axum::extract::Path;
use axum_extra::extract::Query;
use serde::Deserialize;

use crate::lisp::{self};

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    search_term: String,
}

// TODO: Handle errors, i am focusing firstly on making a working prototype

pub async fn search(Query(params): Query<SearchRequest>, Path(source): Path<String>) -> String {
    let search_results = lisp::search(&source, &params.search_term);
    serde_json::to_string(&search_results).unwrap()
}
