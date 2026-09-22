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
pub enum QueueErr {
    LockErr(ErrorDetails),
    RecvErr(ErrorDetails),
}

impl fmt::Display for QueueErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueueErr::LockErr(d) => write!(f, "queue lock: {d}"),
            QueueErr::RecvErr(d) => write!(f, "queue wake: {d}"),
        }
    }
}

impl std::error::Error for QueueErr {}

#[macro_export]
macro_rules! unwrap_recv {
    ($e:expr, $details:expr) => {
        $e.map_err(|real_err| {
            $crate::queue_err::QueueErr::RecvErr($crate::queue_err::ErrorDetails {
                err_msg: format!("{:?}", real_err),
                file: $details.file,
                method: $details.method,
            })
        })?
    };
}
