// page_modal_about_mod.rs

//use unwrap::unwrap;
use wasm_bindgen::prelude::*;
//use wasm_bindgen::{JsCast, JsValue};
//use serde_json::Value;
use wasm_bindgen::JsCast;
//use wasm_bindgen_futures::spawn_local;

use crate::on_click;
use crate::utils_mod as ut;
use crate::web_sys_mod as w;

/// fetch and inject HTML in index.html/div_for_modal
/// then open as modal
pub async fn page_modal_about() {
    // fetch page_main.html and inject it
    let resp_body_text = w::fetch_response("pages/page_modal_about.html").await;
    // only the html inside the <body> </body>
    let (html_fragment, _new_pos_cursor) =
        ut::get_delimited_text(&resp_body_text, 0, "<body>", "</body>").unwrap();
    w::set_inner_html("div_for_modal", &html_fragment);

    // region: binding - read from config
    w::set_text("span_version", &format!("v{}", env!("CARGO_PKG_VERSION")));
    // endregion: binding - read from config

    // region: event handlers
    on_click!("modal_about_close", modal_about_close_on_click);
    // endregion: event handlers
}

/// close the modal page
fn modal_about_close_on_click(_element_id: &str) {
    w::set_inner_html("div_for_modal", "");
}
