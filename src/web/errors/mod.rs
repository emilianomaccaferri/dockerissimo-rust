use std::collections::HashMap;

use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use validator::{ValidationErrors, ValidationErrorsKind};

pub enum PogloError {
    DbError(sqlx::Error),
    ParsingError(String, StatusCode),
    InvalidFieldsError(HashMap<&'static str, ValidationErrorsKind>),
    Simple(StatusCode, String),
}

impl IntoResponse for PogloError {
    fn into_response(self) -> Response {
        let tuple_response = match self {
            PogloError::ParsingError(text, _) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"success": false, "error": text})),
            ),
            PogloError::DbError(err) => {
                // log db error
                let mut status = StatusCode::INTERNAL_SERVER_ERROR;
                let err_string = match err {
                    sqlx_core::Error::Database(db_err) => {
                        if let Some(code) = db_err.code() {
                            if code == "23505" {
                                status = StatusCode::CONFLICT;
                                "duplicate_row".to_owned()
                            } else {
                                format!("err_code_{}", code)
                            }
                        } else {
                            "generic_db_error".to_owned()
                        }
                    }
                    _ => "internal_server_error".to_owned(),
                };
                (
                    status,
                    Json(json!({"success": false, "error": err_string })),
                )
            }
            PogloError::InvalidFieldsError(err) => {
                let invalid_fields: Vec<&str> =
                    err.into_keys().map(|i| i).collect();
                (
                    StatusCode::BAD_REQUEST,
                    Json(
                        json!({"success": false, "error": "invalid_fields", "fields": invalid_fields}),
                    ),
                )
            }
            PogloError::Simple(code, msg) => {
                (code, Json(json!({ "success": false, "error": msg })))
            }
        };

        tuple_response.into_response()
    }
}

impl From<sqlx::Error> for PogloError {
    fn from(err: sqlx::Error) -> Self {
        PogloError::DbError(err)
    }
}

impl From<JsonRejection> for PogloError {
    fn from(err: JsonRejection) -> Self {
        PogloError::ParsingError("invalid_body".to_owned(), err.status())
    }
}

impl From<ValidationErrors> for PogloError {
    fn from(err: ValidationErrors) -> Self {
        Self::InvalidFieldsError(err.into_errors())
    }
}
