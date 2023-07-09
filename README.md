Rust Error Messages Wizard - Write Good Error Message
==============

# Features

* errors defined by `thiserror`
* `error-stack` friendly
* Convert `anyhow::Result` to `error-stack` Report

# Error Code Design

* Create component error enum as following:

```rust
use thiserror::Error as ThisError;

/// errors for config component: app-100
#[allow(dead_code)]
#[derive(ThisError, Debug)]
pub enum ConfigError {
    #[error("APP-100404: config file not found: {0}")]
    NotFound(String),
    #[error("APP-100422: invalid JSON Format: {0}")]
    Invalid(String),
}
```

* Use `error-stack` to handle errors:

```rust
use error_stack::{IntoReport, Result, ResultExt, report};

fn parse_config() -> Result<ConfigMap, ConfigError> {
    let json_file = "config.json";
    let config = std::fs::read_to_string(json_file)
        .into_report()
        .change_context(ConfigError::NotFound(json_file.to_string()))?;
    let map: ConfigMap = serde_json::from_str(&config)
        .into_report()
        .change_context(ConfigError::Invalid(json_file.to_string()))?;
    Ok(map)
}
```

# References

* thiserror: https://github.com/dtolnay/thiserror
* error-stack: https://github.com/hashintel/hash/tree/main/packages/libs/error-stack
* Java Error Messages Wizard: https://github.com/linux-china/java-error-messages-wizard
* How To Wrap Your Errors With Enums When Using Error-Stack: https://betterprogramming.pub/how-to-wrap-your-errors-with-enums-when-using-error-stack-77b122016e6e
