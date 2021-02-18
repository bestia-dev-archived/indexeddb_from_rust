// page_units_mod.rs

use std::ops::Index;

use unwrap::unwrap;
use wasm_bindgen::prelude::*;
//use wasm_bindgen::{JsCast, JsValue};
use serde_json::Value;
use wasm_bindgen::JsCast;

use crate::currdb_currency_mod::*;
use crate::web_sys_mod as w;
use crate::{currdb_config_mod, on_click};
use crate::{idbr_mod, utils_mod as ut};

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
    use crate::currdb_mod::{Databases, ObjectStores};
    use crate::idbr_mod as idb;
    use strum::AsStaticRef;
    let db = idb::Database::use_db(&Databases::Currdb.as_static()).await;
    let cursor = db.get_cursor(ObjectStores::Currency.as_static()).await;
    loop {
        let key = cursor.get_key();
        let key: String = unwrap!(serde_wasm_bindgen::from_value(key));
        let value = cursor.get_value();
        let fields: CurrencyFields = unwrap!(serde_wasm_bindgen::from_value(value));

        let template_with_data = ut::replace_wt_placeholder(&template, "wt_unit", &key);
        let template_with_data =
            ut::replace_wt_placeholder(&template_with_data, "wt_name", &fields.name);
        let template_with_data = ut::replace_wt_placeholder(
            &template_with_data,
            "wt_rate",
            &format!("{:.3}", fields.rate),
        );

        html_list.push_str(&template_with_data);
        if cursor.next().await.is_none() {
            break;
        }
    }
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
