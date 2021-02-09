// indexeddb_from_rust lib.rs

//use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

mod idb_mod;
mod web_sys_mod;

use idb_mod::*;
use web_sys_mod::*;

#[wasm_bindgen(start)]
/// To start the Wasm application, wasm_bindgen runs this functions
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();
    // write the app version just for debug purposes
    debug_write(&format!(
        "indexeddb_from_rust v{}",
        env!("CARGO_PKG_VERSION")
    ));
    check_browser_capability();
    //async block
    wasm_bindgen_futures::spawn_local(async {
        let db1 = init_and_open_db().await;

        put_key_value(
            &db1,
            "currency".to_owned(),
            "EUR".to_owned(),
            "euro".to_owned(),
        )
        .unwrap();
        put_key_value(
            &db1,
            "currency".to_owned(),
            "USD".to_owned(),
            "dollar".to_owned(),
        )
        .unwrap();
        put_key_value(
            &db1,
            "currency".to_owned(),
            "HRK".to_owned(),
            "kuna".to_owned(),
        )
        .unwrap();

        let text = get_key_value(&db1, "currency".to_owned(), "HRK".to_owned()).await;
        debug_write(&format!("{:?}", text));
    });
    // return
    Ok(())
}
