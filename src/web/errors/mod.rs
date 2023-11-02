use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub enum PogloError {
    DbError(sqlx::Error),
}

impl IntoResponse for PogloError {
    fn into_response(self) -> Response {
        let tuple_response = match self {
            PogloError::DbError(err) => {
                // log db error
                let err_string = match err {
                    sqlx_core::Error::Database(db_err) => {
                        if let Some(err_code) = db_err.code() {
                            err_code.to_string()
                        } else {
                            "generic_db_error".to_owned()
                        }
                    }
                    _ => "internal_server_error".to_owned(),
                };
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"success": false, "error": err_string })),
                )
            }
        };

        tuple_response.into_response()
    }
}

impl From<sqlx::Error> for PogloError {
    fn from(inner: sqlx::Error) -> Self {
        PogloError::DbError(inner)
    }
}
