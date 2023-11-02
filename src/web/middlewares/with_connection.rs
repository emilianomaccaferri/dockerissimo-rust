use std::sync::Arc;
use tokio::sync::Mutex;

use axum::{
    extract::State, http::Request, middleware::Next, response::Response,
};

use crate::web::{
    errors::PogloError, types::with_connection::WithConnection, PogloState,
};

pub async fn with_connection<T>(
    State(state): State<PogloState>,
    mut req: Request<T>,
    next: Next<T>,
) -> Result<Response, PogloError> {
    let conn = state.pool.acquire().await?;
    let extensions = req.extensions_mut();
    let mutex = Arc::new(Mutex::new(conn));

    extensions.insert(WithConnection(Arc::clone(&mutex)));
    let response = next.run(req).await;

    Ok(response)
}

pub async fn with_transactioned_connection<T>(
    State(state): State<PogloState>,
    mut req: Request<T>,
    next: Next<T>,
) -> Result<Response, PogloError> {
    let mut conn = state.pool.acquire().await?;
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
