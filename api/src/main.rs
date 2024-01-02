use crate::api::{search::search, state::state};

use axum::{routing::get, Router};
use errors::ApiError;

mod api;
mod errors;
mod lisp;

pub const SETTINGS_FILE: &str = include_str!("../settings.lisp");

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    let app = Router::new()
        .route("/search/:source", get(search))
        .route("/state/:source/:state", get(state));

    #[cfg(debug_assertions)]
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    #[cfg(not(debug_assertions))]
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();

    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
