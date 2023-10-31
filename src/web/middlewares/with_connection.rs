use anyhow::Error;
use sqlx::Acquire;
use sqlx_core::any::AnyConnectionBackend;
use std::sync::Arc;
use tokio::sync::Mutex;

use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use crate::web::{
    errors::{server_error, HttpError},
    types::with_connection::WithConnection,
    PogloState,
};

pub async fn with_connection<T>(
    State(state): State<PogloState>,
    mut req: Request<T>,
    next: Next<T>,
) -> Result<Response, HttpError> {
    let conn = state.pool.acquire().await.map_err(server_error).unwrap();
    let extensions = req.extensions_mut();
    let mutex = Arc::new(Mutex::new(conn));

    extensions.insert(WithConnection(Arc::clone(&mutex)));
    let response = next.run(req).await;
    if (response.status() != 200) {}
    // let finished_conn = mutex.lock().await.;

    Ok(response)
}

pub async fn with_transactioned_connection<T>(
    State(state): State<PogloState>,
    mut req: Request<T>,
    next: Next<T>,
) -> Result<Response, HttpError> {
    let mut conn = state.pool.acquire().await.map_err(server_error).unwrap();
    sqlx::query("begin").execute(&mut *conn).await?;
    // we begin the transaction
    let extensions = req.extensions_mut();
    let mutex = Arc::new(Mutex::new(conn));

    extensions.insert(WithConnection(Arc::clone(&mutex)));
    let response = next.run(req).await;
    let mut retrieved_conn = mutex.lock().await; // we retrieve the connection when the response is done
    dbg!(&response);
    if response.status() != 200 {
        // if something went bad we need to rollback everything!
        println!("rollbacking...");
        sqlx::query("rollback")
            .execute(retrieved_conn.as_mut())
            .await?;
    } else {
        println!("ok!");
        sqlx::query("commit")
            .execute(retrieved_conn.as_mut())
            .await?;
    }

    // connection is closed on drop!
    drop(retrieved_conn);

    Ok(response)
}
