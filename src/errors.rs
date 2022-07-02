use thiserror::Error as ThisError;

/// errors for config component: app-100
#[derive(ThisError, Debug)]
pub enum ConfigError {
    #[error("APP-100404: config file not found: {0}")]
    NotFound(String),
    #[error("APP-100422: invalid JSON Format: {0}")]
    Invalid(String),
}

/// errors for account component: APP-101
#[derive(ThisError, Debug)]
pub enum AccountError {
    #[error("APP-101404: account not found: {0}")]
    NotFound(String),
    #[error("APP-101400: account blocked: {0}")]
    Blocked(String),
    #[error("APP-101500: unknown internal error")]
    Unknown,
}
