use std::fmt;

pub struct InBetweenErr {
    pub err_msg: String,
    pub file: String,
    pub method: String,
}

impl fmt::Display for InBetweenErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}::{}: {}", self.file, self.method, self.err_msg)
    }
}
