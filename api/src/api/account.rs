use axum::{extract, response::Response};
use serde::Deserialize;

use crate::database::{account, get_connection};

#[derive(Deserialize)]
pub struct AccountRequest {
    username: String,
    password: String,
}

pub async fn register(extract::Json(body): extract::Json<AccountRequest>) -> Response<String> {
    let register = account::register(&get_connection().await, &body.username, &body.password).await;

    Response::builder()
        .status(if register.is_ok() { 200 } else { 400 })
        .body(if let Ok(account) = register {
            format!(
                r#"{{"status":200,"session_id":"{}"}}"#,
                account.session_id.unwrap()
            )
        } else {
            String::from(r#"{"status":400}"#)
        })
        .unwrap()
}
