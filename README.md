[comment]: # (lmake_md_to_doc_comments segment start A)

# indexeddb_from_rust

[comment]: # (lmake_cargo_toml_to_md start)

**experimenting with indexeddb in rust wasm PWA**  
***[repo](https://github.com/LucianoBestia/indexeddb_from_rust); version: 2021.217.1927  date: 2021-02-17 authors: Luciano Bestia***  

[comment]: # (lmake_cargo_toml_to_md end)

[comment]: # (lmake_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-688-green.svg)](https://github.com/LucianoBestia/indexeddb_from_rust/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-58-blue.svg)](https://github.com/LucianoBestia/indexeddb_from_rust/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-88-purple.svg)](https://github.com/LucianoBestia/indexeddb_from_rust/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/indexeddb_from_rust/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/indexeddb_from_rust/)

[comment]: # (lmake_lines_of_code end)

[![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/indexeddb_from_rust/blob/master/LICENSE) [![Rust](https://github.com/LucianoBestia/indexeddb_from_rust/workflows/RustAction/badge.svg)](https://github.com/LucianoBestia/indexeddb_from_rust/)

## experimenting

Indexeddb is the standard database storage inside the browser. It is not Sql. It is a document database.  
It is more or less a key-value storage, but the value can be a javascript object and that can be complex.  
The api is completely async, strange and in javascript. How to use it efficiently from rust? This is a question.  

## F12 developer tools

It is easy to see the content of indexeddb in F12. Very convenient.  

## Workaround, javascript library

If the original api is too hard, maybe I can try with [idb](https://github.com/jakearchibald/idb) javascript library that makes it easier to use.
Javascript has changed over time. Javascript is now in ES2020 edition.  
But Typescript is even better for me. I will write some typescript code and invoke it from rust.  

## Typescript adventure

The Typescript compiler must be installed with `npm` that is a part of `nodejs`. I must first install `nodejs`.  
On Debian the package `sudo apt install nodejs` is old version 10. The recommended version is 14, but it is from another package source.  
nodesource.com is providing a script to add the new package source and install `nodejs`.  
This is the commands:  

```bash
cd ~
curl -sL https://deb.nodesource.com/setup_14.x -o nodesource_setup.sh
nano nodesource_setup.sh
sudo bash nodesource_setup.sh
sudo apt install nodejs
node -v
npm -v
sudo apt install build-essential
```

Now I can install Typescript:  

```bash
npm install -g typescript
tsc --version
tsc --help
```

In the terminal I just use `tsc` to transpile my source code with settings from `tsconfig.json`.  
I added this to my `cargo make` for easy developing.
I made 2 folders `src` and `js` for typescript and javascript.  

## imports

I had major problems with `import` statements.  
I tried first with `npm install --save idb`. It saves the files in a separate `node_modules` folder. That didn't work nice with my `import` statements. I don't know why.  
At last I decided to create `idb` as a separate folder and copy the `node_modules/idb/build/esm`.  
I needed to play with `tsconfig.json` to make it work.  
I added the keys: baseUrl, rootDir, outDir, esModuleInterop and most important path.  
From one side this import paths are just like folder structure, from the other side they are like url paths.  
Confusing. But after a long experimentation I made it work. I hope I don't need ever to change this settings.  

## code flow

The browser opens index.html.  
There it runs `import init from "./pkg/indexeddb_from_rust.js";`  
and then `init("./pkg/indexeddb_from_rust_bg.wasm");`  
This is the wasm code compiled from `lib.rs`  
Rust code imports javascript module and functions with:  

```rust
#[wasm_bindgen(raw_module = "/indexeddb_from_rust/js/idb_exports.js")]
extern "C" {
    fn check_browser_capability();
    #[wasm_bindgen(catch)]
    fn init_db() -> Result<(), JsValue>;
    #[wasm_bindgen(catch)]
    fn add_key_value(store: String, key: String, value: String) -> Result<(), JsValue>;
}
```

The `idb_exports.js` is the result of typescript transpilation of `idb_exports.ts`, my main typescript module.
Inside that module I need to import the `idb` module with:  
`import * as idb from '/indexeddb_from_rust/idb/index.js';`  

## missing unsafe

When importing javascript functions with `#[wasm_bindgen]` and `extern "C"`, the rust-analyzer shows a warning about `missing unsafe`. This is not correct, the rustc compiler compile it just fine. It is because the attribute macro wasm_bindgen uses magic and makes it safe. But rust-analyzer (for now) cannot understand attribute macros.  
For those looking to disable the missing-unsafe rule until it's fixed and are using VS Code, adding the following to your settings.json and reloading your editor will suppress these errors:

```config
"rust-analyzer.diagnostics.disabled": [
    "missing-unsafe"
]
```

## extern "C" - importing javascript functions

Javascript functions are imported using the `extern "C"` block.  
For now `rustfmt` has a bug that removes the word async, because here we have javascript functions and not C functions.  
The workaround is to add `rustfmt::skip`:  

```rust
#[rustfmt::skip]
#[wasm_bindgen]
extern "C" {
    pub(crate) async fn ...
}
```

For a javascript function with no return value is simple:  
`pub(crate) fn check_browser_capability();`  
A javascript async function can return one JSValue.  
`pub(crate) async fn get_key_value(key: String, ) -> JsValue;`  
If we want to catch errors in the Promise, add attribute `wasm_bindgen(catch)`, then the functions returns `Result<JsValue, JsValue>`:  

```rust
#[wasm_bindgen(catch)]
pub(crate) async fn init_db() -> Result<JsValue, JsValue>;
```

The imported async fn needs to be await just like rust functions. The macro wasm_bindgen makes some magic to transform Promises to futures on import:  
`let currdb = open_db().await.unwrap();`  
Some of the functions are async and others are not. It can lead to strange problems if an async function is used as normal. This is a thing to be careful about.

## Currency exchange rates

I will get the daily exchange rate in json format from:  
<http://www.floatrates.com/daily/eur.json>  
and fill it into indexeddb.  

## pages

This PWA will have more pages. Pages are complete static html files inside tha pages folder. They use the same css as index.html.  
It is easy to edit and preview pages because they are complete.  
The rust code will fetch the html, extract only the body content and set_inner_html to div_for_wasm_html_injecting.  

## indexed_db key-value

The indexeddb value is a javascript object. That is really practical for javascript, but not so for rust.  
Rust structs must be serialized to json string, then javascript converts this json string into a javascript object and store it.  
I will rather store only rust/javascript strings into key-value. I choose the string data format QVS20, great for tables.  
It is very easy to parse.  


## serde-wasm-bindgen

Maybe it is better to use [serde-wasm-bindgen](https://github.com/cloudflare/serde-wasm-bindgen) to work directly with javascript values from rust, because indexeddb stores javascript objects. And the user interface needs exactly the same javascript objects. There s usually very little processing of data from the database to the user interface.  
From Rust to javascript:  
`serde_wasm_bindgen::to_value(&some_supported_rust_value)`  
From javascript to rust:  
`let value: SomeSupportedRustType = serde_wasm_bindgen::from_value(value)?;`  

## idb rust functions

### init_upgrade_db

First of all the db must be initialized and upgraded.  
`idb_mod::Database::init_upgrade_db("currdb", 2, "upgrade_currdb").await;`  
When the version is greater that the existing db version, it calls the rust function with the name in 3rd string parameter.  
This function must be exported from rust to javascript with the attribute `#[wasm_bindgen]`.  
The function accepts 4 parameters: `db: JsValue, old_version: JsValue, new_version: JsValue, transaction: JsValue,`.  
For different versions we can prepare different functions to make it more readable.  
We create a new store with: `db.create_object_store("Currency");`.  
We put data in the store with the use of Transaction mode versionchange.  
First we define the store and then put the data:  
`let cfg = tx.get_object_store("Config");`  
`cfg.put("base_currency", "EUR");`  

### modify one data with implicit transaction

```rust
let db = idb::Database::use_db("currdb").await;
db.put_key_value("store", "key", "value").await.unwrap();
```

### modify many data in one transaction

```rust
let db = idb::Database::use_db("currdb").await;
let tx = db.transaction();
let store = tx.get_object_store_readwrite("Currency");
store.put("a", "a");
store.put("b", "b");
store.put("c", "c");
tx.close();
```

