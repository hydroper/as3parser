# Getting started

Install `as3_parser` in your Cargo project with `cargo add as3_parser`, or add this to your `Cargo.toml` file:

```toml
[dependencies]
as3_parser = "0.2"
```

Parse programs or expressions through the `parser_facade` submodule of the `as3_parser` crate:

```rust
use as3_parser::*;

let source = Source::new(None, "x ** y".into(), &CompilerOptions::new());
if let Some(program) = parser_facade::parse_program(&source) {
    // program: Rc<ast::Program>
}
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

* Fields from record type expressions can contain an ASDoc comment
* Meta data can contain an ASDoc comment
* Annotatable directives can contain an ASDoc comment
* Package definitions can contain an ASDoc comment

## Building your own verifier

> **NOTICE**: A verifier might be futurely developed as part of this repository.

* A verifier derives semantic symbols and produces multiple errors from the verification phase. Not all meta data are trivial to implement:
  * `[Embed]`
* In Rust you use the `std::rc::Rc` type to use reference counting, which is similiar to C++'s `std::shared_t<T>`. Most AST structures are behind a `Rc`, so that you can attach a symbol to it; for this, use the `ByAddress` type exported from the crates.io `by_address` crate (for example, `ByAddress<Rc<ast::Expression>>`) as a key in a [`HashMap`](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html).
* A verifier implements bidirectional type checking.
* A verifier implements control flow checking.

## Building your own compiler

* Reading and writing SWFs: the Ruffle player contains a [`swf` package](https://github.com/ruffle-rs/ruffle/tree/master/swf) that reads and writes SWF from/into structures.