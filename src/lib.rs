// indexeddb_from_rust lib.rs

//use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

mod config_mod;
mod idb_mod;
mod idbjs_mod;
mod main_page_mod;
mod qvs20_currency_mod;
mod web_sys_mod;

use crate::idb_mod as idb;
use web_sys_mod as w;

#[wasm_bindgen(start)]
/// To start the Wasm application, wasm_bindgen runs this functions
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();
    // write the app version just for debug purposes
    w::debug_write(&format!(
        "indexeddb_from_rust v{}",
        env!("CARGO_PKG_VERSION")
    ));
    idb::check_browser_capability();

    //async block
    wasm_bindgen_futures::spawn_local(async {
        idb::init_upgrade_db_currency_rates().await;
        main_page_mod::main_page().await;
    });

    // return
    Ok(())
}
