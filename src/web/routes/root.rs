use axum::Json;
use serde_json::{json, Value};

pub async fn index() -> Json<Value> {
    Json(json!({ "success": true, "message": "welcome to poglo" }))
}
