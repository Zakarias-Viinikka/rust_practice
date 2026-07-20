use crate::db_table::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::console;

pub fn generate_create_table_sql(table: &Table) -> String {
    let mut cols = vec!["id INTEGER PRIMARY KEY AUTOINCREMENT".to_string()];
    cols.extend(
        table
            .columns
            .iter()
            .filter(|c| c.column_name != "id")
            .map(|c| format!("{} {}", c.column_name, c.column_type.as_str())),
    );

    format!(
        "CREATE TABLE IF NOT EXISTS {} ({});",
        table.table_name,
        cols.join(", ")
    )
}

pub fn generate_insert_sql(table: &Table, values: Vec<String>) -> String {
    let quoted_values: Vec<String> = values
        .iter()
        .map(|v| format!("'{}'", sanitize(v.as_ref())))
        .collect();
    format!(
        "INSERT INTO {} ({}) VALUES ({});",
        table.table_name,
        table
            .columns
            .iter()
            .map(|c| c.column_name.clone())
            .collect::<Vec<_>>()
            .join(", "),
        quoted_values.join(", ")
    )
}
pub fn generate_swap_two_values_sql(
    id1: usize,
    id2: usize,
    table_name: String,
    column_name: String,
) -> String {
    format!(
        "
    UPDATE {table}
    SET {column} = CASE
        WHEN id = {id1} THEN (SELECT {column} FROM {table} WHERE id = {id2})
        WHEN id = {id2} THEN (SELECT {column} FROM {table} WHERE id = {id1})
    END
    WHERE id IN ({id1}, {id2});
    ",
        table = table_name,
        column = column_name,
        id1 = id1,
        id2 = id2
    )
}

pub fn generate_delete_sql(id: usize, table_name: String) -> String {
    format!(
        "DELETE FROM {table} WHERE id = {id};",
        table = table_name,
        id = id
    )
}

//UPDATE users SET name = 'Bob', age = 30 WHERE id = 3;
pub fn generate_update_sql<I, K, V>(
    id: usize,
    table_name: &str,
    columns_and_new_values: I,
) -> String
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<str>, // column name can be &str or String
    V: AsRef<str>, // value can be &str or String
{
    let col_and_val = columns_and_new_values
        .into_iter()
        .map(|(col, val)| {
            let sanitized_val = sanitize(val.as_ref());
            format!("{} = '{}'", col.as_ref(), sanitized_val)
        })
        .collect::<Vec<_>>()
        .join(", ");

    format!(
        "UPDATE {table} SET {col_and_val} WHERE id = {id};",
        table = table_name,
        col_and_val = col_and_val,
        id = id
    )
}

pub fn generate_read_from_table_sql(
    table_name: impl AsRef<str>,
    arguments: Vec<String>,
    columns_to_read: Vec<String>,
) -> String {
    //SELECT col1, col2 FROM table_name WHERE id = 1;
    // SELECT  FROM content WHERE ;
    let arguments = if arguments.is_empty() || arguments == [""] {
        String::new()
    } else {
        //response = await askWorker(["get_data", "content", "", [""]]);
        if arguments.len() == 1 && &arguments[0] == "" {
            "".to_string()
        } else {
            format!(" WHERE {}", arguments.join(" AND "))
        }
    };
    let columns = if columns_to_read.len() == 1 && &columns_to_read[0] == "" {
        "*".to_string()
    } else {
        columns_to_read.join(", ")
    };
    /*console::log_1(&JsValue::from(format!(
        "SELECT {cols} FROM {table}{args};",
        cols = columns,
        table = table_name.as_ref(),
        args = arguments,
    )));*/
    format!(
        "SELECT {cols} FROM {table}{args};",
        cols = columns,
        table = table_name.as_ref(),
        args = arguments,
    )
}

fn sanitize(input: &str) -> String {
    input.replace("'", "''")
}

/*
enum HappySql { SanitizedSqlInput(String)) }

together with a method i would have like fn sanitize_userinput_to_sql(input: String) -> SanitizedSqlInput(String)) {}
 */

//wasm-pack test --headless --chrome
#[cfg(test)]
mod tests {
    use super::*;
    //use wasm_bindgen_test::*;

    #[test]
    pub fn test_generate_create_table_sql() {
        let table = Table {
            table_name: "employees".to_string(),
            columns: vec![
                Column {
                    column_name: "emp_id".to_string(),
                    column_type: ColumnType::Integer,
                },
                Column {
                    column_name: "first_name".to_string(),
                    column_type: ColumnType::Text,
                },
                Column {
                    column_name: "salary".to_string(),
                    column_type: ColumnType::Real,
                },
            ],
        };

        let sql = generate_create_table_sql(&table);
        let expected = "CREATE TABLE IF NOT EXISTS employees (id INTEGER PRIMARY KEY AUTOINCREMENT, emp_id INTEGER, first_name TEXT, salary REAL);";
        assert_eq!(sql, expected);
    }

    #[test]
    pub fn test_generate_insert_sql() {
        let table = Table {
            table_name: "products".to_string(),
            columns: vec![
                Column {
                    column_name: "product_id".to_string(),
                    column_type: ColumnType::Integer,
                },
                Column {
                    column_name: "product_name".to_string(),
                    column_type: ColumnType::Text,
                },
            ],
        };
        let values = vec!["100".to_string(), "Laptop".to_string()];
        let sql = generate_insert_sql(&table, values);
        let expected = "INSERT INTO products (product_id, product_name) VALUES ('100', 'Laptop');";
        assert_eq!(sql, expected);
    }

    #[test]
    pub fn test_generate_swap_two_values_sql() {
        let sql = generate_swap_two_values_sql(5, 10, "inventory".to_string(), "stock".to_string());

        assert!(sql.contains("UPDATE inventory"));
        assert!(sql.contains("SET stock = CASE"));
        assert!(sql.contains("WHEN id = 5 THEN (SELECT stock FROM inventory WHERE id = 10)"));
        assert!(sql.contains("WHEN id = 10 THEN (SELECT stock FROM inventory WHERE id = 5)"));
        assert!(sql.contains("WHERE id IN (5, 10)"));
    }

    #[test]
    pub fn test_generate_delete_sql() {
        let sql = generate_delete_sql(42, "orders".to_string());
        let expected = "DELETE FROM orders WHERE id = 42;";
        assert_eq!(sql, expected);
    }

    //pub fn generate_update_sql<I, K, V>(id: usize, table_name: &str, columns_and_new_values: I) -> String
    #[test]
    pub fn test_generate_update_sql_single() {
        let sql = generate_update_sql(5, "employees", vec![("name", "Alice")]);
        let expected = "UPDATE employees SET name = 'Alice' WHERE id = 5;";
        assert_eq!(sql, expected);
    }

    #[test]
    pub fn test_generate_update_sql_multiple() {
        let sql = generate_update_sql(10, "products", vec![("price", "99"), ("stock", "5")]);
        let expected = "UPDATE products SET price = '99', stock = '5' WHERE id = 10;";
        assert_eq!(sql, expected);
    }

    #[test]
    pub fn test_generate_update_sql_escapes_quotes() {
        let sql = generate_update_sql(7, "authors", vec![("name", "O'Reilly")]);
        let expected = "UPDATE authors SET name = 'O''Reilly' WHERE id = 7;";
        assert_eq!(sql, expected);
    }

    #[test]
    pub fn test_sanitize() {
        // No quotes → unchanged
        assert_eq!(sanitize("hello"), "hello");
        // Single quote → doubled
        assert_eq!(sanitize("O'Reilly"), "O''Reilly");
        // Multiple quotes → all doubled
        assert_eq!(sanitize("a'b'c"), "a''b''c");
        // Empty string → empty
        assert_eq!(sanitize(""), "");
    }

    //select from tests
    #[test]
    fn test_select_all_columns_no_conditions() {
        let sql = generate_read_from_table_sql("players", vec![], vec![]);
        assert_eq!(sql, "SELECT * FROM players;");
    }

    #[test]
    fn test_select_specific_columns_with_condition() {
        let sql = generate_read_from_table_sql(
            "games",
            vec!["result = '1-0'".to_string()],
            vec!["white".to_string(), "black".to_string()],
        );
        assert_eq!(sql, "SELECT white, black FROM games WHERE result = '1-0';");
    }

    #[test]
    fn test_multiple_conditions_multiple_columns() {
        let sql = generate_read_from_table_sql(
            "puzzles",
            vec!["rating > 2000".to_string(), "theme = 'mate'".to_string()],
            vec!["id".to_string(), "fen".to_string(), "solution".to_string()],
        );
        assert_eq!(
            sql,
            "SELECT id, fen, solution FROM puzzles WHERE rating > 2000 AND theme = 'mate';"
        );
    }

    #[test]
    fn test_empty_columns_with_condition() {
        let sql = generate_read_from_table_sql(
            "openings",
            vec!["name LIKE '%Sicilian%'".to_string()],
            vec![],
        );
        assert_eq!(sql, "SELECT * FROM openings WHERE name LIKE '%Sicilian%';");
    }

    #[test]
    fn test_no_columns_no_conditions() {
        let sql = generate_read_from_table_sql("events", vec![], vec![]);
        assert_eq!(sql, "SELECT * FROM events;");
    }

    // --- cases the current implementation already handles correctly ---

    #[test]
    fn empty_arguments_vec_omits_where() {
        let sql = generate_read_from_table_sql(
            "content",
            vec![],
            vec!["col1".to_string(), "col2".to_string()],
        );
        assert_eq!(sql, "SELECT col1, col2 FROM content;");
    }

    #[test]
    fn single_empty_string_argument_omits_where() {
        // matches the askWorker(["get_data", "content", "", [""]]) convention
        let sql =
            generate_read_from_table_sql("content", vec!["".to_string()], vec!["col1".to_string()]);
        assert_eq!(sql, "SELECT col1 FROM content;");
    }

    #[test]
    fn single_empty_string_column_becomes_star() {
        let sql = generate_read_from_table_sql(
            "content",
            vec!["id = 1".to_string()],
            vec!["".to_string()],
        );
        assert_eq!(sql, "SELECT * FROM content WHERE id = 1;");
    }

    #[test]
    fn both_empty_gives_select_star_no_where() {
        let sql =
            generate_read_from_table_sql("content", vec!["".to_string()], vec!["".to_string()]);
        assert_eq!(sql, "SELECT * FROM content;");
    }

    #[test]
    fn normal_case_still_works() {
        let sql = generate_read_from_table_sql(
            "content",
            vec!["id = 1".to_string()],
            vec!["col1".to_string(), "col2".to_string()],
        );
        assert_eq!(sql, "SELECT col1, col2 FROM content WHERE id = 1;");
    }

    // --- cases that currently FAIL: zero-length / mixed empty entries aren't filtered ---

    #[test]
    fn zero_length_columns_vec_should_be_star() {
        // The "*" branch only triggers on len() == 1 with an empty string.
        // A zero-length vec skips it and joins nothing, producing
        // "SELECT  FROM content;" instead of "SELECT * FROM content;".
        let sql = generate_read_from_table_sql("content", vec![], vec![]);
        assert_eq!(sql, "SELECT * FROM content;");
    }

    #[test]
    fn two_empty_string_arguments_should_omit_where() {
        // len() == 2, so the single-empty-string check is skipped and both
        // empty strings get joined with " AND ", producing
        // "SELECT * FROM content WHERE  AND ;".
        let sql = generate_read_from_table_sql(
            "content",
            vec!["".to_string(), "".to_string()],
            vec!["".to_string()],
        );
        assert_eq!(sql, "SELECT * FROM content;");
    }

    #[test]
    fn empty_string_mixed_with_real_argument_should_be_filtered() {
        // Currently produces "... WHERE  AND id = 1;" — dangling leading AND.
        let sql = generate_read_from_table_sql(
            "content",
            vec!["".to_string(), "id = 1".to_string()],
            vec!["col1".to_string()],
        );
        assert_eq!(sql, "SELECT col1 FROM content WHERE id = 1;");
    }

    #[test]
    fn empty_string_mixed_with_real_column_should_be_filtered() {
        // Currently produces "SELECT col1,  FROM content;" — trailing empty column.
        let sql = generate_read_from_table_sql(
            "content",
            vec!["id = 1".to_string()],
            vec!["col1".to_string(), "".to_string()],
        );
        assert_eq!(sql, "SELECT col1 FROM content WHERE id = 1;");
    }
}
//cargo test create_sql_statements::
