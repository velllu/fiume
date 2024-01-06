use axum::extract;
use serde::Deserialize;

use crate::database::{account, get_connection};

#[derive(Deserialize)]
pub struct Account {
    username: String,
    password: String,
}

pub async fn register(extract::Json(body): extract::Json<Account>) -> String {
    let _register = account::register(&get_connection().await, &body.username, &body.password)
        .await
        .unwrap();

    String::from("TODO: Handle errors")
}
