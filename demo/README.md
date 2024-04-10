# AS3Parser Demo

[Online demo](https://hydroper.github.io/as3parser/demo)

## Building

Requirements:

* Rust
* Run `cargo install -f wasm-bindgen-cli`

Run:

```sh
cargo build -p as3_parser_demo --release --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir demo/dist target/wasm32-unknown-unknown/release/as3_parser_demo.wasm
```

## Serving

## Using Live Server in Visual Studio Code

Install the Live Server extension in Visual Studio Code and serve the index.html file using that extension.

## Using Miniserve

Requirements:

```plain
cargo install miniserve
```

Serve it locally by running:

```sh
miniserve . --index "index.html" -p 8080
```