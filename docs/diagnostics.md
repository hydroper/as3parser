# Diagnostics

The parser typically takes a `Source` object and produces a program structure. Diagnostics are emitted to that `Source` rather than being returned by the parser. You can obtain them as follows:

```rust
// Sorts the diagnostics, including these from files resolved from
// `IncludeDirective`.
source.sort_diagnostics();

// diagnostics: Vec<Diagnostic>
// This includes diagnostics from files resolved from
// `IncludeDirective`.
let diagnostics = source.recursive_diagnostics();

// Formats the diagnostic with default English
for diagnostic in diagnostics {
    println!("{}", diagnostic.format_default());
}
```