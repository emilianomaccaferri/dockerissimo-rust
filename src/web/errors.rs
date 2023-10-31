use axum::{http::StatusCode, Json};
use serde_json::{Value, json};

pub fn server_error<E>(err: E) -> (StatusCode, Json<Value>)
where
    E: std::error::Error,
{
    (
        StatusCode::INTERNAL_SERVER_ERROR, 
        Json(json!(
                { "success": "false", "error": "fatal-error" }
            )
        )
    )
}