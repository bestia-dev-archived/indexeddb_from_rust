// currdb_currency_mod

// the store Currency in indexeddb database currdb

use crate::currdb_mod::{Databases, ObjectStores};
use crate::idbr_mod as idbr;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use strum::AsStaticRef;
use unwrap::unwrap;
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize)]
/// fields in the value column of key-value currency
pub struct ValueStruct {
    pub name: String,
    pub rate: f64,
}

pub fn to_jsvalue(name: String, rate: f64) -> JsValue {
    let value = ValueStruct { name, rate };
    let js_value = serde_wasm_bindgen::to_value(&value).unwrap();
    // return
    js_value
}

/// put in ObjectStore
pub async fn put_inside_object_store(
    object_store: &idbr::ObjectStoreInsideTransaction,
    iso_code: String,
    name: String,
    rate: f64,
) {
    let js_value = to_jsvalue(name, rate);
    object_store.put_js_value(iso_code, &js_value);
}

/// put inside transaction
pub async fn put_inside_transaction(
    tx: &idbr::Transaction,
    iso_code: String,
    name: String,
    rate: f64,
) {
    let store = tx.get_object_store_readwrite(&ObjectStores::Currency.as_static());
    put_inside_object_store(&store, iso_code, name, rate).await;
}

pub async fn put_inside_database(iso_code: String, name: String, rate: f64) {
    let db = idbr::Database::use_db(&Databases::Currdb.as_static()).await;
    let tx = db.transaction_open();
    put_inside_transaction(&tx, iso_code, name, rate).await;
    tx.close();
}

pub async fn fill_currency_store(json_map_string_value: &Map<String, Value>) {
    let db = idbr::Database::use_db(&Databases::Currdb.as_static()).await;
    let tx = db.transaction_open();
    let store = tx.get_object_store_readwrite(&ObjectStores::Currency.as_static());
    for string_value in json_map_string_value {
        let iso_code = string_value.0.to_uppercase();
        let name = string_value.1["name"].to_string();
        let rate = unwrap!(string_value.1["rate"].as_f64());
        put_inside_object_store(&store, iso_code, name, rate).await;
    }
    tx.close();
}
