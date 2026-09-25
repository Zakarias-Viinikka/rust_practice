pub struct GetDataIn {
    pub table_name: String,
    pub columns_to_read: Vec<String>,
}

pub struct GetDataOut {
    pub rows: Vec<protocol::row_col::Row>,
}

// --- //
/*

    input has 0 type guarantees.
    output returns a Col, but it doesn't say of what type.

    however the layer between for parsing cols can be written so the schema
    + layer guarantee type safety in terms of what the caller sees

*/

/*
* #[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct GetDataIn {
    pub table_name: String,
    pub arguments: SelectArguments,
    pub columns_to_read: Vec<String>,
}
*/
