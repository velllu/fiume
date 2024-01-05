use axum::extract;
use serde::Deserialize;

use crate::database::{account, get_connection};

#[derive(Deserialize)]
pub struct RegisterRequest {
    username: String,
    password: String,
}

pub async fn register(extract::Json(body): extract::Json<RegisterRequest>) -> String {
    let _register = account::register(&get_connection().await, &body.username, &body.password)
        .await
        .unwrap();

    String::from("TODO: Handle errors")
}
