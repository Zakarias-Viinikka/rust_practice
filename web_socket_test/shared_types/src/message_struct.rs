use crate::db_error::DbError;
use serde::{Deserialize, Serialize};
/// Wrapper around Vec<u8> that serializes as a base64 string in JSON.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Base64Bytes(pub Vec<u8>);
impl Serialize for Base64Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use base64::Engine;
        let encoded = base64::engine::general_purpose::STANDARD.encode(&self.0);
        serializer.serialize_str(&encoded)
    }
}
impl<'de> Deserialize<'de> for Base64Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use base64::Engine;
        let s = String::deserialize(deserializer)?;
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(s)
            .map_err(serde::de::Error::custom)?;
        Ok(Base64Bytes(bytes))
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub message_id: usize,
    pub request: Request,
    pub content: Base64Bytes, // bincode-serialized payload, base64-encoded in JSON
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Response {
    pub message_id: usize,
    pub request: Request,
    pub result: Result<Base64Bytes, DbError>,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Request {
    CreateTable,
    ListTables,
    GetData,
    GetDataOrdered,
    InsertData,
    DropTable,
    EditColInRow,
    CheckTable,
    DeleteRow,
    SwapColumns,
    CreateIndex,
    CheckIndex,
    AddColumn,
    RemoveColumn,
    ExportDatabase,
    ExportTables,
    CreateTableFromExport,
    CopyTable,
}

pub fn message_to_json_str(msg: &Message) -> Result<String, serde_json::Error> {
    serde_json::to_string(msg)
}
pub fn json_str_to_message(json: &str) -> Result<Message, serde_json::Error> {
    serde_json::from_str(json)
}
pub fn response_to_json_str(resp: &Response) -> Result<String, serde_json::Error> {
    serde_json::to_string(resp)
}
pub fn json_str_to_response(json: &str) -> Result<Response, serde_json::Error> {
    serde_json::from_str(json)
}

pub fn i_dont_want_to(message_id: usize, request: Request) -> Vec<u8> {
    let response = Response {
        message_id,
        request,
        result: Err(DbError::BadCode(
            "server does not handle this request".to_string(),
        )),
    };

    response_to_json_str(&response)
        .expect("serializing hardcoded response should not fail")
        .into_bytes()
}
