use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum DbError {
    CureFail(String),
    ConnError(String),
    IllegalInput(String),
    SqlExecuteFail(String),
    SerializeError(String),
    BadCode(String), //this is only meant to error if there is something wrong with the code
}
