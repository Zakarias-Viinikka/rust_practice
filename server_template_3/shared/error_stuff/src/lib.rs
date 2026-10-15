use std::fmt;

#[derive(Debug, Clone)]
pub struct ErrorDetails {
    pub err_msg: String,
    pub file: String,
    pub method: String,
}

impl fmt::Display for ErrorDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}: {}", self.file, self.method, self.err_msg)
    }
}

impl std::error::Error for ErrorDetails {}

#[derive(Debug, Clone)]
pub enum ServerRelatedError {
    ServerTalker(ErrorDetails),
    Protocol(ErrorDetails),
    ResponseRouter(ErrorDetails),
}

impl fmt::Display for ServerRelatedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerRelatedError::ServerTalker(d) => write!(f, "server_talker: {d}"),
            ServerRelatedError::Protocol(d) => write!(f, "protocol: {d}"),
            ServerRelatedError::ResponseRouter(d) => write!(f, "response_router: {d}"),
        }
    }
}

impl std::error::Error for ServerRelatedError {}
