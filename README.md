[comment]: # (lmake_md_to_doc_comments segment start A)

# indexeddb_from_rust

[comment]: # (lmake_cargo_toml_to_md start)

**experimenting with indexeddb in rust wasm PWA**  
***[repo](https://github.com/LucianoBestia/indexeddb_from_rust); version: 2021.209.1405  date: 2021-02-09 authors: Luciano Bestia***  

[comment]: # (lmake_cargo_toml_to_md end)

[comment]: # (lmake_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-40-green.svg)]()
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-4-blue.svg)]()
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-12-purple.svg)]()
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)]()
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)]()

[comment]: # (lmake_lines_of_code end)

[![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/indexeddb_from_rust/blob/master/LICENSE) [![Rust](https://github.com/LucianoBestia/indexeddb_from_rust/workflows/RustAction/badge.svg)](https://github.com/LucianoBestia/indexeddb_from_rust/)

## Try it

<https://bestia.dev/indexeddb_from_rust/>  
![screenshot](https://github.com/LucianoBestia/indexeddb_from_rust/raw/main/images/minimal_clock.jpg)  

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

In the terminal I just use `tsc` to transpile my source code.  I added this to `cargo make` for easy developing.
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
#[wasm_bindgen(raw_module = "/indexeddb_from_rust/js/indexeddb_lib.js")]
extern "C" {
    fn check_browser_capability();
    #[wasm_bindgen(catch)]
    fn open_db() -> Result<(), JsValue>;
    #[wasm_bindgen(catch)]
    fn add_key_value(store: String, key: String, value: String) -> Result<(), JsValue>;
}
```

The `indexeddb_lib.js` is the result of typescript transpilation of `indexeddb_lib.ts`, my main typescript module.
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
