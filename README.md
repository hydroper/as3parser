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

It is almost finished; just missing directives ([progress tracker](crates/as3_parser/progress.md)).

## Getting started

Install it in your Cargo project with `cargo add as3_parser`.

```rust
use as3_parser::ast;
// `Parser` will be exposed once the parser is finished.
```

What the parser currently looks like:

```rust
use as3_parser::*;
let source = Source::new(None, "x ** y".into(), &CompilerOptions::new());
let mut parser = Parser::new(&source);
let exp = parser.parse_expression(ExpressionContext {
    ..default()
}).ok();
if exp.is_some() {
    parser.expect_eof();
}
if !source.invalidated() {
    let exp = exp.unwrap();
    // exp: Rc<ast::Expression>
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