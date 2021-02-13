// currency_mod

// the store currency in indexeddb database currdb

use serde_json::{Map, Value};
use unwrap::unwrap;

use crate::idb_mod as idb;

pub async fn fill_currency_store(json_map_string_value: &Map<String, Value>) {
    let db = idb::Database::use_db("currdb").await;
    let tx = db.transaction_open();
    let store = tx.get_object_store_readwrite("currency");
    for string_value in json_map_string_value {
        // the value will have 2 columns: name(string) and rate(f64)
        // indexed_db would serialize rust object to json and then in javascript json to javascript object and then store
        // I will use only strings. The conversion to/from string will be in QVS20 format for compact, ubiquitous and fast conversion
        let name = unwrap!(string_value.1["name"].as_str());
        let rate = unwrap!(string_value.1["rate"].as_f64());
        let qvs20_value = serialize_qvs20_single_row(name, rate);
        store.put(&string_value.0.to_uppercase(), &qvs20_value);
    }
    tx.close();
}

// The indexeddb store `currency` has a key: currency iso_code
// and a string value. The Value string is in simple qvs20 format to accommodate more data fields.
// <https://github.com/LucianoBestia/QVS20>
// It is so simple, that the serialization can be done "manually".
// I know 100% for sure there cannot be 4 special characters that must be escaped: [, ], LF, \
// Value has 2 fields: name:string and rate:decimal
// example: [U.S. Dollar][1.2114283313591]\n
// qvs20 schema:
// [S][currency][]
// [String][Decimal]
// [][]
// [][]
// [name][rate]\n";

// serialize single row
pub fn serialize_qvs20_single_row(name: &str, rate: f64) -> String {
    let mut text = String::with_capacity(40);
    text.push('[');
    text.push_str(name);
    text.push(']');

    text.push('[');
    text.push_str(&rate.to_string());
    text.push(']');

    text.push('\n');
    // return
    text
}

// deserialize single row
pub fn deserialize_qvs20_single_row(qvs20_string: &str) -> (String, f64) {
    let vec_of_string: Vec<&str> = qvs20_string
        .trim_start_matches("[")
        .trim_end_matches("]\n")
        .split_terminator("][")
        .collect();
    let name = vec_of_string[0].to_string();
    let rate = vec_of_string[1].parse::<f64>().unwrap();
    // return
    (name, rate)
}

#[cfg(test)]
mod test {
    use super::*;
    // use unwrap::unwrap;

    #[test]
    pub fn t01() {
        let x = serialize_qvs20_single_row("U.S. Dollar", 1.2114283313591);
        assert_eq!(x, "[U.S. Dollar][1.2114283313591]\n");
    }
    #[test]
    pub fn t02() {
        let (name, rate) = deserialize_qvs20_single_row("[U.S. Dollar][1.2114283313591]\n");
        assert_eq!(name, "U.S. Dollar");
        assert_eq!(rate, 1.2114283313591);
    }
}
