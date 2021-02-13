// config_mod

// config is in store "config" in database currdb in indexeddb

use crate::idb_mod as idb;

pub async fn currdb() -> idb::Database {
    idb::Database::use_db("currdb").await
}

pub async fn set_base_currency(iso_code: &str) {
    currdb()
        .await
        .put_key_value("config", "base_currency", iso_code)
        .await
        .unwrap();
}

pub async fn get_base_currency() -> String {
    let value = idb::get_key_value("currdb", "config", "base_currency").await;
    // return
    value
}

pub async fn set_quote_currency(iso_code: &str) {
    idb::put_key_value("currdb", "config", "quote_currency", iso_code)
        .await
        .unwrap();
}

pub async fn get_quote_currency() -> String {
    let value = idb::get_key_value("currdb", "config", "quote_currency").await;
    // return
    value
}

pub async fn set_rate(rate: &str) {
    idb::put_key_value("currdb", "config", "rate", rate)
        .await
        .unwrap();
}

pub async fn get_rate() -> String {
    let value = idb::get_key_value("currdb", "config", "rate").await;
    // return
    value
}

pub async fn set_date_fetch(date_fetch: &str) {
    idb::put_key_value("currdb", "config", "date_fetch", date_fetch)
        .await
        .unwrap();
}

pub async fn get_date_fetch() -> String {
    let value = idb::get_key_value("currdb", "config", "date_fetch").await;
    // return
    value
}
