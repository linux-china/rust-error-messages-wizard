use error_stack::{Report, Result};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub struct AnyhowError {
    #[source]  // optional if field name is `source`
    source: anyhow::Error,
}

impl std::fmt::Display for AnyhowError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(format!("{:?}", self.source).as_str())
    }
}

pub trait AnyhowIntoReport: Sized {
    /// Type of the [`Ok`] value in the [`Result`]
    type Ok;

    /// Type of the resulting [`Err`] variant wrapped inside a [`Report<E>`].
    type Err;

    /// Converts the [`Err`] variant of the [`Result`] to a [`Report`]
    fn report(self) -> Result<Self::Ok, Self::Err>;
}

impl<T> AnyhowIntoReport for anyhow::Result<T> {
    type Ok = T;
    type Err = AnyhowError;

    #[track_caller]
    fn report(self) -> Result<T, AnyhowError> {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err(Report::from(AnyhowError { source: error })),
        }
    }
}
