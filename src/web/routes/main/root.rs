use crate::{
    log_util::LoggableOutcome,
    web::{extractors::validate_body::ValidatedJson, PogloState},
};
use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::Acquire;
use validator::Validate;

use crate::web::errors::PogloError;

#[derive(Deserialize, Validate)]
pub struct Test {
    #[validate(range(min = 1, max = 100))]
    t: i32,
}

pub async fn handler(
    State(s): State<PogloState>, // debug handler
    ValidatedJson(body): ValidatedJson<Test>,
) -> Result<Json<Value>, PogloError> {
    let mut conn = s.pool.acquire().await?;
    let mut tx = conn.begin().await?;

    sqlx::query("insert into nongle values($1)")
        .bind(body.t)
        .execute(tx.as_mut())
        .await
        .log_err_to_error("damn")?;

    sqlx::query("update nongle set number = number + 1 where number > 10")
        .execute(tx.as_mut())
        .await?;

    tx.commit().await?;

    Ok(Json(
        json!({ "success": true, "message": "welcome to poglo" }),
    ))
}
