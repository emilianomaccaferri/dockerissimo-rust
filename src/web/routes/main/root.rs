use crate::log_util::LoggableOutcome;
use async_trait::async_trait;
use axum::{
    body::HttpBody, debug_handler, extract::FromRequest, BoxError, Extension,
    Json,
};
use axum_extra::extract::WithRejection;
use serde::{de::DeserializeOwned, Deserialize};
use serde_json::{json, Value};

use crate::web::{errors::PogloError, types::with_connection::WithConnection};

#[derive(Deserialize)]
pub struct Test {
    number: i32,
}

#[debug_handler]
pub async fn handler(
    Extension(WithConnection(conn)): Extension<WithConnection>,
    Json(body): Json<Test>,
) -> Result<Json<Value>, PogloError> {
    let mut got_conn = conn.lock().await;
    let mut_conn = got_conn.as_mut();

    sqlx::query("insert into nongle values($1)")
        .bind(body.number)
        .execute(&mut *mut_conn)
        .await
        .log_err_to_error("damn")?;

    Ok(Json(
        json!({ "success": true, "message": "welcome to poglo" }),
    ))
}
