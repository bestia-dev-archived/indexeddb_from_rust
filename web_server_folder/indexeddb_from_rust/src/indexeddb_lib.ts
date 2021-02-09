import * as idb from '/indexeddb_from_rust/idb/index.js';

/// does the browser have indexedDB
export function check_browser_capability(){
    if (!window.indexedDB) {
        console.log("NO, IndexedDB is not available.");
    } else {
        console.log("Yes, Indexeddb is available");
    }
}

/// open db with upgrade code
export async function open_db() {
    console.log("open_db");
    // Failed to resolve module specifier `idb`
    await idb.openDB('db1', 1, {
    upgrade(db) {
        console.log("upgrade(db)");
      db.createObjectStore('currency');
    },
  });
}

/// short way for openDB
const db1 = idb.openDB("db1", 1);

/// add key-value in a store
export async function add_key_value(store:string, key:string, value:string){
    console.log("add");
    (await db1).add(store, value,key )
}

