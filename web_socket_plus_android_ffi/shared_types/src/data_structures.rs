use crate::create_table;
use crate::db_error::DbError;
use crate::table_row;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTableIn {
    pub table_name: String,
    pub columns: Vec<create_table::ColumnDef>,
}

#[derive(Serialize, Deserialize, Debug, uniffi::Record)]
pub struct CreateTableOut {
    //this used to be Result<(), DbError>
    // but uniffi didn't like that so i changed it to option
    pub result: Option<DbError>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListTablesOut {
    pub table_names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Enum)]
pub enum SelectArgument {
    XEqualY { x: String, y: String },
    XNotEqualY { x: String, y: String },
    XGreaterThanY { x: String, y: String },
    XLessThanY { x: String, y: String },
    XGreaterThanOrEqualY { x: String, y: String },
    XLessThanOrEqualY { x: String, y: String },
    XLikeY { x: String, y: String },
    XInY { x: String, y: Vec<String> },
    All,
}

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct GetDataIn {
    pub table_name: String,
    pub arguments: Vec<SelectArgument>,
    pub columns_to_read: Vec<String>,
}

impl SelectArgument {
    pub fn to_sql_condition(&self) -> String {
        fn quote_value(value: &str) -> String {
            format!("'{}'", value.replace('\'', "''"))
        }
        fn quote_ident(ident: &str) -> String {
            format!("\"{}\"", ident.replace('"', "\"\""))
        }

        match self {
            SelectArgument::XEqualY { x, y } => format!("{} = {}", quote_ident(x), quote_value(y)),
            SelectArgument::XNotEqualY { x, y } => {
                format!("{} != {}", quote_ident(x), quote_value(y))
            }
            SelectArgument::XGreaterThanY { x, y } => {
                format!("{} > {}", quote_ident(x), quote_value(y))
            }
            SelectArgument::XLessThanY { x, y } => {
                format!("{} < {}", quote_ident(x), quote_value(y))
            }
            SelectArgument::XGreaterThanOrEqualY { x, y } => {
                format!("{} >= {}", quote_ident(x), quote_value(y))
            }
            SelectArgument::XLessThanOrEqualY { x, y } => {
                format!("{} <= {}", quote_ident(x), quote_value(y))
            }
            SelectArgument::XLikeY { x, y } => {
                format!("{} LIKE {}", quote_ident(x), quote_value(y))
            }
            SelectArgument::XInY { x, y } => {
                let quoted_values: Vec<String> = y.iter().map(|v| quote_value(v)).collect();
                format!("{} IN ({})", quote_ident(x), quoted_values.join(", "))
            }
            SelectArgument::All => String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct GetDataOut {
    pub rows: Vec<table_row::Row>,
}

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct GetDataOrderedIn {
    pub table_name: String,
    pub arguments: Vec<SelectArgument>,
    pub columns_to_read: Vec<String>,
    pub order_by: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct ColumnValue {
    pub column_name: String,
    pub value: table_row::Col,
}

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct InsertDataIn {
    pub table_name: String,
    pub values: Vec<ColumnValue>,
}

#[derive(Serialize, Deserialize, Debug, uniffi::Record)]
pub struct InsertDataOut {
    //this used to be Result<(), DbError>
    // but uniffi didn't like that so i changed it to option
    pub result: Option<DbError>,
}

// public_data_shapes.rs
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DropTableIn {
    pub table_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct EditColInRowIn {
    pub table_name: String,
    pub row_id: String,
    pub column: String,
    pub new_value: table_row::Col,
}

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct CheckTableIn {
    pub table_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct TableColumnInfo {
    pub cid: i64,
    pub name: String,
    pub type_name: String,
    pub not_null: bool,
    pub default_value: Option<String>,
    pub primary_key: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct CheckTableOut {
    pub columns: Vec<TableColumnInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct DeleteRowIn {
    pub table_name: String,
    pub row_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct SwapColumnsIn {
    pub table_name: String,
    pub row_id_1: String,
    pub row_id_2: String,
    pub column: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct CreateIndexIn {
    pub table_name: String,
    pub column_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct CheckIndexIn {
    pub table_name: String,
    pub column_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct CheckIndexOut {
    pub is_indexed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct AddColumnIn {
    pub table_name: String,
    pub column: create_table::ColumnDef,
}

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct ExportDatabaseIn {}

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct ExportDatabaseOut {
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct RemoveColumnIn {
    pub table_name: String,
    pub column_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct ExportTablesIn {
    pub table_names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct TableExport {
    pub table_name: String,
    pub columns: Vec<TableColumnInfo>,
    pub rows: Vec<table_row::Row>,
}

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct ExportTablesOut {
    pub tables: Vec<TableExport>,
}

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct CreateTableFromExportIn {
    pub table_name: String,
    pub table: TableExport,
}

#[derive(Serialize, Deserialize, Debug, Clone, uniffi::Record)]
pub struct CopyTableIn {
    pub source_table_name: String,
    pub new_table_name: String,
}
