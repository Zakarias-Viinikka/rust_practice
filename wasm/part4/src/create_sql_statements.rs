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

pub fn generate_delete_sql(table_name: &str, id: &str) -> String {
    format!(
        "DELETE FROM {table} WHERE id = {id};",
        table = table_name,
        id = id
    )
}

//UPDATE users SET name = 'Bob', age = 30 WHERE id = 3;
pub fn generate_update_sql<K, V>(
    table_name: &str,
    id: usize,
    column_and_new_value: &(K, V),
) -> String
where
    K: AsRef<str>, // column name can be &str or String
    V: AsRef<str>, // value can be &str or String
{
    let col_and_val = {
        let sanitized_val = sanitize(column_and_new_value.1.as_ref());
        format!("{} = '{}'", column_and_new_value.0.as_ref(), sanitized_val)
    };

    #[cfg(all(debug_assertions, target_arch = "wasm32"))]
    {
        let tmp = format!(
            "UPDATE {table} SET {col_and_val} WHERE id = {id};",
            table = table_name,
            col_and_val = col_and_val,
            id = id
        );
        web_sys::console::log_1(&JsValue::from(tmp));
    }

    format!(
        "UPDATE {table} SET {col_and_val} WHERE id = {id};",
        table = table_name,
        col_and_val = col_and_val,
        id = id
    )
}

pub fn generate_read_from_table_sql(
    table_name: impl AsRef<str>,
    arguments: &[impl AsRef<str>],
    columns_to_read: &[impl AsRef<str>],
) -> String {
    // Collect non‑empty columns as &str slices
    let valid_columns: Vec<&str> = columns_to_read
        .iter()
        .filter(|c| !c.as_ref().is_empty())
        .map(|c| c.as_ref())
        .collect();

    let columns = if valid_columns.is_empty() {
        "*".to_string()
    } else {
        valid_columns.join(", ")
    };

    // Collect non‑empty argument conditions as &str slices
    let valid_conditions: Vec<&str> = arguments
        .iter()
        .filter(|a| !a.as_ref().is_empty())
        .map(|a| a.as_ref())
        .collect();

    let where_clause = if valid_conditions.is_empty() {
        String::new()
    } else {
        format!(" WHERE {}", valid_conditions.join(" AND "))
    };

    format!(
        "SELECT {} FROM {}{};",
        columns,
        table_name.as_ref(),
        where_clause
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

    // =========================================================================
    //  generate_create_table_sql
    // =========================================================================

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

    // =========================================================================
    //  generate_insert_sql
    // =========================================================================

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

    // =========================================================================
    //  generate_swap_two_values_sql
    // =========================================================================

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
    pub fn test_generate_swap_two_values_sql_different_ids() {
        let sql = generate_swap_two_values_sql(3, 99, "tasks".to_string(), "position".to_string());

        assert!(sql.contains("UPDATE tasks"));
        assert!(sql.contains("SET position = CASE"));
        assert!(sql.contains("WHEN id = 3 THEN (SELECT position FROM tasks WHERE id = 99)"));
        assert!(sql.contains("WHEN id = 99 THEN (SELECT position FROM tasks WHERE id = 3)"));
        assert!(sql.contains("WHERE id IN (3, 99)"));
    }

    // =========================================================================
    //  generate_update_sql
    // =========================================================================

    #[test]
    pub fn test_generate_update_sql_single() {
        // Single column update
        let sql = generate_update_sql("employees", 5, &("name", "Alice"));
        let expected = "UPDATE employees SET name = 'Alice' WHERE id = 5;";
        assert_eq!(sql, expected);
    }

    #[test]
    pub fn test_generate_update_sql_single_column_different_types() {
        let sql = generate_update_sql("products", 10, &("price", "99"));
        let expected = "UPDATE products SET price = '99' WHERE id = 10;";
        assert_eq!(sql, expected);
    }

    #[test]
    pub fn test_generate_update_sql_escapes_quotes() {
        // Single quotes inside values get doubled (SQL escaping)
        let sql = generate_update_sql("authors", 7, &("name", "O'Reilly"));
        let expected = "UPDATE authors SET name = 'O''Reilly' WHERE id = 7;";
        assert_eq!(sql, expected);
    }

    // =========================================================================
    //  sanitize
    // =========================================================================

    #[test]
    pub fn test_sanitize_no_quotes() {
        assert_eq!(sanitize("hello"), "hello");
    }

    #[test]
    pub fn test_sanitize_single_quote() {
        assert_eq!(sanitize("O'Reilly"), "O''Reilly");
    }

    #[test]
    pub fn test_sanitize_multiple_quotes() {
        assert_eq!(sanitize("a'b'c"), "a''b''c");
    }

    #[test]
    pub fn test_sanitize_empty_string() {
        assert_eq!(sanitize(""), "");
    }

    #[test]
    pub fn test_generate_insert_sql_escapes_quotes() {
        let table = Table {
            table_name: "authors".to_string(),
            columns: vec![Column {
                column_name: "name".to_string(),
                column_type: ColumnType::Text,
            }],
        };
        let sql = generate_insert_sql(&table, vec!["O'Reilly".to_string()]);
        let expected = "INSERT INTO authors (name) VALUES ('O''Reilly');";
        assert_eq!(sql, expected);
    }

    #[test]
    pub fn test_sanitize_quote_at_start_and_end() {
        assert_eq!(sanitize("'abc'"), "''abc''");
    }

    // =========================================================================
    //  generate_read_from_table_sql  —  COLUMNS: * vs explicit
    // =========================================================================

    #[test]
    fn test_select_all_columns_no_conditions() {
        // Empty vecs for both → SELECT * with no WHERE
        let sql = generate_read_from_table_sql("players", &[] as &[&str], &[] as &[&str]);
        assert_eq!(sql, "SELECT * FROM players;");
    }

    #[test]
    fn single_empty_string_column_becomes_star() {
        // [""] in columns slot → treated as "give me all columns"
        let sql = generate_read_from_table_sql("content", &["id = 1"], &[""]);
        assert_eq!(sql, "SELECT * FROM content WHERE id = 1;");
    }

    #[test]
    fn empty_string_mixed_with_real_column_should_be_filtered() {
        // Empty strings in the columns list must be filtered out
        let sql = generate_read_from_table_sql("content", &["id = 1"], &["col1", ""]);
        assert_eq!(sql, "SELECT col1 FROM content WHERE id = 1;");
    }

    // =========================================================================
    //  generate_read_from_table_sql  —  WHERE clause: presence / absence
    // =========================================================================

    #[test]
    fn empty_arguments_vec_omits_where() {
        // Empty conditions slice → no WHERE clause at all
        let sql = generate_read_from_table_sql("content", &[] as &[&str], &["col1", "col2"]);
        assert_eq!(sql, "SELECT col1, col2 FROM content;");
    }

    #[test]
    fn single_empty_string_argument_omits_where() {
        // A single empty string in conditions → treat as "no conditions"
        let sql = generate_read_from_table_sql("content", &[""], &["col1"]);
        assert_eq!(sql, "SELECT col1 FROM content;");
    }

    #[test]
    fn both_empty_gives_select_star_no_where() {
        // Empty string in both slots → SELECT * and no WHERE
        let sql = generate_read_from_table_sql("content", &[""], &[""]);
        assert_eq!(sql, "SELECT * FROM content;");
    }

    #[test]
    fn two_empty_string_arguments_should_omit_where() {
        // Multiple empty strings → all filtered, no WHERE remains
        let sql = generate_read_from_table_sql("content", &["", ""], &[""]);
        assert_eq!(sql, "SELECT * FROM content;");
    }

    #[test]
    fn empty_string_mixed_with_real_argument_should_be_filtered() {
        // Mix of empty and real conditions → empty removed, real kept
        let sql = generate_read_from_table_sql("content", &["", "id = 1"], &["col1"]);
        assert_eq!(sql, "SELECT col1 FROM content WHERE id = 1;");
    }

    // =========================================================================
    //  generate_read_from_table_sql  —  full combinations
    // =========================================================================

    #[test]
    fn test_select_specific_columns_with_condition() {
        // Explicit columns + single condition
        let sql = generate_read_from_table_sql("games", &["result = '1-0'"], &["white", "black"]);
        assert_eq!(sql, "SELECT white, black FROM games WHERE result = '1-0';");
    }

    #[test]
    fn test_multiple_conditions_multiple_columns() {
        // Multiple columns + multiple conditions joined with AND
        let sql = generate_read_from_table_sql(
            "puzzles",
            &["rating > 2000", "theme = 'mate'"],
            &["id", "fen", "solution"],
        );
        assert_eq!(
            sql,
            "SELECT id, fen, solution FROM puzzles WHERE rating > 2000 AND theme = 'mate';"
        );
    }

    #[test]
    fn test_empty_columns_with_condition() {
        // Empty columns slice + a real condition → * plus WHERE
        let sql =
            generate_read_from_table_sql("openings", &["name LIKE '%Sicilian%'"], &[] as &[&str]);
        assert_eq!(sql, "SELECT * FROM openings WHERE name LIKE '%Sicilian%';");
    }

    #[test]
    fn normal_case_still_works() {
        // Basic happy path: explicit columns + single condition
        let sql = generate_read_from_table_sql("content", &["id = 1"], &["col1", "col2"]);
        assert_eq!(sql, "SELECT col1, col2 FROM content WHERE id = 1;");
    }
}
