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

[Working with interpolated ActionScript](docs/interpolation.md)

[Standards](docs/standards.md)

[New Syntax](docs/new-syntax.md)

[Processing Deviations](docs/processing-deviations.md)

## Verifier

ActionScript execution consists of parsing, verification, and evaluation. Verification can be performed ahead of time, as is already done by the existing initial compilers of the language (ASC 2 and Apache Flex's MXML compiler), reporting errors and warnings, deriving a control flow graph for every activation, and attaching symbols to syntactic nodes. This project might be integrated with a verifier in the future.

## MXML parsing

Average parsing of XML documents such as MXML may be provided anytime in this project, and it should not be difficult at all as long as it deviates slightly from the XML specification, which is a bit large to read for a proper implementation.

- [x] Define tree structures
- [ ] Parse a XML document (UTF-8 support only).
  - [ ] Filter whitespace nodes out of an element when it includes at least one child element.
  - [ ] Create whitespace nodes solely for whitespace chunks beginning in a line and ending in the same or another line, such that whitespace intermixed with characters belong to them.
- [ ] Unescape entities through the `htmlentity` crate.

## CSS parsing

MXML supports a subset of CSS.

- [ ] Define tree structures
- [ ] Parse a style sheet

## License

Apache License 2.0, copyright 2024 Hydroper
