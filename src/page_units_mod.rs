// page_units_mod.rs

use std::ops::Index;

use unwrap::unwrap;
use wasm_bindgen::prelude::*;
//use wasm_bindgen::{JsCast, JsValue};
use serde_json::Value;
use wasm_bindgen::JsCast;

use crate::utils_mod as ut;
use crate::web_sys_mod as w;
use crate::{currdb_config_mod, on_click};

/// fetch and inject HTML fragment into index.html/div_for_wasm_html_injecting
pub async fn page_units() {
    // fetch mani_page.html and inject it
    let resp_body_text = w::fetch_response("pages/page_units.html").await;
    // only the html inside the <body> </body>
    let (html_fragment, _new_pos_cursor) = unwrap!(ut::get_delimited_text(
        &resp_body_text,
        0,
        "<body>",
        "</body>"
    ));
    // get template
    let (template, _new_pos_cursor) = unwrap!(ut::get_delimited_text(
        &html_fragment,
        0,
        "<!--use as template-->",
        "<!--end use as template-->",
    ));
    // remove template from html_fragment
    let html_fragment = ut::get_text_without_delimited_fragment(
        &html_fragment,
        0,
        "<!--use as template-->",
        "<!--end use as template-->",
    );
    // remove ignore as template
    let html_fragment = ut::get_text_without_delimited_fragment(
        &html_fragment,
        0,
        "<!--ignore as template-->",
        "<!--end ignore as template-->",
    );
    w::set_inner_html("div_for_wasm_html_injecting", &html_fragment);

    let mut html_list = String::with_capacity(1000);
    // repeat the template with data from indexed db in template inside div_list_grid

    // read from indexed db row by row


    for _x in 0..5 {
        let template_with_data = ut::replace_wt_placeholder(&template, "wt_unit", "111");
        let template_with_data = ut::replace_wt_placeholder(&template_with_data, "wt_name", "222");
        let template_with_data = ut::replace_wt_placeholder(&template_with_data, "wt_rate", "333");
        html_list.push_str(&template_with_data);
        w::debug_write(&template_with_data);
    }
    w::debug_write(&html_list);
    w::set_inner_html("div_list_grid", &html_list);

    // event handlers
    // how to delete all old event handlers?
    on_click!("div_back", div_back_on_click);
}

/// go to page_main
pub fn div_back_on_click(_element_id: &str) {
    wasm_bindgen_futures::spawn_local(async {
        crate::page_main_mod::page_main().await;
    });
}
