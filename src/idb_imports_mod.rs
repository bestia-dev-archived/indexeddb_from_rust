// idb_imports_mod

// Imported functions from javascript/typescript idb_exports.ts.
// Javascript recognizes only JsValue and &str.
// I want to isolate this functions because they are used only by the idb_mod.
// idb_mod should have fully rust code and types.

use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

// rustfmt 1.4.25 bug removes the word async from extern "C". Skip this.
#[rustfmt::skip]
#[wasm_bindgen(raw_module = "/indexeddb_from_rust/js/idb_exports.js")]
extern "C" {
    pub(crate) fn check_browser_capability();
    /// open db with name currdb and returns the idb.IDBPDatabase as JsValue
    #[wasm_bindgen(catch)]
    pub(crate) async fn init_upgrade_db(db_name: &str,version:u32,rust_closure_for_upgrade: &Closure<dyn Fn(JsValue, JsValue, JsValue, JsValue)>) -> Result<JsValue, JsValue>;
    pub(crate) async fn use_db(db_name:&str)->JsValue;
    #[wasm_bindgen(catch)]
    pub(crate) async fn add_key_value(db_name: &str, store_name: &str, key: &str, value: &str, ) -> Result<(), JsValue>;
    #[wasm_bindgen(catch)]
    pub(crate) async fn put_key_value(db_name: &str, store_name: &str, key: &str, value: &str, ) -> Result<(), JsValue>;
    pub(crate) async fn get_key_value(db_name: &str, store_name: &str, key: &str, ) -> JsValue;
    pub(crate) fn transaction_open(db:&JsValue) ->JsValue;
    pub(crate) fn create_object_store(db:JsValue,store_name:&str);
    pub(crate) fn get_object_store_from_transaction_versionchange(tx:&JsValue,store_name:&str) -> JsValue;
    pub(crate) fn get_object_store_from_transaction_readwrite(tx:&JsValue,store_name:&str) -> JsValue;
    pub(crate) fn transaction_object_store_put(tx_ob_st: JsValue, key:&str, value:&str);
    pub(crate) fn transaction_object_store_put_js_value(tx_ob_st: JsValue, key:&str, value:&JsValue);
    #[wasm_bindgen(catch)]
    pub(crate) async fn db_put_key_value(db:&JsValue, store_name:&str, key:&str, value:&str) -> Result<(), JsValue>;
    pub(crate) fn transaction_close(tx:&JsValue);
    pub(crate) async fn cursor(db:&JsValue, store_name:&str)->JsValue;
    pub(crate) async fn cursor_continue(cursor:&JsValue)->JsValue;
    pub(crate) fn cursor_key(cursor:&JsValue)->JsValue;
    pub(crate) fn cursor_value(cursor:&JsValue)->JsValue;
}
