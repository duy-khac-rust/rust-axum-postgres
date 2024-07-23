pub type ResultCore<T> = std::result::Result<T, ErrorCore>;

#[derive(Debug)]
pub enum ErrorCore {
    ErrorConfig(config::ConfigError),
}

impl std::fmt::Display for ErrorCore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<config::ConfigError> for ErrorCore {
    fn from(value: config::ConfigError) -> Self {
        Self::ErrorConfig(value)
    }
}

impl std::error::Error for ErrorCore {}
