use axum::{Extension, Json};
use serde_json::{json, Value};
use sqlx::{Executor, Postgres};
use sqlx_core::any::AnyConnectionBackend;

use crate::web::{errors::HttpError, types::with_connection::WithConnection};

#[derive(sqlx::FromRow, Debug)]
struct Test {
    number: i32,
}

pub async fn index(
    Extension(conn): Extension<WithConnection>,
) -> Result<Json<Value>, HttpError> {
    let mut got_conn = conn.0.lock().await;
    let mut_conn = got_conn.as_mut();

    sqlx::query("insert into nongle values(15)")
        .execute(&mut *mut_conn)
        .await?;
    let result = sqlx::query_as::<Postgres, Test>("select * from nongle")
        .fetch_all(&mut *mut_conn)
        .await?;

    dbg!(result);
    // try_thing()?;

    Ok(Json(
        json!({ "success": true, "message": "welcome to poglo" }),
    ))
}

fn try_thing() -> Result<(), anyhow::Error> {
    anyhow::bail!("it failed!")
}
