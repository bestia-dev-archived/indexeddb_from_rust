// currdb_currency_mod

// the store Currency in indexeddb database currdb

use crate::currdb_mod::{Databases, ObjectStores};
use crate::idbr_mod as idbr;
use crate::web_sys_mod as w;
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
    let jsvalue = serde_wasm_bindgen::to_value(&value).unwrap();
    // return
    jsvalue
}

pub fn from_jsvalue(jsvalue: JsValue) -> (String, f64) {
    let value_struct: ValueStruct = serde_wasm_bindgen::from_value(jsvalue).unwrap();
    // return
    (value_struct.name, value_struct.rate)
}

/// put in ObjectStore
pub async fn put_inside_object_store(
    object_store: &idbr::ObjectStoreInsideTransaction,
    iso_code: String,
    name: String,
    rate: f64,
) {
    let jsvalue = to_jsvalue(name, rate);
    object_store.put_jsvalue(iso_code, &jsvalue);
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

pub async fn fill_currency_store(base_currency: &str, json_map_string_value: &Map<String, Value>) {
    let db = idbr::Database::use_db(&Databases::Currdb.as_static()).await;
    let tx = db.transaction_open();
    let store = tx.get_object_store_readwrite(&ObjectStores::Currency.as_static());
    for string_value in json_map_string_value {
        let iso_code = string_value.0.to_uppercase();
        let name = string_value.1["name"].to_string();
        let rate = unwrap!(string_value.1["rate"].as_f64());
        put_inside_object_store(&store, iso_code, name, rate).await;
    }
    // put also base currency
    put_inside_object_store(
        &store,
        base_currency.to_owned(),
        base_currency.to_owned(),
        1.0,
    )
    .await;
    tx.close();
    w::debug_write(&format!("transaction end: {}", ""));
}

pub async fn get_rate(iso_code: &str) -> f64 {
    let db = idbr::Database::use_db(&Databases::Currdb.as_static()).await;
    let jsvalue = db
        .get_jsvalue(ObjectStores::Currency.as_static(), iso_code)
        .await;
    let (_name, rate) = from_jsvalue(jsvalue);
    // return
    rate
}
