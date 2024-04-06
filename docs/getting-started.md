# Getting started

Install `as3_parser` in your Cargo project with `cargo add as3_parser`, or add this to your `Cargo.toml` file:

```toml
[dependencies]
as3_parser = "0.4"
```

Parse programs or expressions through the `ParserFacade` structure of the `as3_parser` crate:

```rust
use as3_parser::ns::*;

let source = CompilationUnit::new(None, "x ** y".into(), &CompilerOptions::new());
if let Some(program) = ParserFacade::parse_program(&source) {
    // program: Rc<Program>
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

* Meta data can contain an ASDoc comment
* Annotatable directives can contain an ASDoc comment
* Package definitions can contain an ASDoc comment

## Building your own compiler

* **Verifier**: the `as3_parser` project is currently not integrated with a verifier, so you would need to build one yourself. A verifier yields verify errors, control flow graphs, and symbols.
* **Reading and writing SWFs**: the Ruffle player contains a [`swf` package](https://github.com/ruffle-rs/ruffle/tree/master/swf) that reads and writes SWF from/into structures.
* Detect [domain memory operations](https://obtw.wordpress.com/2013/04/03/making-bytearray-faster) to generate optimized AVM2 instructions.
