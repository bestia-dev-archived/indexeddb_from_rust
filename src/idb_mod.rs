// idb_mod.rs

use crate::web_sys_mod as w;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
// use crate::web_sys_mod::*;

use crate::idbjs_mod as idbjs;

// Imported functions from javascript recognize only JsValue and &str
// I want to isolate this functions into idbjs_mod and they are used only by the idb_mod

pub fn check_browser_capability() {
    idbjs::check_browser_capability();
}

/// init_upgrade_db_currency_rates will create the database and call upgrade_db()
pub async fn init_upgrade_db_currency_rates() {
    idbjs::init_upgrade_db("currency_rates", 2).await.unwrap();
}

/// upgrade_db is called from javascript init_upgrade_db()
#[wasm_bindgen]
#[allow(dead_code)]
pub fn upgrade_db(db: JsValue, old_version: JsValue, new_version: JsValue, transaction: JsValue) {
    let db = Database::from(db);
    let tx = Transaction::from(transaction);
    let old_version = unwrap!(old_version.as_f64()) as i32;
    let new_version = unwrap!(new_version.as_f64()) as i32;
    w::debug_write(&format!(
        "upgrade_db from v{} to v{}",
        old_version, new_version
    ));

    if old_version <= 0 {
        upgrade_from_v00_to_v01(&db);
    }
    if old_version <= 1 {
        upgrade_from_v01_to_v02(&db, &tx);
    }
}

fn upgrade_from_v00_to_v01(db: &Database) {
    w::debug_write("upgrade_from_v00_to_v01");
    db.create_object_store("currency");
}

fn upgrade_from_v01_to_v02(db: &Database, tx: &Transaction) {
    w::debug_write("upgrade_from_v01_to_v02");
    db.create_object_store("config");
    w::debug_write("after create_object_store");
    let cfg = tx.get_object_store("config");

    w::debug_write("after get_object_store_from_transaction");
    cfg.put("base_currency", "EUR");
    cfg.put("quote_currency", "USD");
    cfg.put("rate", "1.21");
    cfg.put("date_fetch", "none");
}

pub async fn put_key_value(
    db_name: &str,
    store: &str,
    key: &str,
    value: &str,
) -> Result<(), JsValue> {
    // return
    idbjs::put_key_value(db_name, store, key, value).await
}

pub async fn get_key_value(db_name: &str, store: &str, key: &str) -> String {
    // return
    unwrap!(idbjs::get_key_value(db_name, store, key).await.as_string())
}

struct Database {
    db: JsValue,
}
impl Database {
    fn create_object_store(&self, store_name: &str) {
        idbjs::create_object_store(self.db.clone(), store_name);
    }
}
/// Database from JsValue
impl From<JsValue> for Database {
    /// Database from JsValue
    fn from(db: JsValue) -> Self {
        // return
        Database { db }
    }
}
struct Transaction {
    tx: JsValue,
}
impl Transaction {
    fn get_object_store(&self, store_name: &str) -> ObjectStoreInsideTransaction {
        let tx_ob_st = idbjs::get_object_store_from_transaction(&self.tx, store_name);
        let tx_ob_st = ObjectStoreInsideTransaction::from(tx_ob_st);
        // return
        tx_ob_st
    }
}
/// Transaction from JsValue
impl From<JsValue> for Transaction {
    /// Transaction from JsValue
    fn from(tx: JsValue) -> Self {
        // return
        Transaction { tx }
    }
}
/// Transaction into JsValue
impl From<Transaction> for JsValue {
    /// Transaction into JsValue
    fn from(tx: Transaction) -> JsValue {
        tx.tx
    }
}

struct ObjectStoreInsideTransaction {
    tx_ob_st: JsValue,
}
impl ObjectStoreInsideTransaction {
    pub fn put(&self, key: &str, value: &str) {
        idbjs::transaction_object_store_put(self.tx_ob_st.clone(), key, value);
    }
}
/// ObjectStoreInsideTransaction from JsValue
impl From<JsValue> for ObjectStoreInsideTransaction {
    /// ObjectStoreInsideTransaction from JsValue
    fn from(tx_ob_st: JsValue) -> Self {
        // return
        ObjectStoreInsideTransaction { tx_ob_st }
    }
}
/// ObjectStoreInsideTransaction into JsValue
impl From<ObjectStoreInsideTransaction> for JsValue {
    /// ObjectStoreInsideTransaction into JsValue
    fn from(tx: ObjectStoreInsideTransaction) -> JsValue {
        tx.tx_ob_st.clone()
    }
}
