use axum::extract::Path;
use axum_extra::extract::Query;
use serde::Deserialize;

use crate::lisp::{self, State};

#[derive(Debug, Deserialize)]
pub struct StateRequest {
    link: String,
}

pub async fn state(
    Query(params): Query<StateRequest>,
    Path((source, state)): Path<(String, String)>,
) -> String {
    let state_results = lisp::state(&source, &params.link, &state);

    match state_results {
        State::Options(media_and_state) => serde_json::to_string(&media_and_state).unwrap(),
        State::Video(link) => format!(r#"{{"link": {}}}"#, serde_json::to_string(&link).unwrap()),
    }
}
