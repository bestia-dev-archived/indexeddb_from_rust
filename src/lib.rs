use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(raw_module = "/indexeddb_from_rust/js/indexeddb_lib.js")]
extern "C" {
    fn check_browser_capability();
    #[wasm_bindgen(catch)]
    fn open_db() -> Result<(), JsValue>;
    #[wasm_bindgen(catch)]
    fn add_key_value(store: String, key: String, value: String) -> Result<(), JsValue>;
}

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
    //unsafe {
    check_browser_capability();
    //}
    //async block
    wasm_bindgen_futures::spawn_local(async {
        //unsafe {
        open_db().unwrap();
        //wait for initialization

        //add("currency".to_owned(), "EUR".to_owned(), "euro".to_owned()).unwrap();
        //add("currency".to_owned(), "USD".to_owned(), "dollar".to_owned()).unwrap();
        //}
    });
    // return
    Ok(())
}

/// return window object
pub fn window() -> web_sys::Window {
    unwrap!(web_sys::window())
}

/// get element by id
pub fn get_element_by_id(element_id: &str) -> web_sys::Element {
    let document = unwrap!(window().document());
    unwrap!(document.get_element_by_id(element_id))
}

/// debug write into session_storage
pub fn debug_write(text: &str) {
    // writing to the console
    console::log_1(&JsValue::from_str(text));
}
