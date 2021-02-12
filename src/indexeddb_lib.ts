// This import paths must be defined in tsconfig.json including the .d.ts types
// because the difference of folder structure and url paths
import * as idb from '/indexeddb_from_rust/idb/index.js';
import * as rust from '/indexeddb_from_rust/pkg/indexeddb_from_rust.js';

/// does the browser have indexedDB
export function check_browser_capability(){
    if (!window.indexedDB) {
        console.log("NO, IndexedDB is not available.");
    } else {
        console.log("Yes, Indexeddb is available");
    }
}

/// init db with upgrade code, returns a promise
/// it must be the first command for indexeddb and it must have enough time to upgrade
/// before later commands
export async function init_upgrade_db(db_name:string, version:number) {
    console.log("init_upgrade_db");
    let db = await idb.openDB(db_name,version, {
        upgrade(db, oldVersion, newVersion, transaction) {
            rust.upgrade_db(db, oldVersion, newVersion, transaction);
        },
    });
    return db;
}

/// create object store
export async function create_object_store(db:idb.IDBPDatabase,store_name:string) {
    db.createObjectStore(store_name);
}

/// get object store from transaction
export function get_object_store_from_transaction(tx:idb.IDBPTransaction<unknown, string[], "versionchange">,store_name:string) {
    let object_store = tx.objectStore(store_name);
    return object_store;
}

// put inside a transaction_object_store
export function transaction_object_store_put(tx_ob_st: any, key:string, value:string) {
    tx_ob_st.put(value,key);
}

/// use db returns a promise. 
/// I hope this is fast, because I call it often.
/// I hope it is reused and don't makes millions of unclosed db objects in memory
export async function use_db(db_name:string) {
    let db = await idb.openDB(db_name);
    return db;
}

/// add key-value in a store
export async function add_key_value(db_name:string, store:string, key:string, value:string){
    console.log("add");
    let db = await use_db(db_name);
    db.add(store, value, key);
}

/// put key-value in a store (upsert)
export async function put_key_value(db_name:string, store:string, key:string, value:string){
    let db = await use_db(db_name);
    db.put(store, value, key);
}

/// get key-value in a store 
export async function get_key_value(db_name:string, store:string, key:string){
    console.log("get");
    let db = await use_db(db_name);
    const value = await db.get(store, key);
    return value;
}

/// open transaction returns a Promise
export async function transaction(db_name:string, store:string){
    console.log("transaction");
    let db = await use_db(db_name);
    const tx = db.transaction(store, 'readwrite');
    return tx;
}

