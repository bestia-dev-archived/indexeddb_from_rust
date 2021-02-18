// currdb_currency_mod

// the store Currency in indexeddb database currdb

use crate::currdb_mod::{Databases, ObjectStores};
use crate::idbr_mod as idb;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use strum::AsStaticRef;
use unwrap::unwrap;

#[derive(Serialize, Deserialize)]
/// fields in the value column of key-value currency
pub struct CurrencyFields {
    pub name: String,
    pub rate: f64,
}

pub async fn fill_currency_store(json_map_string_value: &Map<String, Value>) {
    let db = idb::Database::use_db(&Databases::Currdb.as_static()).await;
    let tx = db.transaction_open();
    let store = tx.get_object_store_readwrite(&ObjectStores::Currency.as_static());
    for string_value in json_map_string_value {
        let iso_code = string_value.0.to_uppercase();
        // the value will have 2 columns: name(string) and rate(f64)
        // indexed_db would serialize rust object to json and then in javascript json to javascript object and then store
        // I will use only strings. The conversion to/from string will be in QVS20 format for compact, ubiquitous and fast conversion
        let name = unwrap!(string_value.1["name"].as_str()).to_owned();
        let rate = unwrap!(string_value.1["rate"].as_f64());
        let new_value = CurrencyFields { name, rate };
        let js_value = serde_wasm_bindgen::to_value(&new_value).unwrap();
        store.put_js_value(&iso_code, &js_value);
    }
    tx.close();
}
