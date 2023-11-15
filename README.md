# ActionScript 3 Parser

<p align="center">
  <a href="https://lib.rs/crates/as3_parser">
    <img src="https://img.shields.io/badge/lib.rs-green">
  </a>
  <a href="https://docs.rs/as3_parser/latest/as3_parser">
    <img src="https://img.shields.io/badge/Rust%20API%20Documentation-gray">
  </a>
</p>

Handwritten ActionScript 3 parser in the Rust language.

## Status

The parser is complete. I am adding documentation and more tests to it.

## Getting started

Install it in your Cargo project with `cargo add as3_parser`.

```rust
use as3_parser::*;

let source = Source::new(None, "x ** y".into(), &CompilerOptions::new());
if let Some(program) = parser_facade::parse_program(&source) {
    // program: Rc<ast::Program>
}
```

## Features

This ActionScript 3 parser adds several syntax constructs from TypeScript, ECMAScript 4 and Apache Royale Compiler. See [Features](docs/features.md) for full details.

## Standards

A copy of the ECMA and Adobe standards is available at the [docs/standards](docs/standards) directory of this repository, including:

* Language Specifications
* AVM2 Overview
* SWF 19 Specification

## License

Mozilla Public License: https://www.mozilla.org/en-US/MPL/2.0/