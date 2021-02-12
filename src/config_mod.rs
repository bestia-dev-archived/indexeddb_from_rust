// config_mod

// config is in store "config" in database currency_rates in indexeddb

use crate::idb_mod as idb;

pub async fn set_base_currency(iso_code: &str) {
    idb::put_key_value("currency_rates", "config", "base_currency", iso_code)
        .await
        .unwrap();
}

pub async fn get_base_currency() -> String {
    let value = idb::get_key_value("currency_rates", "config", "base_currency").await;
    // return
    value
}

pub async fn set_quote_currency(iso_code: &str) {
    idb::put_key_value("currency_rates", "config", "quote_currency", iso_code)
        .await
        .unwrap();
}

pub async fn get_quote_currency() -> String {
    let value = idb::get_key_value("currency_rates", "config", "quote_currency").await;
    // return
    value
}

pub async fn set_rate(rate: &str) {
    idb::put_key_value("currency_rates", "config", "rate", rate)
        .await
        .unwrap();
}

pub async fn get_rate() -> String {
    let value = idb::get_key_value("currency_rates", "config", "rate").await;
    // return
    value
}

pub async fn set_date_fetch(date_fetch: &str) {
    idb::put_key_value("currency_rates", "config", "date_fetch", date_fetch)
        .await
        .unwrap();
}

pub async fn get_date_fetch() -> String {
    let value = idb::get_key_value("currency_rates", "config", "date_fetch").await;
    // return
    value
}
