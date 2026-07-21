(async () => {
  try {
    const root = await navigator.storage.getDirectory();
    console.log("OPFS root handle:", root);
  } catch (e) {
    console.error("OPFS NOT AVAILABLE:", e);
  }
})();
import init, { LiveForever } from '../pkg/part4.js';
await init();
let db_manager = null;   // will hold the instance after successful creation
// each handler takes the raw message array and returns the payload to send back
const handlers = {
  initialize: async (msg) => {
    // msg[1] is the database name (String)
    db_manager = await LiveForever.new(msg[1]);
    return "ok";
  },
  drop_table: async () => {
    await db_manager.drop_table();
    return "droppety woppetied all of it";
  },
  check_table: (msg) => db_manager.check_table(msg[1]),
  get_data: (msg) => db_manager.get_data(msg[1], msg[2], msg[3]),
  insert_data: async (msg) => {
    await db_manager.insert_data(msg[1], msg[2]);
    return `inserted: ${msg[1]} and ${msg[2]}`;
  },
  // msg: ["edit_row", table_name, row_id, column, new_value]
  edit_row: async (msg) => { //log('edit', await ask(["edit_row", table, id, col, val]));
    /*
    pub async fn edit_col_in_row(
        &self,
        table_name: String,
        row_id: String,
        column: String,
        value: String,
    ) */
    await db_manager.edit_col_in_row(msg[1], msg[2], msg[3], msg[4]);
    return "ok";
  },
  // msg: ["delete_row", table_name, row_id]
  delete_row: async (msg) => {
    await db_manager.delete_row(msg[1], msg[2]);
    return "ok";
  },
  // msg: ["swap_columns", table_name, row_id_1, row_id_2, column]
  swap_columns: async (msg) => {
    await db_manager.swap_columns(msg[1], msg[2], msg[3], msg[4]);
    return "ok";
  },
};

self.onmessage = async (e) => {
  const msg = e.data;
  const command = msg[0];
  const handler = handlers[command];

  if (!handler) {
    self.postMessage(["error", "unknown command"]);
    return;
  }
  if (command !== "initialize" && !db_manager) {
    self.postMessage(["error", "Database not initialised. Send an 'initialize' command first."]);
    return;
  }

  try {
    const data = await handler(msg);
    self.postMessage([command, data]);
  } catch (err) {
    self.postMessage(["error", err.toString()]);
  }
};
console.log("loaded worker js");
self.postMessage(["ready"]);
