# Getting started

Switch to nightly Rust:

```sh
rustup default nightly
```

Install `as3_parser` in your Cargo project with `cargo add as3_parser`, or add this to your `Cargo.toml` file:

```toml
[dependencies]
as3_parser = "1.0"
```

Parse programs or expressions through the `ParserFacade` structure of the `as3_parser` crate:

```rust
use as3_parser::ns::*;

// Create compilation unit
let compilation_unit = CompilationUnit::new(None, "x ** y".into());

// Parser options
let parser_options = ParserOptions::default();

// Parse through ParserFacade
let program = ParserFacade(&compilation_unit, parser_options).parse_program();
```

## Serializing nodes

In Rust, to serialize data you generally use the `serde` package. Add it to your project's `Cargo.toml` manifest with:

```toml
[dependencies]
serde = { version = "1.0.192", features = ["rc", "derive"] }
serde_json = "1.0.108"
```

With this, you can serialize a node to a JSON with:

```rust
let json = serde_json::to_string_pretty(&node).unwrap();
```

## ASDoc

The parser attaches ASDoc comments to supported elements and recognizes tags beforehand, maintaining indentation of code blocks.

* Meta data can contain an ASDoc comment
* Annotatable directives can contain an ASDoc comment
* Package definitions can contain an ASDoc comment

Source location is fully available within ASDoc comments. For display texts, the location always ranges from the tag at-character to the last non whitespace character.