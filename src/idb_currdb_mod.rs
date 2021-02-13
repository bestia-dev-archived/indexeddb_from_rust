// idb_currdb_mod

// Specific code for work with the `currdb` database

use crate::idb_mod as idb;
use crate::web_sys_mod as w;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;

/// init_upgrade_currdb will create the database and call upgrade_currdb()
pub async fn init_upgrade_currdb() {
    idb::Database::init_upgrade_db("currdb", 2, "upgrade_currdb").await;
}

/// upgrade_currdb is called from javascript function init_upgrade_db()
#[wasm_bindgen]
#[allow(dead_code)]
pub fn upgrade_currdb(
    db: JsValue,
    old_version: JsValue,
    new_version: JsValue,
    transaction: JsValue,
) {
    let db = idb::Database::from(db);
    let tx = idb::Transaction::from(transaction);
    let old_version = unwrap!(old_version.as_f64()) as i32;
    let new_version = unwrap!(new_version.as_f64()) as i32;
    w::debug_write(&format!(
        "upgrade_currdb from v{} to v{}",
        old_version, new_version
    ));

    if old_version <= 0 {
        upgrade_from_v00_to_v01(&db);
    }
    if old_version <= 1 {
        upgrade_from_v01_to_v02(&db, &tx);
    }
}

fn upgrade_from_v00_to_v01(db: &idb::Database) {
    w::debug_write("upgrade_from_v00_to_v01");
    db.create_object_store("currency");
}

fn upgrade_from_v01_to_v02(db: &idb::Database, tx: &idb::Transaction) {
    w::debug_write("upgrade_from_v01_to_v02");
    db.create_object_store("config");
    let cfg = tx.get_object_store_versionchange("config");
    // this is a special put inside a transaction, that is inside version upgrade
    cfg.put("base_currency", "EUR");
    cfg.put("quote_currency", "USD");
    cfg.put("rate", "1.21");
    cfg.put("date_fetch", "none");
}
