// idb_mod.rs

// use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
// use wasm_bindgen::JsCast;

// use crate::web_sys_mod::*;

// rustfmt 1.4.25 bug removes the word async from extern "C". Skip this.
#[rustfmt::skip]
#[wasm_bindgen(raw_module = "/indexeddb_from_rust/js/indexeddb_lib.js")]
extern "C" {
    pub(crate) fn check_browser_capability();
    /// open db with name db1 and returns the idb.IDBPDatabase as JsValue
    #[wasm_bindgen(catch)]
    pub(crate) async fn open_db(db_name:String) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch)]
    pub(crate) async fn add_key_value( db1: &JsValue, store: String, key: String, value: String, ) -> Result<(), JsValue>;
    /// db1: idb.IDBPDatabase 
    #[wasm_bindgen(catch)]
    pub(crate) async fn put_key_value( db1: &JsValue, store: String, key: String, value: String, ) -> Result<(), JsValue>;
    /// db1: idb.IDBPDatabase 
    pub(crate) async fn get_key_value(db1: &JsValue, store: String, key: String, ) -> JsValue;
    pub(crate) fn transaction(db1:&JsValue, store:String) ->JsValue;
}
