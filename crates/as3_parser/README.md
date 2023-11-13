# ActionScript 3 parser

> **NOTICE:** The parser is almost finishing (excluding tests). A verifier will be futurely implemented.

An ActionScript 3 parser written in the Rust language.

## Requirements

* Nightly Rust

## Usage

*Tokenizing*:

```rust
use as3_parser::*;

// Tokenize `n * n`
let _n = "n".to_owned();
let source = Source::new(None, "n * n".into(), &CompilerOptions::new());
let mut tokenizer = Tokenizer::new(&source);
assert!(matches!(tokenizer.scan_ie_div(), Ok((Token::Identifier(_n), _))));
assert!(matches!(tokenizer.scan_ie_div(), Ok((Token::Times, _))));
assert!(matches!(tokenizer.scan_ie_div(), Ok((Token::Identifier(_n), _))));
```

*Parsing*: not available yet.

*Verification*: not available yet.