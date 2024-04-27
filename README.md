# ActionScript 3 Parser

<p align="center">
  <a href="https://lib.rs/crates/as3_parser">
    <img src="https://img.shields.io/badge/lib.rs-green">
  </a>
  <a href="https://docs.rs/as3_parser">
    <img src="https://img.shields.io/badge/Rust%20API%20Documentation-gray">
  </a>
</p>

ActionScript 3 parser in the Rust language.

[Online demo](https://hydroper.github.io/as3parser/demo)

## Documentation

[Getting started](docs/getting-started.md)

[Working with diagnostics](docs/diagnostics.md)

[Working with locations](docs/locations.md)

[Working with MXML](docs/working-with-mxml.md)

[Working with CSS](docs/working-with-css.md)

[Attaching Meaning](docs/attaching-meaning.md)

[Reference Documents](docs/references.md)

[New Syntax](docs/new-syntax.md)

[Processing Deviations](docs/processing-deviations.md)

## Wiki

The [wiki](https://github.com/hydroper/as3parser/wiki) of this repository contains other introductory articles.

## Verifier

ActionScript execution consists of parsing, verification, and evaluation. Verification can be performed ahead of time, as is already done by the existing initial compilers of the language (ASC 2 and Apache Flex's MXML compiler), reporting errors and warnings, deriving a control flow graph for every activation, and attaching symbols to syntactic nodes. This project might be integrated with a verifier in the future.

## License

Apache License 2.0, copyright 2024 Hydroper
