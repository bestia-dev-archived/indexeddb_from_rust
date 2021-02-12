// idb_mod.rs

use crate::web_sys_mod as w;
use js_sys::Number;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
// use wasm_bindgen::JsCast;

// use crate::web_sys_mod::*;

// rustfmt 1.4.25 bug removes the word async from extern "C". Skip this.
#[rustfmt::skip]
#[wasm_bindgen(raw_module = "/indexeddb_from_rust/js/indexeddb_lib.js")]
extern "C" {
    pub(crate) fn check_browser_capability();
    /// open db with name currency_rates and returns the idb.IDBPDatabase as JsValue
    #[wasm_bindgen(catch)]
    pub(crate) async fn init_upgrade_db(db_name: &str,version:u32) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch)]
    pub(crate) async fn add_key_value(db_name: &str, store: &str, key: &str, value: &str, ) -> Result<(), JsValue>;
    /// currency_rates: idb.IDBPDatabase 
    #[wasm_bindgen(catch)]
    pub(crate) async fn put_key_value(db_name: &str, store: &str, key: &str, value: &str, ) -> Result<(), JsValue>;
    /// currency_rates: idb.IDBPDatabase 
    pub(crate) async fn get_key_value(db_name: &str, store: &str, key: &str, ) -> JsValue;
    pub(crate) fn transaction(db:&JsValue, store:String) ->JsValue;
    pub(crate) fn create_object_store(db:&JsValue,store_name:&str);
    pub(crate) fn get_object_store_from_transaction(tx:&JsValue,store_name:&str) -> JsValue;
    pub(crate) fn transaction_object_store_put(tx_ob_st: &JsValue, key:&str, value:&str);
}

/// init_upgrade_db_currency_rates will create the database and call upgrade_db()
pub async fn init_upgrade_db_currency_rates() {
    init_upgrade_db("currency_rates", 2).await.unwrap();
}

/// upgrade_db is called from javascript init_upgrade_db()
#[wasm_bindgen]
#[allow(dead_code)]
pub fn upgrade_db(
    db: &JsValue,
    old_version: &JsValue,
    new_version: &JsValue,
    transaction: &JsValue,
) {
    let old_version = unwrap!(old_version.as_f64()) as i32;
    let new_version = unwrap!(new_version.as_f64()) as i32;
    w::debug_write(&format!(
        "upgrade_db from v{} to v{}",
        old_version, new_version
    ));

    if old_version <= 0 {
        upgrade_from_v00_to_v01(db);
    }
    if old_version <= 1 {
        upgrade_from_v01_to_v02(db, transaction);
    }
}

fn upgrade_from_v00_to_v01(db: &JsValue) {
    w::debug_write("upgrade_from_v00_to_v01");
    create_object_store(db, "currency");
}

fn upgrade_from_v01_to_v02(db: &JsValue, tx: &JsValue) {
    w::debug_write("upgrade_from_v01_to_v02");
    create_object_store(db, "config");
    w::debug_write("after create_object_store");
    let tx_ob_st = &get_object_store_from_transaction(tx, "config");
    w::debug_write("after get_object_store_from_transaction");
    transaction_object_store_put(tx_ob_st, "base_currency", "EUR");
    transaction_object_store_put(tx_ob_st, "quote_currency", "USD");
    transaction_object_store_put(tx_ob_st, "rate", "1.21");
    transaction_object_store_put(tx_ob_st, "date_fetch", "none");
}
