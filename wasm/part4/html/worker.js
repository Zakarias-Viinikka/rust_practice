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

self.onmessage = async (e) => {
  const msg = e.data;
  try {
    if (msg[0] === "initialize") {
      // msg[1] is the database name (String)
      db_manager = await LiveForever.new(msg[1]);
      self.postMessage(["initialize", "ok"]);
    }
    else if (!db_manager) {
      // If we haven't initialised yet, reject all other commands
      self.postMessage(["error", "Database not initialised. Send a 'initialize' command first."]);
    } else if (msg[0] === "drop_table") {
      const data = await db_manager.drop_table();
      self.postMessage(["drop_table", "droppety woppetied all of it"]);
    } else if (msg[0] === "check_table") {
      const data = await db_manager.check_table(msg[1]);
      self.postMessage(["check_table", data]);
    } else if (msg[0] === "get_data") {
      const data = await db_manager.get_data(msg[1], msg[2], msg[3]);
      self.postMessage(["get_data", data]);
    } else if (msg[0] === "insert_data") {
      await db_manager.insert_data(msg[1], msg[2]);
      self.postMessage(["insert_data", `inserted: ${msg[1]} and ${msg[2]}`]);
    } else {
      self.postMessage(["error", "unknown command"]);
    }
  } catch (err) {
    self.postMessage(["error", err.toString()]);
  }
};

console.log("loaded worker js");

self.postMessage(["ready"]);
