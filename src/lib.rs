// indexeddb_from_rust lib.rs

//use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

mod currdb_config_mod;
mod currdb_currency_mod;
mod currdb_mod;
mod idbr_imports_mod;
mod idbr_mod;
mod page_main_mod;
mod page_units_mod;
mod utils_mod;
mod web_sys_mod;

use crate::web_sys_mod as w;

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
    crate::idbr_mod::check_browser_capability();

    //async block
    wasm_bindgen_futures::spawn_local(async {
        crate::currdb_mod::init_upgrade_currdb().await;
        crate::page_main_mod::page_main().await;
    });

    // return
    Ok(())
}
