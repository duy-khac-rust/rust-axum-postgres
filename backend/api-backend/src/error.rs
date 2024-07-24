// use axum::{http::StatusCode, response::IntoResponse, Json};
// use serde_json::json;
// use thiserror::Error;

// pub type ResultUser<T> = std::result::Result<T, ErrorUser>;

// #[derive(Debug, Error)]
// pub enum ErrorUser {
//     #[error("Sqlx error: {0}")]
//     SqlxError(#[from] sqlx::Error),

//     #[error("Not found")]
//     NotFound,
// }

// impl IntoResponse for ErrorUser {
//     fn into_response(self) -> axum::response::Response {
//         match self {
//             Self::SqlxError(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))).into_response(),
//             Self::NotFound => (StatusCode::NOT_FOUND, Json(json!({ "error": "Not found" }))).into_response(),
//         }
//     }
// }
// pub type Result<T> = core::result::Result<T, String>;
