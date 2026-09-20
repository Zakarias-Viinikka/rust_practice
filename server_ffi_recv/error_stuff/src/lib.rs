use protocol::error::DbError;
use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ServErr {
    #[error("db error: {0}")]
    DbError(DbError),
    #[error("{0}")]
    ServerError(ErrorDetails),
}

#[derive(Debug, Clone)]
pub struct ErrorDetails {
    pub err_msg: String,
    pub file: String,
    pub method: String,
}

impl fmt::Display for ErrorDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "server error in {}::{}: {}",
            self.file, self.method, self.err_msg
        )
    }
}
