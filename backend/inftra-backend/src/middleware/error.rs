pub type ResultMiddleware<T> = std::result::Result<T, ErrorMiddleware>;

#[derive(Debug)]
pub enum ErrorMiddleware {
    ErrorNotFound(axum::http::StatusCode),
}

impl std::fmt::Display for ErrorMiddleware {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<axum::http::StatusCode> for ErrorMiddleware {
    fn from(_value: axum::http::StatusCode) -> Self {
        Self::ErrorNotFound(axum::http::StatusCode::NOT_FOUND)
    }
}

impl std::error::Error for ErrorMiddleware {}
