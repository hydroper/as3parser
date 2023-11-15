# Getting started

Install `as3_parser` in your Cargo project with `cargo add as3_parser`, or add this to your `Cargo.toml` file:

```toml
[dependencies]
as3_parser = "0.1"
```

Parse programs or expressions through the `parser_facade` submodule of the `as3_parser` crate:

```rust
use as3_parser::*;

let source = Source::new(None, "x ** y".into(), &CompilerOptions::new());
if let Some(program) = parser_facade::parse_program(&source) {
    // program: Rc<ast::Program>
}
```