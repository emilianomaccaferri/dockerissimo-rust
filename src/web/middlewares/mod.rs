use axum::{http::{request, Request, StatusCode}, middleware::Next, response::Response, Json};
use serde_json::{json, Value};
use axum::RequestPartsExt;
use crate::web::extractors::pool_connection::WithPoolConnection;

use super::errors::server_error;

pub async fn auth_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Json<Value>, StatusCode>
where
    B: Send,
{

    let (mut parts, body) = request.into_parts();
    let conn: WithPoolConnection = parts.extract().await?.map_err(server_error);
    Ok(Json(json!({"pog": "champ"})))

}