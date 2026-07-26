function log(message, tag = 'info') {
  const logEl = document.getElementById('log');
  if (!logEl) return;

  const entry = document.createElement('div');
  // Add a class if it's an error or fail – use your danger color
  if (tag === 'error' || tag === 'fail') {
    entry.classList.add('log-error');
  }

  const time = new Date().toLocaleTimeString();
  entry.innerHTML = `<span class="tag">[${tag}]</span> <span class="time">${time}</span> ${message}`;
  logEl.appendChild(entry);
  logEl.scrollTop = logEl.scrollHeight;
}

// --- worker setup ---
const myWorker = new Worker('test_worker.js', { type: 'module' });

// queue for ask() calls, so requests to the worker never overlap
let askQueue = Promise.resolve();

// wait for the worker to signal it's loaded and ready
await new Promise(resolve => {
    myWorker.onmessage = (e) => {
        if (e.data[0] === "ready") resolve();
    };
});

//document.getElementById('status')?.classList.add('ready');   // optional but harmless

// --- database setup ---
await worker_do_work(["initialize", "tests_db_conn_name"]);

log('ready', 'worker initialized');

function worker_do_work(msg) {
  askQueue = askQueue.then(() => new Promise(resolve => {
    myWorker.onmessage = (e) => resolve(e.data);
    myWorker.postMessage(msg);
  }));
  return askQueue;
}

function assert(condition, message) {
  if (!condition) throw new Error(message);
}

async function test(name, fn) {
  try {
    await fn();
    log(`✓ ${name}`, "pass");
  } catch (err) {
    log(`✗ ${name} – ${err.message}`, "fail");
  }
}
/*
  --- drop all tables ---
*/
await test("drop all tables", async () => {
  const reply = await worker_do_work(["list_tables"]);
  if (reply[0] === "error") throw new Error(reply[1]);

  const tables = reply[1];
  for (const name of tables) {
    const dropReply = await worker_do_work(["delete_table", name]);
    if (dropReply[0] === "error") throw new Error(`drop ${name} failed: ${dropReply[1]}`);
  }

  const final = await worker_do_work(["list_tables"]);
  if (final[0] === "error") throw new Error(final[1]);
  if (final[1].length !== 0) throw new Error("not all tables dropped");
});

/*
  --- db is empty at start ---
*/
await test("db is empty at start", async () => {
  const reply = await worker_do_work(["list_tables"]);
  if (reply[0] === "error") throw new Error(reply[1]);
  const tables = reply[1];
  if (tables.length !== 0) throw new Error(`expected empty, got ${tables.length} tables`);
});

/*
  --- create table, insert rows, check schema, then verify data ---
*/
await test("create table, insert rows, check schema & data", async () => {
  // 1. Create a table with two columns (default value must be a string, not null)
  const columns = [
    // [name, type, primaryKey, notNull, unique, defaultValue, autoincrement]
    ["id",   "INTEGER", true,  true,  true,  "",   false, true ],
    ["name", "TEXT",    false, false, false, "",   false, false]
  ];

  let reply = await worker_do_work(["create_table", "people", columns]);
  if (reply[0] === "error") throw new Error("create table failed: " + reply[1]);

  // 2. Insert two rows (only the 'name' column, id auto-generates)
  reply = await worker_do_work(["insert_data", "people", ["name"], ["Alice"]]);
  if (reply[0] === "error") throw new Error("insert 1 failed: " + reply[1]);
  reply = await worker_do_work(["insert_data", "people", ["name"], ["Bob"]]);
  if (reply[0] === "error") throw new Error("insert 2 failed: " + reply[1]);

  // 3. Check the table schema
  reply = await worker_do_work(["check_table", "people"]);
  if (reply[0] === "error") throw new Error("check_table failed: " + reply[1]);

  const schemaLines = reply[1]; // array of strings like "info0: name=id, type=INTEGER, ..."

  // 4. Schema assertions
  if (!Array.isArray(schemaLines) || schemaLines.length !== 2) {
    throw new Error(`expected 2 columns, got ${JSON.stringify(schemaLines)}`);
  }

  const col0 = schemaLines[0];
  if (!col0.includes("name=id"))   throw new Error(`col0 missing name=id: ${col0}`);
  if (!col0.includes("type=INTEGER")) throw new Error(`col0 missing type=INTEGER: ${col0}`);

  const col1 = schemaLines[1];
  if (!col1.includes("name=name"))   throw new Error(`col1 missing name=name: ${col1}`);
  if (!col1.includes("type=TEXT"))   throw new Error(`col1 missing type=TEXT: ${col1}`);

  // 5. Verify data via get_data – request only the 'name' column for all rows
  reply = await worker_do_work(["get_data", "people", "", ["name"]]);
  if (reply[0] === "error") throw new Error("get_data failed: " + reply[1]);

  const rows = reply[1]; // expected shape: [["Alice"], ["Bob"]]
  if (!Array.isArray(rows) || rows.length !== 2) {
    throw new Error(`expected 2 rows, got ${JSON.stringify(rows)}`);
  }

  // extract names from each row (each row is an array of one element)
  const names = rows.map(row => row[0]).sort();
  if (names[0] !== "Alice" || names[1] !== "Bob") {
    throw new Error(`expected names Alice & Bob, got ${JSON.stringify(names)}`);
  }
});

// ----- create_table error cases -----

await test("create_table: duplicate name is allowed", async () => {
  const tbl = "dup_test";
  const cols = [["id","INTEGER",true,true,true,"",false]];

  let reply = await worker_do_work(["create_table", tbl, cols]);
  if (reply[0] === "error") throw new Error("initial create failed: " + reply[1]);

  reply = await worker_do_work(["create_table", tbl, cols]);
  if (reply[0] === "error") throw new Error("expected duplicate create to succeed, got error: " + reply[1]);

  await worker_do_work(["delete_table", tbl]);
});

await test("create_table: empty name fails", async () => {
  const reply = await worker_do_work(["create_table", "", [["id","INTEGER",true,true,true,"",false]]]);
  if (reply[0] !== "error") throw new Error("expected error for empty table name");
});

await test("create_table: zero columns fails", async () => {
  const reply = await worker_do_work(["create_table", "zero_cols", []]);
  if (reply[0] !== "error") throw new Error("expected error for empty column list");
});

await test("create_table: malformed column definition fails", async () => {
  // ColumnDef expects exactly 7 elements; providing fewer should fail at deserialization
  const badCols = [["id","INTEGER",true]]; // only 3 elements
  const reply = await worker_do_work(["create_table", "bad_def", badCols]);
  if (reply[0] !== "error") throw new Error("expected error for malformed column definition");
});

// ----- insert_data error cases -----

// set up a table with a NOT NULL column and no default
await test("setup: create table for insert error tests", async () => {
  const cols = [
    ["id",      "INTEGER", true, true,  true,  "",   false],
    ["username","TEXT",    false,true,  false, "",   false, false]  // NOT NULL
  ];
  let reply = await worker_do_work(["create_table", "err_test", cols]);
  if (reply[0] === "error") throw new Error("setup failed: " + reply[1]);
});

// --- actual error cases ---

await test("insert_data: non-existent table fails", async () => {
  const reply = await worker_do_work(["insert_data", "ghost", ["col"], ["val"]]);
  if (reply[0] !== "error") throw new Error("expected error for non-existent table");
});

await test("insert_data: wrong column name fails", async () => {
  // 'name' doesn't exist in err_test
  const reply = await worker_do_work(["insert_data", "err_test", ["name"], ["test"]]);
  if (reply[0] !== "error") throw new Error("expected error for bad column name");
});

await test("insert_data: missing NOT NULL value fails", async () => {
  // omit the 'username' column which is NOT NULL
  const reply = await worker_do_work(["insert_data", "err_test", ["id"], ["1"]]);
  if (reply[0] !== "error") throw new Error("expected error when violating NOT NULL");
});

await test("insert_data: duplicate primary key fails", async () => {
  // 'people' table already has rows with id=1 and id=2. Try to insert a row with id=1 explicitly.
  const reply = await worker_do_work(["insert_data", "people", ["id","name"], ["1","evil"]]);
  if (reply[0] !== "error") throw new Error("expected error for duplicate PK");
});

// clean up
await test("cleanup: drop err_test", async () => {
  await worker_do_work(["delete_table", "err_test"]);
});

await test("edit row: non-existent row is silently ignored", async () => {
  // count rows before the attempt
  let reply = await worker_do_work(["get_data", "people", "", ["id"]]);
  if (reply[0] === "error") throw new Error("get_data failed: " + reply[1]);
  const beforeCount = reply[1].length;

  // try to edit a row that doesn't exist
  reply = await worker_do_work(["edit_row", "people", "999", "name", "Ghost"]);
  if (reply[0] === "error") throw new Error("unexpected error editing non-existent row: " + reply[1]);

  // verify row count is unchanged
  reply = await worker_do_work(["get_data", "people", "", ["id"]]);
  if (reply[0] === "error") throw new Error("get_data failed: " + reply[1]);
  const afterCount = reply[1].length;
  if (afterCount !== beforeCount) {
    throw new Error(`row count changed: ${beforeCount} → ${afterCount}`);
  }
});

await test("delete row: non-existent row is silently ignored", async () => {
  // count rows before
  let reply = await worker_do_work(["get_data", "people", "", ["id"]]);
  if (reply[0] === "error") throw new Error("get_data failed: " + reply[1]);
  const beforeCount = reply[1].length;

  // delete a row that doesn't exist
  reply = await worker_do_work(["delete_row", "people", "999"]);
  if (reply[0] === "error") throw new Error("unexpected error deleting non-existent row: " + reply[1]);

  // verify row count is unchanged
  reply = await worker_do_work(["get_data", "people", "", ["id"]]);
  if (reply[0] === "error") throw new Error("get_data failed: " + reply[1]);
  const afterCount = reply[1].length;
  if (afterCount !== beforeCount) {
    throw new Error(`row count changed: ${beforeCount} → ${afterCount}`);
  }
});

await test("swap: exchange values between two rows", async () => {
  // swap 'name' between id=1 (Alicia) and id=2 (Bob)
  let reply = await worker_do_work(["swap_columns", "people", "1", "2", "name"]);
  if (reply[0] === "error") throw new Error("swap failed: " + reply[1]);

  // verify row 1 now has "Bob"
  reply = await worker_do_work(["get_data", "people", "id=1", ["name"]]);
  if (reply[0] === "error") throw new Error("get_data failed: " + reply[1]);
  if (reply[1][0][0] !== "Bob") throw new Error(`row 1 expected "Bob", got "${reply[1][0][0]}"`);

  // verify row 2 now has "Alicia"
  reply = await worker_do_work(["get_data", "people", "id=2", ["name"]]);
  if (reply[0] === "error") throw new Error("get_data failed: " + reply[1]);
  if (reply[1][0][0] !== "Alice") throw new Error(`row 2 expected "Alice", got "${reply[1][0][0]}"`);
});

await test("swap: same row – value unchanged, no error", async () => {
  // swap name on row 1 with itself
  let reply = await worker_do_work(["swap_columns", "people", "1", "1", "name"]);
  if (reply[0] === "error") throw new Error("swap with same row should not error: " + reply[1]);

  // value must still be "Bob" (from previous swap)
  reply = await worker_do_work(["get_data", "people", "id=1", ["name"]]);
  if (reply[0] === "error") throw new Error("get_data failed: " + reply[1]);
  if (reply[1][0][0] !== "Bob") throw new Error(`row 1 expected "Bob", got "${reply[1][0][0]}"`);
});

await test("swap: non-existent row fails", async () => {
  const reply = await worker_do_work(["swap_columns", "people", "1", "999", "name"]);
  if (reply[0] !== "error") throw new Error("expected error for non-existent row");
});

await test("swap: non-existent column fails", async () => {
  const reply = await worker_do_work(["swap_columns", "people", "1", "2", "age"]);
  if (reply[0] !== "error") throw new Error("expected error for non-existent column");
});

await test("swap: non-existent table fails", async () => {
  const reply = await worker_do_work(["swap_columns", "ghost", "1", "2", "name"]);
  if (reply[0] !== "error") throw new Error("expected error for non-existent table");
});

await test("get_data: single row by id", async () => {
  const reply = await worker_do_work(["get_data", "people", "id=1", ["name"]]);
  if (reply[0] === "error") throw new Error("get_data failed: " + reply[1]);
  const rows = reply[1];
  if (!Array.isArray(rows) || rows.length !== 1) {
    throw new Error(`expected 1 row, got ${JSON.stringify(rows)}`);
  }
  if (rows[0][0] !== "Bob") throw new Error(`expected "Bob", got "${rows[0][0]}"`);
});

await test("get_data: subset of columns", async () => {
  // request only the 'id' column
  const reply = await worker_do_work(["get_data", "people", "", ["id"]]);
  if (reply[0] === "error") throw new Error("get_data failed: " + reply[1]);
  const rows = reply[1];
  if (!Array.isArray(rows) || rows.length !== 2) {
    throw new Error(`expected 2 rows, got ${rows.length}`);
  }
  // each row should be an array of exactly one element (the id)
  rows.forEach((row, i) => {
    if (row.length !== 1) throw new Error(`row ${i} expected 1 column, got ${row.length}`);
  });
});

await test("get_data: empty table returns empty array", async () => {
  // create a temporary empty table
  await worker_do_work(["create_table", "empty_tbl", [["id","INTEGER",true,true,true,"",false]]]);
  const reply = await worker_do_work(["get_data", "empty_tbl", "", ["id"]]);
  if (reply[0] === "error") throw new Error("get_data failed: " + reply[1]);
  if (!Array.isArray(reply[1]) || reply[1].length !== 0) {
    throw new Error(`expected empty array, got ${JSON.stringify(reply[1])}`);
  }
  await worker_do_work(["delete_table", "empty_tbl"]);
});

await test("get_data: non-existent row returns empty", async () => {
  const reply = await worker_do_work(["get_data", "people", "id=999", ["name"]]);
  if (reply[0] === "error") throw new Error("get_data failed: " + reply[1]);
  if (!Array.isArray(reply[1]) || reply[1].length !== 0) {
    throw new Error(`expected empty array for missing row, got ${JSON.stringify(reply[1])}`);
  }
});

/* testing concurrent inserts: ordering and consistency */
await test("concurrent inserts: ordering and consistency", async () => {
  const tbl = "concurrent_test";

  // 1. Create a simple table with autoincrement id and a text value
  let reply = await worker_do_work(["create_table", tbl, [
    ["id",  "INTEGER", true, true, true, "", false, true],
    ["val", "TEXT",    false,false,false,"", false, false]
  ]]);
  if (reply[0] === "error") throw new Error("create table failed: " + reply[1]);

  // 2. Prepare multiple inserts and fire them ALL at the same time
  const values = ["first", "second", "third", "fourth", "fifth"];
  const insertPromises = values.map(v =>
    worker_do_work(["insert_data", tbl, ["val"], [v]])
  );

  // 3. Wait for all inserts to complete (they are enqueued concurrently)
  const insertResults = await Promise.all(insertPromises);
  insertResults.forEach((res, i) => {
    if (res[0] === "error") throw new Error(`insert ${i} ("${values[i]}") failed: ${res[1]}`);
  });

  // 4. Read all rows (id and val) – they must be in insertion order
  reply = await worker_do_work(["get_data", tbl, "", ["id", "val"]]);
  if (reply[0] === "error") throw new Error("get_data failed: " + reply[1]);
  const rows = reply[1];
  if (rows.length !== values.length) {
    throw new Error(`expected ${values.length} rows, got ${rows.length}`);
  }

  // 5. Verify that ids are 1,2,3,4,5 and values match the exact order
  for (let i = 0; i < values.length; i++) {
    const [id, val] = rows[i];
    if (id !== String(i + 1)) throw new Error(`row ${i} id expected ${i + 1}, got ${id}`);
    if (val !== values[i])    throw new Error(`row ${i} val expected "${values[i]}", got "${val}"`);
  }

  // 6. Clean up
  await worker_do_work(["delete_table", tbl]);
});

// ----- Edit and Delete row tests -----

await test("setup: create table for edit/delete tests", async () => {
  const cols = [
    ["id",  "INTEGER", true, true, true, "", false, true],
    ["name","TEXT",    false,false,false,"", false, false],
    ["age", "INTEGER", false,false,false,"", false, false]
  ];
  let reply = await worker_do_work(["create_table", "edit_delete_test", cols]);
  if (reply[0] === "error") throw new Error("create table failed: " + reply[1]);

  // Insert two rows: Alice (30), Bob (25)
  reply = await worker_do_work(["insert_data", "edit_delete_test", ["name","age"], ["Alice","30"]]);
  if (reply[0] === "error") throw new Error("insert Alice failed: " + reply[1]);
  reply = await worker_do_work(["insert_data", "edit_delete_test", ["name","age"], ["Bob","25"]]);
  if (reply[0] === "error") throw new Error("insert Bob failed: " + reply[1]);
});

await test("edit row: change a value and verify", async () => {
  // Change Alice's age to 31
  let reply = await worker_do_work(["edit_row", "edit_delete_test", "1", "age", "31"]);
  if (reply[0] === "error") throw new Error("edit failed: " + reply[1]);

  // Verify via get_data
  reply = await worker_do_work(["get_data", "edit_delete_test", "id=1", ["age"]]);
  if (reply[0] === "error") throw new Error("get_data failed: " + reply[1]);
  if (reply[1][0][0] !== "31") throw new Error(`expected age 31, got "${reply[1][0][0]}"`);
});

await test("edit row: non-existent column should fail", async () => {
  const reply = await worker_do_work(["edit_row", "edit_delete_test", "1", "salary", "50000"]);
  if (reply[0] !== "error") throw new Error("expected error for non-existent column");
});

await test("edit row: non-existent row is silently ignored", async () => {
  // Count rows before
  let reply = await worker_do_work(["get_data", "edit_delete_test", "", ["id"]]);
  const beforeCount = reply[1].length;

  // Edit row 999
  reply = await worker_do_work(["edit_row", "edit_delete_test", "999", "name", "Ghost"]);
  // Should succeed (no error), because no-op is the current behavior
  if (reply[0] === "error") throw new Error("unexpected error for non-existent row edit: " + reply[1]);

  // Row count must not change
  reply = await worker_do_work(["get_data", "edit_delete_test", "", ["id"]]);
  const afterCount = reply[1].length;
  if (afterCount !== beforeCount) throw new Error(`row count changed: ${beforeCount} → ${afterCount}`);
});

await test("delete row: remove a row and verify", async () => {
  // Delete Bob (id=2)
  let reply = await worker_do_work(["delete_row", "edit_delete_test", "2"]);
  if (reply[0] === "error") throw new Error("delete failed: " + reply[1]);

  // Get all rows – should be only Alice (id=1)
  reply = await worker_do_work(["get_data", "edit_delete_test", "", ["id","name","age"]]);
  if (reply[0] === "error") throw new Error("get_data failed: " + reply[1]);
  const rows = reply[1];
  if (rows.length !== 1) throw new Error(`expected 1 row, got ${rows.length}`);
  if (rows[0][1] !== "Alice") throw new Error(`expected Alice, got "${rows[0][1]}"`);
  if (rows[0][2] !== "31")    throw new Error(`age should be 31, got "${rows[0][2]}"`);
});

await test("delete row: non-existent row is silently ignored", async () => {
  // Count rows before
  let reply = await worker_do_work(["get_data", "edit_delete_test", "", ["id"]]);
  const beforeCount = reply[1].length;

  // Delete row 999
  reply = await worker_do_work(["delete_row", "edit_delete_test", "999"]);
  if (reply[0] === "error") throw new Error("unexpected error for non-existent row delete: " + reply[1]);

  // Row count unchanged
  reply = await worker_do_work(["get_data", "edit_delete_test", "", ["id"]]);
  const afterCount = reply[1].length;
  if (afterCount !== beforeCount) throw new Error(`row count changed: ${beforeCount} → ${afterCount}`);
});

await test("delete row: delete last row leaves table empty", async () => {
  // Delete Alice (id=1)
  let reply = await worker_do_work(["delete_row", "edit_delete_test", "1"]);
  if (reply[0] === "error") throw new Error("delete last row failed: " + reply[1]);

  // Table should be empty
  reply = await worker_do_work(["get_data", "edit_delete_test", "", ["id"]]);
  if (reply[0] === "error") throw new Error("get_data failed: " + reply[1]);
  if (reply[1].length !== 0) throw new Error("table not empty after deleting all rows");
});

await test("cleanup: drop edit_delete_test table", async () => {
  await worker_do_work(["delete_table", "edit_delete_test"]);
});

await test("list_tables: multiple tables are returned", async () => {
  // Create two distinct tables
  await worker_do_work(["create_table", "multi_test_a", [["id","INTEGER",true,true,true,"",false,true]]]);
  await worker_do_work(["create_table", "multi_test_b", [["id","INTEGER",true,true,true,"",false,true]]]);

  // Call list_tables
  const reply = await worker_do_work(["list_tables"]);
  if (reply[0] === "error") throw new Error("list_tables failed: " + reply[1]);

  const tables = reply[1];
  if (!Array.isArray(tables)) throw new Error("expected array of table names");

  // Verify both tables are present
  if (!tables.includes("multi_test_a")) throw new Error("missing multi_test_a");
  if (!tables.includes("multi_test_b")) throw new Error("missing multi_test_b");

  // Clean up
  await worker_do_work(["delete_table", "multi_test_a"]);
  await worker_do_work(["delete_table", "multi_test_b"]);
});

// ----- Create index tests -----

// Ensure we have a table to index
await test("setup: table for index tests", async () => {
  const cols = [
    ["id",  "INTEGER", true, true, true, "", false, true],
    ["tag", "TEXT",    false,false,false,"", false, false]
  ];
  let reply = await worker_do_work(["create_table", "index_test", cols]);
  if (reply[0] === "error") throw new Error("create table failed: " + reply[1]);
});

await test("create index: on existing column succeeds", async () => {
  const reply = await worker_do_work(["create_index", "index_test", "tag"]);
  // We just care that no error occurred.
  if (reply[0] === "error") throw new Error("create index failed: " + reply[1]);
  // Optionally check the success message
  if (reply[1] !== undefined && !reply[1].includes("indexed")) {
    throw new Error(`unexpected success message: ${reply[1]}`);
  }
});

await test("create index: non-existent table should error", async () => {
  const reply = await worker_do_work(["create_index", "ghost", "col"]);
  // This is expected to fail; log a pass if an error is returned, otherwise log the actual outcome.
  if (reply[0] !== "error") {
    // If it doesn't error, document what happened and pass anyway (or fail, depending on desired contract)
    throw new Error(`expected error for missing table, but got ${JSON.stringify(reply)}`);
  }
  // If error, test passes automatically (no throw)
});

await test("create index: non-existent column should error", async () => {
  const reply = await worker_do_work(["create_index", "index_test", "nonexistent"]);
  if (reply[0] !== "error") {
    throw new Error(`expected error for missing column, but got ${JSON.stringify(reply)}`);
  }
});

await test("cleanup: drop index_test table", async () => {
  await worker_do_work(["delete_table", "index_test"]);
});

/*
  Tests for behavior that succeeds silently instead of erroring.
  Meant to be pasted into your existing suite (anywhere after "initialize"
  has run). Each test creates and drops its own table, so it doesn't
  depend on — or interfere with — state from your other tests.

  Every test here is traced to something specific:
    A, B, C, D  -> your Rust code uses "IF NOT EXISTS" in several places
    E, F, G     -> Rust's `zip()` truncates, and NULL reads back as ""
*/

// --- A: check_table on a table that was never created ---
await test("check_table: non-existent table returns empty list, no error", async () => {
  const reply = await worker_do_work(["check_table", "definitely_not_a_real_table"]);
  if (reply[0] === "error") throw new Error(`expected no error, got: ${reply[1]}`);
  if (!Array.isArray(reply[1]) || reply[1].length !== 0) {
    throw new Error(`expected empty schema, got: ${JSON.stringify(reply[1])}`);
  }
});

// --- B: re-creating a table with a different schema keeps old schema ---
await test("create_table: re-creating with different columns keeps the OLD schema", async () => {
  const tbl = "schema_drift_test";
  const originalCols = [["id", "INTEGER", true, true, true, "", false]];
  const differentCols = [
    ["id", "INTEGER", true, true, true, "", false],
    ["extra_col", "TEXT", false, false, false, "", false],
  ];

  let reply = await worker_do_work(["create_table", tbl, originalCols]);
  if (reply[0] === "error") throw new Error("initial create failed: " + reply[1]);

  reply = await worker_do_work(["create_table", tbl, differentCols]);
  if (reply[0] === "error") {
    throw new Error("expected second create to succeed silently, got error: " + reply[1]);
  }

  reply = await worker_do_work(["check_table", tbl]);
  if (reply[0] === "error") throw new Error("check_table failed: " + reply[1]);
  const schemaLines = reply[1];

  if (schemaLines.length !== 1) {
    throw new Error(
      `expected schema to stay at 1 column (old schema wins), got ${schemaLines.length}: ${JSON.stringify(schemaLines)}`
    );
  }

  await worker_do_work(["delete_table", tbl]);
});

// --- C: creating the same index twice ---
await test("create_index: creating the same index twice succeeds silently", async () => {
  const tbl = "dup_index_test";
  await worker_do_work(["create_table", tbl, [["id", "INTEGER", true, true, true, "", false]]]);

  let reply = await worker_do_work(["create_index", tbl, "id"]);
  if (reply[0] === "error") throw new Error("first create_index failed: " + reply[1]);

  reply = await worker_do_work(["create_index", tbl, "id"]);
  if (reply[0] === "error") {
    throw new Error("expected duplicate create_index to succeed silently, got error: " + reply[1]);
  }

  await worker_do_work(["delete_table", tbl]);
});

// --- D: deleting a table that was never created ---
await test("delete_table: non-existent table succeeds silently (no error)", async () => {
  const reply = await worker_do_work(["delete_table", "table_that_was_never_created"]);
  if (reply[0] === "error") throw new Error(`expected no error, got: ${reply[1]}`);
});

// --- E: insert_data with fewer values than column names ---
await test("insert_data: fewer values than column names silently drops the extra column", async () => {
  const tbl = "mismatch_test";
  await worker_do_work(["create_table", tbl, [
    ["id", "INTEGER", true, true, true, "", false],
    ["name", "TEXT", false, false, false, "", false],
  ]]);

  // 2 column names, only 1 value → name is omitted, becomes NULL → reads as ""
  const reply = await worker_do_work(["insert_data", tbl, ["id", "name"], ["7"]]);
  if (reply[0] === "error") throw new Error(`expected this to succeed silently, got error: ${reply[1]}`);

  const check = await worker_do_work(["get_data", tbl, "", ["id", "name"]]);
  if (check[0] === "error") throw new Error("get_data failed: " + check[1]);
  if (check[1][0][1] !== "") {
    throw new Error(`expected "name" to read back as "", got: ${JSON.stringify(check[1][0])}`);
  }

  await worker_do_work(["delete_table", tbl]);
});

// --- F: insert_data with more values than column names ---
await test("insert_data: more values than column names silently drops the extra value", async () => {
  const tbl = "mismatch_test2";
  await worker_do_work(["create_table", tbl, [
    ["id", "INTEGER", true, true, true, "", false],
    ["name", "TEXT", false, false, false, "", false],
  ]]);

  // 1 column name, 2 values → ExtraIgnored is discarded
  const reply = await worker_do_work(["insert_data", tbl, ["name"], ["Alice", "ExtraIgnored"]]);
  if (reply[0] === "error") throw new Error(`expected this to succeed silently, got error: ${reply[1]}`);

  const check = await worker_do_work(["get_data", tbl, "", ["name"]]);
  if (check[0] === "error") throw new Error("get_data failed: " + check[1]);
  if (check[1][0][0] !== "Alice") {
    throw new Error(`expected "Alice" (the paired value), got: ${JSON.stringify(check[1][0])}`);
  }

  await worker_do_work(["delete_table", tbl]);
});

// --- G: NULL vs empty string are indistinguishable on read ---
await test("silent: NULL and empty string both read back as ''", async () => {
  const tbl = "null_vs_empty";
  // Table with a nullable text column and no default
  await worker_do_work(["create_table", tbl, [
    ["id",  "INTEGER", true, true, true, "", false],
    ["data", "TEXT",    false,false,false,"", false, false]
  ]]);

  // Insert a row with an explicit empty string
  await worker_do_work(["insert_data", tbl, ["id", "data"], ["1", ""]]);
  // Insert a row where we omit 'data' → it will be NULL
  await worker_do_work(["insert_data", tbl, ["id"], ["2"]]);

  const reply = await worker_do_work(["get_data", tbl, "", ["id", "data"]]);
  if (reply[0] === "error") throw new Error("get_data failed: " + reply[1]);
  const rows = reply[1];
  if (rows.length !== 2) throw new Error("expected 2 rows");

  // Both should be "" — we can't tell apart
  const row1data = rows[0][1];
  const row2data = rows[1][1];
  if (row1data !== "") throw new Error(`row1 expected '', got '${row1data}'`);
  if (row2data !== "") throw new Error(`row2 expected '', got '${row2data}'`);

  // If we ever want to distinguish them, this test will start failing,
  // which is exactly the point.

  await worker_do_work(["delete_table", tbl]);
});

// // //


// ============================================================
//  1. drop_table (the "drop all tables" command) – should error
// ============================================================
await test("drop_table (no arg): expected to error because Rust expects a table name", async () => {
  // The worker handler calls db_manager.drop_table() with NO argument,
  // but Rust requires table_name: String. This should fail.
  const reply = await worker_do_work(["drop_table"]);
  if (reply[0] !== "error") {
    throw new Error("expected error for missing table_name argument, but got success: " + JSON.stringify(reply));
  }
  // If it fails, test passes. If it somehow succeeds, the test fails – which is still useful.
});

// ============================================================
//  2. get_data, edit_row, delete_row on a non‑existent table
// ============================================================
await test("get_data: non-existent table should error (or return empty?)", async () => {
  const reply = await worker_do_work(["get_data", "table_does_not_exist", "", ["id"]]);
  // Currently unknown behaviour: it may error, or it may return an empty array.
  // We'll assert that either an error is returned OR the payload is an empty array.
  if (reply[0] === "error") {
    // If it errors, that's fine – we just document it as the expected behaviour.
    return;
  }
  // If it succeeded, check that the payload is an empty array (consistent with "no such table" = empty result)
  if (!Array.isArray(reply[1]) || reply[1].length !== 0) {
    throw new Error(`expected error or empty array, got: ${JSON.stringify(reply)}`);
  }
});

await test("edit_row: non-existent table should error", async () => {
  const reply = await worker_do_work(["edit_row", "nope_table", "1", "col", "val"]);
  if (reply[0] !== "error") throw new Error("expected error for non-existent table, got: " + JSON.stringify(reply));
});

await test("delete_row: non-existent table should error", async () => {
  const reply = await worker_do_work(["delete_row", "ghost_table", "1"]);
  if (reply[0] !== "error") throw new Error("expected error for non-existent table, got: " + JSON.stringify(reply));
});

// ============================================================
//  3. get_data with an empty column list
// ============================================================
await test("get_data: empty column list – what happens?", async () => {
  // Create a temporary table
  const tbl = "empty_cols_test";
  await worker_do_work(["create_table", tbl, [["id","INTEGER",true,true,true,"",false]]]);
  await worker_do_work(["insert_data", tbl, ["id"], ["1"]]);

  const reply = await worker_do_work(["get_data", tbl, "", []]);
  // This might generate 'SELECT FROM ...' which is invalid SQL -> error.
  // Or maybe your Rust code handles it differently. Let's find out.
  if (reply[0] === "error") {
    // Good, it caught the problem. Pass the test.
    await worker_do_work(["delete_table", tbl]);
    return;
  }
  // If it succeeded, log what we got – it may be an empty row or something weird.
  const payload = reply[1];
  console.log("get_data with empty columns returned:", payload);
  await worker_do_work(["delete_table", tbl]);
  // Don't fail the test; just document that it doesn't error.
  // (Change this if you want it to be considered a failure)
});

// ============================================================
//  4. insert_data with empty col_names and vals arrays
// ============================================================
await test("insert_data: empty columns/values – expect default row or error", async () => {
  const tbl = "empty_insert_test";
  await worker_do_work(["create_table", tbl, [["id","INTEGER",true,true,true,"",false]]]);

  const reply = await worker_do_work(["insert_data", tbl, [], []]);
  // Probably succeeds and inserts a row with the auto‑increment id and no other values.
  // Or it could error. We'll check.
  if (reply[0] === "error") {
    // Error is also acceptable, just document it.
    await worker_do_work(["delete_table", tbl]);
    return;
  }

  // If success, check that a row was actually added.
  const check = await worker_do_work(["get_data", tbl, "", ["id"]]);
  if (check[0] === "error") throw new Error("get_data failed: " + check[1]);
  if (check[1].length !== 1) {
    throw new Error(`expected 1 row, got ${check[1].length}`);
  }
  await worker_do_work(["delete_table", tbl]);
});

// ============================================================
//  5. Edit / delete with a non‑numeric row ID
// ============================================================
await test("edit_row: non‑numeric row id is silently ignored (no matching row)", async () => {
  const tbl = "non_numeric_test";
  await worker_do_work(["create_table", tbl, [["id","INTEGER",true,true,true,"",false], ["val","TEXT",false,false,false,"",false]]]);
  await worker_do_work(["insert_data", tbl, ["id","val"], ["1","hello"]]);

  // Try to edit with "abc" as the row id
  let reply = await worker_do_work(["edit_row", tbl, "abc", "val", "new"]);
  // Should either error or succeed with no change. We'll accept either as long as data is unchanged.
  // To verify, read the row back.
  reply = await worker_do_work(["get_data", tbl, "id=1", ["val"]]);
  if (reply[0] === "error") throw new Error("get_data failed: " + reply[1]);
  if (reply[1][0][0] !== "hello") {
    throw new Error(`row was changed to "${reply[1][0][0]}" when it should have stayed "hello"`);
  }
  await worker_do_work(["delete_table", tbl]);
});

await test("delete_row: non‑numeric row id is silently ignored", async () => {
  const tbl = "non_numeric_del";
  await worker_do_work(["create_table", tbl, [["id","INTEGER",true,true,true,"",false]]]);
  await worker_do_work(["insert_data", tbl, ["id"], ["1"]]);

  const reply = await worker_do_work(["delete_row", tbl, "xyz"]);
  // Should not error, and row count should stay 1
  const check = await worker_do_work(["get_data", tbl, "", ["id"]]);
  if (check[0] === "error") throw new Error("get_data failed: " + check[1]);
  if (check[1].length !== 1) throw new Error(`row was deleted when it shouldn't have been`);
  await worker_do_work(["delete_table", tbl]);
});

// ============================================================
//  6. create_table with special characters in names
// ============================================================
await test("create_table: table name with spaces and quotes works", async () => {
  const tbl = "my table with 'quotes'";
  const reply = await worker_do_work(["create_table", tbl, [["id","INTEGER",true,true,true,"",false]]]);
  if (reply[0] === "error") {
    // If it errors, that's fine – we just want to document it.
    // Clean up may fail, but we can ignore.
    return;
  }
  // If successful, check that the table exists
  const tables = await worker_do_work(["list_tables"]);
  if (!tables[1].includes(tbl)) throw new Error("table not found in list_tables");
  await worker_do_work(["delete_table", tbl]);
});

await test("create_table: column name is a SQL keyword (select)", async () => {
  const tbl = "keyword_col";
  const cols = [["select", "TEXT", false,false,false,"",false]];
  const reply = await worker_do_work(["create_table", tbl, cols]);
  if (reply[0] === "error") {
    // If it fails because 'select' is reserved, that's okay.
    return;
  }
  const check = await worker_do_work(["check_table", tbl]);
  if (check[0] === "error") throw new Error("check_table failed: " + check[1]);
  const schema = check[1];
  if (!schema[0].includes("name=select")) throw new Error(`expected column name 'select', got: ${schema[0]}`);
  await worker_do_work(["delete_table", tbl]);
});

// ============================================================
//  7. create_table with a non‑empty default value
// ============================================================
await test("create_table: non‑empty default value is used", async () => {
  const tbl = "default_val_test";
  const cols = [
    ["id",  "INTEGER", true, true, true, "42", false],   // default 42 for id? That's weird but let's test a text default
    ["tag", "TEXT",    false,false,false,"'hello'", false] // note: the default value string must include quotes if it's a literal in SQL
  ];
  // The way your ColumnDef works, the default field is a String that will be inserted directly into the SQL.
  // For TEXT defaults, it should contain the SQL literal, e.g. "'hello'" (with single quotes).
  // If you pass "hello" without quotes, SQL will try to interpret it as a column name and fail.
  // Let's use a TEXT default that is a number as string for simplicity:
  const colsSafe = [["id","INTEGER",true,true,true,"",false], ["status","TEXT",false,false,false,"'ok'",false]];
  const reply = await worker_do_work(["create_table", tbl, colsSafe]);
  if (reply[0] === "error") throw new Error("create_table with default failed: " + reply[1]);

  // Insert a row without specifying 'status'
  await worker_do_work(["insert_data", tbl, ["id"], ["1"]]);
  const check = await worker_do_work(["get_data", tbl, "id=1", ["status"]]);
  if (check[0] === "error") throw new Error("get_data failed: " + check[1]);
  // The default 'ok' should be present. Because of quoting, it should be stored as "ok"
  if (check[1][0][0] !== "ok") throw new Error(`expected default 'ok', got: "${check[1][0][0]}"`);
  await worker_do_work(["delete_table", tbl]);
});

// ============================================================
//  8. swap_columns with a single quote in value
// ============================================================
await test("swap: values containing single quotes are handled correctly", async () => {
  const tbl = "quote_swap";
  await worker_do_work(["create_table", tbl, [["id","INTEGER",true,true,true,"",false], ["name","TEXT",false,false,false,"",false]]]);
  await worker_do_work(["insert_data", tbl, ["id","name"], ["1","O'Brien"]]);
  await worker_do_work(["insert_data", tbl, ["id","name"], ["2","Smith"]]);

  const reply = await worker_do_work(["swap_columns", tbl, "1", "2", "name"]);
  if (reply[0] === "error") throw new Error("swap with single quote failed: " + reply[1]);

  const row1 = await worker_do_work(["get_data", tbl, "id=1", ["name"]]);
  const row2 = await worker_do_work(["get_data", tbl, "id=2", ["name"]]);
  if (row1[0] === "error" || row2[0] === "error") throw new Error("get_data failed");
  if (row1[1][0][0] !== "Smith") throw new Error(`row1 expected Smith, got ${row1[1][0][0]}`);
  if (row2[1][0][0] !== "O'Brien") throw new Error(`row2 expected O'Brien, got ${row2[1][0][0]}`);
  await worker_do_work(["delete_table", tbl]);
});

// ============================================================
//  9. Stress test: 100 rows and a long string
// ============================================================
await test("stress: insert 100 rows and verify count", async () => {
  const tbl = "hundred";
  await worker_do_work(["create_table", tbl, [["id","INTEGER",true,true,true,"",false], ["n","INTEGER",false,false,false,"",false]]]);
  for (let i = 1; i <= 100; i++) {
    const reply = await worker_do_work(["insert_data", tbl, ["id","n"], [String(i), String(i)]]);
    if (reply[0] === "error") throw new Error(`insert ${i} failed: ${reply[1]}`);
  }
  const check = await worker_do_work(["get_data", tbl, "", ["id"]]);
  if (check[0] === "error") throw new Error("get_data failed: " + check[1]);
  if (check[1].length !== 100) throw new Error(`expected 100 rows, got ${check[1].length}`);
  await worker_do_work(["delete_table", tbl]);
});

await test("stress: long text value (10k characters)", async () => {
  const tbl = "long_text";
  await worker_do_work(["create_table", tbl, [["id","INTEGER",true,true,true,"",false], ["text","TEXT",false,false,false,"",false]]]);
  const longString = "a".repeat(10000);
  const reply = await worker_do_work(["insert_data", tbl, ["text"], [longString]]);
  if (reply[0] === "error") throw new Error("insert long string failed: " + reply[1]);
  const check = await worker_do_work(["get_data", tbl, "", ["text"]]);
  if (check[0] === "error") throw new Error("get_data failed: " + check[1]);
  if (check[1][0][0] !== longString) throw new Error("retrieved string doesn't match (length: " + check[1][0][0].length + ")");
  await worker_do_work(["delete_table", tbl]);
});

// ============================================================
// 10. Worker initialisation with invalid database name
//      (requires a second worker – not part of the main suite)
// ============================================================
// This can't run inside the same worker because we already initialized.
// Here's a manual snippet you can paste into the browser console:
/*
  const testWorker = new Worker('test_worker.js', { type: 'module' });
  testWorker.onmessage = (e) => {
    if (e.data[0] === "ready") {
      testWorker.postMessage(["initialize", ""]);  // empty db name
    } else {
      console.log("Response from empty-name init:", e.data);
    }
  };
*/
// To test a very long database name:
/*
  const longName = "a".repeat(1000);
  const testWorker2 = new Worker('test_worker.js', { type: 'module' });
  testWorker2.onmessage = (e) => {
    if (e.data[0] === "ready") {
      testWorker2.postMessage(["initialize", longName]);
    } else {
      console.log("Response from long-name init:", e.data);
    }
  };
*/
