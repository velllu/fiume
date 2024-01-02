use axum::extract::Path;
use axum_extra::extract::Query;
use serde::{Deserialize, Serialize};

use crate::lisp::{self};

#[derive(Debug, Deserialize)]
pub struct StateRequest {
    link: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateResponse;

pub async fn state(
    Query(params): Query<StateRequest>,
    Path((source, state)): Path<(String, String)>,
) -> String {
    let state_results = lisp::state(&source, &params.link, &state);
    serde_json::to_string(&state_results).unwrap()
}
