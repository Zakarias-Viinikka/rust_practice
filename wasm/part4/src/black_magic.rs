//use js_sys::{Array, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Blob, Url};
// new for sahpool
use sqlite_wasm_rs::{
    self as ffi,
    sahpool_vfs::{OpfsSAHPoolCfg, install as install_opfs_sahpool},
};
//old //use sqlite_wasm_rs as ffi; //necessary as far as i can tell.

use crate::black_magic_read::read_from_db;
use crate::create_sql_statements::*;
use crate::db_table::*;

use anyhow::{Result, anyhow, bail};
use std::ffi::CString; //let sql_cstr = CString::new(sql).map_err(|e| anyhow!("CString conversion failed: {}", e))?;

pub async fn create_local_db_connection(filename: &str) -> Result<*mut ffi::sqlite3> {
    let filename_cstr = CString::new(filename)?;
    let mut db = std::ptr::null_mut();

    // must be awaited to completion BEFORE opening
    install_opfs_sahpool(&OpfsSAHPoolCfg::default(), true).await?;

    // null = "use the default vfs", which is now sahpool since we passed `true` above
    let vfs_name: *const std::os::raw::c_char = std::ptr::null();
    let ret = unsafe {
        ffi::sqlite3_open_v2(
            filename_cstr.as_ptr().cast(),
            &mut db as *mut _,
            ffi::SQLITE_OPEN_READWRITE | ffi::SQLITE_OPEN_CREATE,
            vfs_name,
        )
    };
    if ret != ffi::SQLITE_OK {
        bail!("Failed to open database: {}", ffi::code_to_str(ret));
    }
    Ok(db)
}

pub fn create_table(db: *mut ffi::sqlite3, table: &Table) -> Result<()> {
    let sql = generate_create_table_sql(table);
    let sql_cstr = CString::new(sql).map_err(|e| anyhow!("CString conversion failed: {}", e))?;
    unsafe {
        let ret = ffi::sqlite3_exec(
            db,
            sql_cstr.as_ptr().cast(),
            None,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        if ret != ffi::SQLITE_OK {
            bail!("Failed to create table: {}", ffi::code_to_str(ret));
        }
        //log!("Table created");
        Ok(())
    }
}

pub fn insert_into_table(db: *mut ffi::sqlite3, table: &Table, values: Vec<String>) -> Result<()> {
    let sql = generate_insert_sql(table, values);
    let sql_cstr = CString::new(sql)?;
    unsafe {
        let ret = ffi::sqlite3_exec(
            db,
            sql_cstr.as_ptr(),
            None,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        if ret != ffi::SQLITE_OK {
            bail!("insert failed: {}", ffi::code_to_str(ret));
        }
    }
    Ok(())
}

/*pub fn export_db(db: *mut ffi::sqlite3) -> Result<()> {
    unsafe {
        let mut size: ffi::sqlite3_int64 = 0;
        let data = ffi::sqlite3_serialize(db, c"main".as_ptr().cast(), &mut size, 0);

        if data.is_null() {
            bail!("Failed to serialize database");
        }

        // Create a slice from the raw data pointer
        let bytes = std::slice::from_raw_parts(data as *const u8, size as usize);

        // Create a Uint8Array from the byte slice
        let uint8_array = js_sys::Uint8Array::view(bytes);

        // Create the Blob from the Array
        let blob = Blob::new_with_u8_array_sequence(&js_sys::Array::of1(&uint8_array))
            .map_err(|e| anyhow!("Failed to create Blob: {:?}", e))?;

        // Create object URL and trigger download
        let url = Url::create_object_url_with_blob(&blob)
            .map_err(|e| anyhow!("Failed to create object URL: {:?}", e))?;

        let window = web_sys::window().ok_or_else(|| anyhow!("No window available"))?;

        let document = window
            .document()
            .ok_or_else(|| anyhow!("No document available"))?;

        let a: web_sys::HtmlElement = document
            .create_element("a")
            .map_err(|e| anyhow!("Failed to create <a> element: {:?}", e))?
            .unchecked_into();

        a.set_attribute("href", &url)
            .map_err(|e| anyhow!("Failed to set href: {:?}", e))?;

        a.set_attribute("download", "database.sqlite")
            .map_err(|e| anyhow!("Failed to set download: {:?}", e))?;

        a.click();

        // Free memory
        ffi::sqlite3_free(data as *mut std::ffi::c_void);

        Ok(())
    }
}*/

pub fn drop_table(db_conn: *mut ffi::sqlite3, table_name: &str) -> Result<(), JsValue> {
    let sql = format!("DROP TABLE IF EXISTS {};", table_name);
    let c_sql = CString::new(sql).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let ret = unsafe {
        ffi::sqlite3_exec(
            db_conn,
            c_sql.as_ptr(),
            None,                 // callback: Option<fn(...)>
            std::ptr::null_mut(), // arg: *mut c_void
            std::ptr::null_mut(), // errmsg: *mut *mut c_char
        )
    };
    if ret != ffi::SQLITE_OK {
        let err = unsafe { ffi::sqlite3_errmsg(db_conn) };
        let err_str = unsafe { std::ffi::CStr::from_ptr(err).to_string_lossy().into_owned() };
        return Err(JsValue::from_str(&format!("Drop failed: {}", err_str)));
    }
    Ok(())
}

pub fn table_shape(db_conn: *mut ffi::sqlite3, table_name: &str) -> Result<Vec<String>, JsValue> {
    let sql = format!("PRAGMA table_info({});", table_name);
    let c_sql = CString::new(sql).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let mut stmt: *mut ffi::sqlite3_stmt = std::ptr::null_mut();
    // SAFETY: db_conn is a valid open connection, c_sql is a valid nul-terminated
    // string, and stmt is a valid out-pointer for prepare_v2 to write into.
    let ret = unsafe {
        ffi::sqlite3_prepare_v2(db_conn, c_sql.as_ptr(), -1, &mut stmt, std::ptr::null_mut())
    };
    if ret != ffi::SQLITE_OK {
        let err = unsafe { ffi::sqlite3_errmsg(db_conn) };
        let err_str = unsafe { std::ffi::CStr::from_ptr(err).to_string_lossy().into_owned() };
        return Err(JsValue::from_str(&format!("Prepare failed: {}", err_str)));
    }

    let mut columns = Vec::new();

    loop {
        // SAFETY: stmt was just successfully prepared above and is not yet finalized.
        let step_ret = unsafe { ffi::sqlite3_step(stmt) };
        match step_ret {
            ffi::SQLITE_ROW => {
                // SAFETY: stmt is on a valid row; column indices 0-5 match the
                // known column order of PRAGMA table_info's result set.
                unsafe {
                    let cid = ffi::sqlite3_column_int(stmt, 0);
                    let name =
                        std::ffi::CStr::from_ptr(ffi::sqlite3_column_text(stmt, 1) as *const i8)
                            .to_string_lossy()
                            .into_owned();
                    let col_type =
                        std::ffi::CStr::from_ptr(ffi::sqlite3_column_text(stmt, 2) as *const i8)
                            .to_string_lossy()
                            .into_owned();
                    let not_null = ffi::sqlite3_column_int(stmt, 3) != 0;
                    let pk = ffi::sqlite3_column_int(stmt, 5) != 0;

                    columns.push(format!(
                        "info{}: name={}, type={}, not_null={}, primary_key={}",
                        cid, name, col_type, not_null, pk
                    ));
                }
            }
            ffi::SQLITE_DONE => break,
            _ => {
                let err = unsafe { ffi::sqlite3_errmsg(db_conn) };
                let err_str =
                    unsafe { std::ffi::CStr::from_ptr(err).to_string_lossy().into_owned() };
                unsafe { ffi::sqlite3_finalize(stmt) };
                return Err(JsValue::from_str(&format!("Step failed: {}", err_str)));
            }
        }
    }

    // SAFETY: stmt is non-null and was successfully prepared; finalize is safe
    // to call exactly once when done stepping.
    unsafe { ffi::sqlite3_finalize(stmt) };

    Ok(columns)
}
