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

## Invalidation

A source is invalidated when it or any of its `IncludeDirective` files contain any errors:

```rust
// invalidated: bool
let invalidated = source.invalidated();
```

## Adding diagnostics

Adding a diagnostic to a `Source` object is a simple call, where `K` is the diagnostic kind:

```as3
// Syntax error
source.add_diagnostic(Diagnostic::new_syntax_error(&location, DiagnosticKind::K, diagnostic_arguments![]));

// Verify error
source.add_diagnostic(Diagnostic::new_verify_error(&location, DiagnosticKind::K, diagnostic_arguments![]));

// Warning
source.add_diagnostic(Diagnostic::new_warning(&location, DiagnosticKind::K, diagnostic_arguments![]));
```

The `diagnostic_arguments![]` literal takes elements in one of the forms:

* `Token(token)`
* `String(string)`