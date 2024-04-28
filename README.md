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

## Compiler

An ActionScript compiler does more than just parsing, including verification, and SWF and SWC processing.

An ActionScript compiler handles three source file formats: ActionScript 3, MXML, and CSS.

An ActionScript compiler outputs several problems, constructs a flow graph for every activation, and attaches meaning to tree nodes.

This project itself is not a compiler, but it's designated to facilitate writing one, parsing the three file formats mentioned.

## License

Apache License 2.0, copyright 2024 Hydroper
