// main_page_mod.rs

use unwrap::unwrap;
use wasm_bindgen::prelude::*;
//use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::JsCast;

use crate::idb_mod as idb;
use crate::on_click;
use crate::web_sys_mod as w;

/// fetch and inject HTML fragment into index.html/div_for_wasm_html_injecting
pub async fn main_page() {
    // fetch mani_page.html and inject it
    let resp_body_text = w::fetch_response("pages/main_page.html").await;
    // only the html inside the <body> </body>
    let html_fragment = w::between_body_tag(&resp_body_text);
    w::set_inner_html("div_for_wasm_html_injecting", &html_fragment);
    // event handlers
    // how to delete all old event handlers?
    on_click!("div_1", div_cell_on_click);
    on_click!("div_2", div_cell_on_click);
    on_click!("div_3", div_cell_on_click);
    on_click!("div_4", div_cell_on_click);
    on_click!("div_5", div_cell_on_click);
    on_click!("div_6", div_cell_on_click);
    on_click!("div_7", div_cell_on_click);
    on_click!("div_8", div_cell_on_click);
    on_click!("div_9", div_cell_on_click);
    on_click!("div_0", div_cell_on_click);

    on_click!("div_dot", div_cell_dot_on_click);
    on_click!("div_backspace", div_backspace_on_click);
    on_click!("div_clear", div_c_on_click);

    on_click!("span_reload", span_reload_on_click);
}

/// reload json from floatrates.com and save to indexeddb
pub fn span_reload_on_click(_element_id: &str) {
    wasm_bindgen_futures::spawn_local(async {
        let resp_body_text = w::fetch_response("http://www.floatrates.com/daily/eur.json").await;
        // there is 8 special characters I want to avoid
        let resp_body_text = resp_body_text
            .replace(r"\u00F3", "ó")
            .replace(r"\t", "")
            .replace(r"\u02bb", "ʻ")
            .replace(r"\u00e3", "ã")
            .replace(r"\u00e9", "é")
            .replace(r"\u00ed", "í");
        // Parse the string of data into serde_json::Value.
        let v: serde_json::Value = unwrap!(serde_json::from_str(&resp_body_text));
        // it is not an array !
        let json_map_string_value = unwrap!(v.as_object());

        let db = idb::Database::use_db("currdb").await;
        let tx = db.transaction();
        let store = tx.get_object_store_readwrite("currency");
        for string_value in json_map_string_value {
            // the value will have 2 columns: name(string) and rate(f64)
            // indexed_db would serialize rust object to json and then in javascript json to javascript object and then store
            // I will use only strings. The conversion to/from string will be in QVS20 format for compact, ubiquitous and fast conversion
            let name = unwrap!(string_value.1["name"].as_str());
            let rate = unwrap!(string_value.1["rate"].as_f64());
            let qvs20_value = crate::qvs20_currency_mod::serialize_qvs20_single_row(name, rate);
            store.put(&string_value.0.to_uppercase(), &qvs20_value);
        }
        tx.close();
    });
}

/// event handler for 0-9 numeric cells
/// if input is only 0 then remove 0
pub fn div_cell_on_click(element_id: &str) {
    let mut input = w::get_text("div_input_number");
    let text = w::get_text(element_id);
    if input == "0" {
        input.clear();
    }
    input.push_str(&text);
    w::set_text("div_input_number", &input);
    convert();
}

/// decimal dot can be used only once
pub fn div_cell_dot_on_click(element_id: &str) {
    let mut input = w::get_text("div_input_number");
    if !input.contains(".") {
        input.push('.');
        w::set_text("div_input_number", &input);
        convert();
    }
}

/// event handler
/// input must never be incorrect f64
/// instead of clearing to empty, it makes it 0 zero
pub fn div_backspace_on_click(_element_id: &str) {
    let mut input = w::get_text("div_input_number");
    input.pop().unwrap();
    if input.is_empty() {
        input.push('0');
    }
    w::set_text("div_input_number", &input);
    convert();
}

/// event handler
/// input must never be incorrect f64
/// clear makes it 0 zero
pub fn div_c_on_click(_element_id: &str) {
    w::set_text("div_input_number", "0");
    w::set_text("div_output_number", "0");
}

/// convert currency with exchange rates
/// input cannot never be incorrect f64
fn convert() {
    let rate = w::get_text("div_toolbar").parse::<f64>().unwrap();
    let input = w::get_text("div_input_number").parse::<f64>().unwrap();
    let output = format!("{:.3}", input * rate);
    w::set_text("div_output_number", &output);
}
