use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;

pub type ResultBase<T> = std::result::Result<T, ErrorBase>;

#[derive(Debug, Error)]
pub enum ErrorBase {
    #[error("Sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Not found")]
    NotFound,
}

impl IntoResponse for ErrorBase {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::SqlxError(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))).into_response(),
            Self::NotFound => (StatusCode::NOT_FOUND, Json(json!({ "error": "Not found" }))).into_response(),
        }
    }
}
