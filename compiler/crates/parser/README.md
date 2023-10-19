# ActionScript 3 parser

> **NOTICE:** Only the tokenizer of this crate is done. The rest is a work-in-progress.

An ActionScript 3 parser and verifier written in the Rust language.

## Usage

*Tokenizing*:

```rust
use as3_parser::*;

// Tokenize `n * n`
let _n = "n".to_owned();
let source = Source::new(None, "n * n".into(), &CompilerOptions::new());
let mut tokenizer = Tokenizer::new(&source, &source.text());
assert!(matches!(tokenizer.scan_ie_div(true), Ok((Token::Identifier(_n), _))));
assert!(matches!(tokenizer.scan_ie_div(true), Ok((Token::Times, _))));
assert!(matches!(tokenizer.scan_ie_div(true), Ok((Token::Identifier(_n), _))));
```

*Parsing*: not available yet.

*Verification*: not available yet.