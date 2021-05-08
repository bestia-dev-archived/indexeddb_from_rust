// page_output_currency_mod.rs

//use std::ops::Index;

use unwrap::unwrap;
use wasm_bindgen::prelude::*;
//use wasm_bindgen::{JsCast, JsValue};
//use serde_json::Value;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

use crate::currdb_currency_mod::*;
use crate::on_click;
use crate::row_on_click;
use crate::web_sys_mod as w;
//use crate::idbr_mod;
use crate::utils_mod as ut;

/// fetch and inject HTML fragment into index.html/div_for_wasm_html_injecting
pub async fn page_output_currency() {
    // fetch page_unit.html and inject it
    let resp_body_text = w::fetch_response("pages/page_output_currency.html").await;
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

    // region: binding - read from config
    w::set_text(
        "div_units_base_currency",
        &crate::currdb_config_mod::get_base_currency().await,
    );
    // endregion: binding - read from config

    // region: read from indexed db row by row
    let mut html_list = String::with_capacity(1000);
    // repeat the template with data from indexed db in template inside div_list_layout
    use crate::currdb_mod::{Databases, ObjectStores};
    use crate::idbr_mod as idb;
    use strum::AsStaticRef;
    let db = idb::Database::use_db(&Databases::Currdb.as_static()).await;
    let cursor = db.get_cursor(ObjectStores::Currency.as_static()).await;
    // I cannot implement the iterator trait because it is sync, but I need async
    // a simple loop will be enough
    let mut row_number_counter: usize = 0;
    loop {
        let key = cursor.get_key();
        let key: String = unwrap!(serde_wasm_bindgen::from_value(key));
        let value = cursor.get_value();
        let fields: ValueStruct = unwrap!(serde_wasm_bindgen::from_value(value));

        let template_with_data =
            template.replace("row_number_counter", &row_number_counter.to_string());

        let template_with_data = ut::replace_wt_placeholder(&template_with_data, "wt_unit", &key);
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
        row_number_counter += 1;
    }
    // region: read from indexed db row by row
    w::set_inner_html("div_list_layout", &html_list);

    // region: event handlers
    on_click!("div_back", div_back_on_click);

    // handler for every row
    for i in 0..=row_number_counter {
        row_on_click!("div_unit_", i, unit_on_click);
    }
    // endregion: event handlers
}

/// go back to page_main
pub fn div_back_on_click(_element_id: &str) {
    spawn_local(async {
        crate::page_main_mod::page_main().await;
    });
}

/// unit is a field in the row of the list
pub fn unit_on_click(element_prefix: &str, row_number: usize) {
    let element_id = format!("{}{}", element_prefix, row_number);
    spawn_local(async move {
        w::debug_write(&format!("element_id: {}", element_id));
        let iso_code = w::get_text(&element_id);
        let iso_code = iso_code.clone();
        crate::currdb_config_mod::set_quote_currency(&iso_code).await;
        // find the new rate
        let rate = crate::currdb_currency_mod::get_rate(&iso_code).await;
        crate::currdb_config_mod::set_rate(&rate.to_string()).await;
        div_back_on_click("");
    });
}
