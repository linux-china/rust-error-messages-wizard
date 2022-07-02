use thiserror::Error as ThisError;

/// errors for config component: app-100
#[derive(ThisError, Debug)]
pub enum ConfigError {
    #[error("APP-100404: Failed to read config file(Config File NotFound): can not find file {0} --- Solutions: 1. check config exits or not")]
    NotFound(String),
    #[error("APP-100422: Failed to parse illegal json file(Invalid JSON Format): can not parse json file {0}  --- Solutions: 1. check format of json file")]
    Invalid(String),
}

/// errors for account component: APP-101
#[derive(ThisError, Debug)]
pub enum AccountError {
    #[error("APP-101404: Failed to find account(Account Not Found): can not find account with {0} --- Solutions: 1. check account ID")]
    NotFound(String),
    #[error("APP-101400: Failed to find blocked account(Account Blocked): {0}: can not fetch blocked account with {0} --- Solutions: 1. check account ID")]
    Blocked(String),
    #[error("APP-101500: unknown internal error")]
    Unknown,
}

#[derive(ThisError, Debug)]
pub enum DataStoreError {
    #[error("APP-120101: DataStore disconnected(DataStore Disconnected): can not connect to DataStore --- Solutions: 1. check DataStore connection")]
    Disconnect(#[from] std::io::Error),
    #[error("APP-120404: Failed to fetch KV value(Value Not Found): value for key `{0}` not found --- Solutions: 1. check key correct or not")]
    NotFound(String),
    #[error("APP-120401: Invalid header(Header Error): expected {expected:?}, found {found:?}")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("APP-120500: Unknown data store error(Internal Error)")]
    Unknown,
}
