use axum::{routing::get, Router};
use errors::ApiError;

use crate::api::{search_handler, Api};

mod api;
mod errors;
mod parsing;

pub const SETTINGS_FILE: &str = include_str!("../../settings.json");

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    let app = Router::new()
        .route("/search", get(search_handler))
        .with_state(Api::new());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
