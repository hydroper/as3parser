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

## Documentation

* [Getting started](docs/getting-started.md)
* [Working with diagnostics](docs/diagnostics.md)
* [Working with embedded ActionScript](docs/embedded-actionscript.md)
* [Standards](docs/standards.md)
* [Features](docs/features.md)
* [Feature syntax summary](docs/feature-syntax-summary.md)
* [Tests](docs/tests.md)

## Future improvements

* [ ] ASDoc: attach first line and column to the main body and each tag.

## Verifier

ActionScript execution consists of parsing, verification and evaluation. Verification can be performed ahead of time, as is already done by the existing initial compilers of the language (ASC 2 and Apache Flex's MXML compiler), deriving control flow graph and a static type host. This project might be integrated with a verifier in the future.

Details: [Documentation progress list](crates/as3_verifier/doc-progress.md), [misc.](crates/as3_verifier/progress-list.md)

Documentation:

* [Type](docs/verifier/Type.md)
  * Kinds
    * [Name](docs/verifier/Type/kinds/name.md)
    * [Package](docs/verifier/Type/kinds/package.md)
    * [Namespace](docs/verifier/Type/kinds/namespace.md)
    * [Namespace set](docs/verifier/Type/kinds/namespace-set.md)
    * [Any type](docs/verifier/Type/kinds/any-type.md)
    * [Undefined type](docs/verifier/Type/kinds/undefined-type.md)
    * [Void type](docs/verifier/Type/kinds/void-type.md)
    * [Never type](docs/verifier/Type/kinds/never-type.md)
    * [Class type](docs/verifier/Type/kinds/class-type.md)
    * [Interface type](docs/verifier/Type/kinds/interface-type.md)
    * [Enum type](docs/verifier/Type/kinds/enum-type.md)
    * [Function type](docs/verifier/Type/kinds/function-type.md)
* [TypeHost](docs/verifier/TypeHost.md)
* [Vector type](docs/verifier/vector.md)

## License

Mozilla Public License: https://www.mozilla.org/en-US/MPL/2.0/