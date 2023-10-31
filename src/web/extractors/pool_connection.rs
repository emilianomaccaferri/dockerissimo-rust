use std::sync::Arc;

use async_trait::async_trait;
use axum::{extract::{FromRequestParts, FromRef}, http::{StatusCode, request::Parts}, Json};
use serde_json::Value;
use sqlx::{pool, Postgres};

use crate::web::{PogloState, errors::server_error};

pub struct WithPoolConnection(pool::PoolConnection<Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for WithPoolConnection 
    where
        Arc<PogloState>: FromRef<S>,
        S: Send + Sync 
{
    type Rejection = (StatusCode, Json<Value>);
    async fn from_request_parts(_: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        
        let get_state = Arc::from_ref(state);
        let conn = get_state.pool.acquire()
            .await
            .map_err(server_error);
        
        Ok(WithPoolConnection(conn.unwrap()))

    }
}