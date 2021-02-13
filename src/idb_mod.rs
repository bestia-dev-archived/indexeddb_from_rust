// idb_mod.rs

// Objects and method to work with indexeddb.
// Fully rust code and types. All the JsValue are wrapped.
// It uses the idb javascript library, idb_exports.ts and idb_imports_mod.rs

use crate::idb_imports_mod as idbjs;
use crate::web_sys_mod as w;
use unwrap::unwrap;
use wasm_bindgen::JsValue;

pub fn check_browser_capability() {
    idbjs::check_browser_capability();
}

pub async fn put_key_value(
    db_name: &str,
    store: &str,
    key: &str,
    value: &str,
) -> Result<(), JsValue> {
    // return
    idbjs::put_key_value(db_name, store, key, value).await
}

pub async fn get_key_value(db_name: &str, store: &str, key: &str) -> String {
    // return
    unwrap!(idbjs::get_key_value(db_name, store, key).await.as_string())
}

// region: Database
pub struct Database {
    db: JsValue,
}
impl Database {
    /// init and upgrade
    pub async fn init_upgrade_db(db_name: &str, version: u32, upgrade_callback_fn_name: &str) {
        idbjs::init_upgrade_db(db_name, version, upgrade_callback_fn_name)
            .await
            .unwrap();
    }
    pub async fn use_db(db_name: &str) -> Self {
        let db = idbjs::use_db(db_name).await;
        // return
        Database { db }
    }
    pub fn create_object_store(&self, store_name: &str) {
        idbjs::create_object_store(self.db.clone(), store_name);
    }
    pub fn transaction_open(&self) -> Transaction {
        let tx = idbjs::transaction_open(&self.db);
        Transaction::from(tx)
    }
    pub async fn put_key_value(&self, store: &str, key: &str, value: &str) -> Result<(), JsValue> {
        idbjs::db_put_key_value(&self.db, store, key, value).await
    }
}
/// Database from JsValue
impl From<JsValue> for Database {
    /// Database from JsValue
    fn from(db: JsValue) -> Self {
        // return
        Database { db }
    }
}
// endregion: Database

// region: Transaction
pub struct Transaction {
    tx: JsValue,
}
impl Transaction {
    pub fn get_object_store_versionchange(&self, store_name: &str) -> ObjectStoreInsideTransaction {
        let tx_ob_st = idbjs::get_object_store_from_transaction_versionchange(&self.tx, store_name);
        let tx_ob_st = ObjectStoreInsideTransaction::from(tx_ob_st);
        // return
        tx_ob_st
    }
    pub fn get_object_store_readwrite(&self, store_name: &str) -> ObjectStoreInsideTransaction {
        let tx_ob_st = idbjs::get_object_store_from_transaction_readwrite(&self.tx, store_name);
        let tx_ob_st = ObjectStoreInsideTransaction::from(tx_ob_st);
        // return
        tx_ob_st
    }
    pub fn close(&self) {
        idbjs::transaction_close(&self.tx);
    }
}
/// Transaction from JsValue
impl From<JsValue> for Transaction {
    /// Transaction from JsValue
    fn from(tx: JsValue) -> Self {
        // return
        Transaction { tx }
    }
}
/// Transaction into JsValue
impl From<Transaction> for JsValue {
    /// Transaction into JsValue
    fn from(tx: Transaction) -> JsValue {
        tx.tx
    }
}
// endregion: Transaction

// region: ObjectStoreInsideTransaction
pub struct ObjectStoreInsideTransaction {
    tx_ob_st: JsValue,
}
impl ObjectStoreInsideTransaction {
    pub fn put(&self, key: &str, value: &str) {
        idbjs::transaction_object_store_put(self.tx_ob_st.clone(), key, value);
    }
}
/// ObjectStoreInsideTransaction from JsValue
impl From<JsValue> for ObjectStoreInsideTransaction {
    /// ObjectStoreInsideTransaction from JsValue
    fn from(tx_ob_st: JsValue) -> Self {
        // return
        ObjectStoreInsideTransaction { tx_ob_st }
    }
}
/// ObjectStoreInsideTransaction into JsValue
impl From<ObjectStoreInsideTransaction> for JsValue {
    /// ObjectStoreInsideTransaction into JsValue
    fn from(tx: ObjectStoreInsideTransaction) -> JsValue {
        tx.tx_ob_st.clone()
    }
}
// endregion: ObjectStoreInsideTransaction
