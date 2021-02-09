// idb_mod.rs

use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};

use crate::web_sys_mod::*;

// rustfmt 1.4.25 bug removes the word async from extern "C". Skip this.
#[rustfmt::skip]
#[wasm_bindgen(raw_module = "/indexeddb_from_rust/js/indexeddb_lib.js")]
extern "C" {
    pub(crate) fn check_browser_capability();
    #[wasm_bindgen(catch)]
    fn js_open_db() -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch)]
    pub(crate) fn add_key_value( db1: &JsValue, store: String, key: String, value: String, ) -> Result<(), JsValue>;
    // db1: idb.IDBPDatabase 
    #[wasm_bindgen(catch)]
    pub(crate) fn put_key_value( db1: &JsValue, store: String, key: String, value: String, ) -> Result<(), JsValue>;
    // db1: idb.IDBPDatabase 
    pub(crate) async fn get_key_value(db1: &JsValue, store: String, key: String, ) -> JsValue;
}

/// init and open db
pub async fn init_and_open_db() -> JsValue {
    // js_open_db returns a promise
    let db1_promise = js_open_db().unwrap();
    // cast from JsValue to the expected type
    let db1_promise = unwrap!(db1_promise.dyn_into::<js_sys::Promise>());
    // db1 is promise. transform into future
    let db1 = wasm_bindgen_futures::JsFuture::from(db1_promise)
        .await
        .unwrap();

    debug_write(&format!("{:?}", db1));
    // return
    db1
}
