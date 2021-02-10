// main_page_mod.rs

use unwrap::unwrap;
use wasm_bindgen::prelude::*;
//use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::JsCast;

use crate::idb_mod as idb;
use crate::web_sys_mod as w;
use crate::on_click;

/// fetch and inject HTML fragment into index.html/div_for_wasm_html_injecting
pub async fn main_page() {
    // fetch mani_page.html and inject it
    let resp_body_text = w::fetch_response("pages/main_page.html").await;
    // only the html inside the <body> </body>
    let html_fragment = w::between_body_tag(&resp_body_text);
    w::set_inner_html("div_for_wasm_html_injecting", &html_fragment);
    // event handlers
    // how to delete all old event handlers?
    on_click!("div_1", div_cell_value_on_click);
    on_click!("div_2", div_cell_value_on_click);
    on_click!("div_3", div_cell_value_on_click);
    on_click!("div_4", div_cell_value_on_click);
    on_click!("div_5", div_cell_value_on_click);
    on_click!("div_6", div_cell_value_on_click);
    on_click!("div_7", div_cell_value_on_click);
    on_click!("div_8", div_cell_value_on_click);
    on_click!("div_9", div_cell_value_on_click);
    on_click!("div_0", div_cell_value_on_click);
    on_click!("div_dot", div_cell_value_on_click);

    on_click!("div_backspace", div_backspace_on_click);
    on_click!("div_clear", div_c_on_click);

    on_click!("span_reload", span_reload_on_click);
}

/// reload json from floatrates.com and save to indexeddb
pub fn span_reload_on_click(_element_id: &str) {
    wasm_bindgen_futures::spawn_local(async {
        let resp_body_text = w::fetch_response("http://www.floatrates.com/daily/eur.json").await;
        // Parse the string of data into serde_json::Value.
        let v: serde_json::Value = unwrap!(serde_json::from_str(&resp_body_text));
        // it is not an array !
        let v = unwrap!(v.as_object());

        // TODO: use one big transaction instead of many small transactions
        let db1 = idb::open_db("db1").await.unwrap();
        for x in v {
            idb::put_key_value(
                &db1,
                "currency".to_owned(),
                x.0.to_uppercase().to_owned(),
                unwrap!(x.1["name"].as_str()).to_owned(),
            )
            .await
            .unwrap();

            w::debug_write(&format!(
                "{} {} {}",
                x.0,
                unwrap!(x.1["name"].as_str()),
                unwrap!(x.1["rate"].as_f64())
            ));
        }
    });
}

/// event handler
pub fn div_cell_value_on_click(element_id: &str) {
    let text = w::get_text(element_id);
    w::set_text(
        "div_input",
        &format!("{}{}", w::get_text("div_input"), text),
    );
    convert();
}

/// event handler
pub fn div_backspace_on_click(_element_id: &str) {
    let mut text = w::get_text("div_input");
    if text.len() > 4 {
        text.pop().unwrap();
        w::set_text("div_input", &text);
        convert();
    }
}

/// event handler
pub fn div_c_on_click(_element_id: &str) {
    w::set_text("div_input", "EUR: ");
    w::set_text("div_output", "USD: ");
}

fn convert() {
    let rate = w::get_text("div_toolbar").parse::<f64>().unwrap();
    let input = w::get_text("div_input").parse::<f64>().unwrap();
    let output = format!("USD: {:.3}", input * rate);
    w::set_text("div_output", &output);
}
