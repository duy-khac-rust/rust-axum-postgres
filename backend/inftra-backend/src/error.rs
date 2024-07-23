pub type ResultInftra<T> = std::result::Result<T, ErrorInftra>;

#[derive(Debug)]
pub enum ErrorInftra {
    ErrorSqlx(sqlx::Error),
    ErrorCustom,
}

impl std::fmt::Display for ErrorInftra {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<sqlx::Error> for ErrorInftra {
    fn from(value: sqlx::Error) -> Self {
        Self::ErrorSqlx(value)
    }
}


impl std::error::Error for ErrorInftra {}
