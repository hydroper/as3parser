# Working with diagnostics

The parser typically takes a `CompilationUnit` and produces a program structure. Diagnostics are emitted to that `CompilationUnit` rather than being returned by the parser. You can obtain them as follows:

```rust
// Sorts the diagnostics, including these from files resolved from
// `IncludeDirective`.
compilation_unit.sort_diagnostics();

// diagnostics: Vec<Diagnostic>
// This includes diagnostics from files resolved from
// `IncludeDirective`.
let diagnostics = compilation_unit.nested_diagnostics();

// Formats the diagnostic with default English
for diagnostic in diagnostics {
    println!("{}", diagnostic.format_english());
}
```

## Invalidation

A compilation unit is invalidated when it or any of its `IncludeDirective` files contain any errors:

```rust
// invalidated: bool
let invalidated = compilation_unit.invalidated();
```

## Adding diagnostics

Adding a diagnostic to a `CompilationUnit` is a simple call, where `K` is the diagnostic kind:

```rust
// Syntax error
compilation_unit.add_diagnostic(Diagnostic::new_syntax_error(&location, DiagnosticKind::K, diagnostic_arguments![]));

// Verify error
compilation_unit.add_diagnostic(Diagnostic::new_verify_error(&location, DiagnosticKind::K, diagnostic_arguments![]));

// Warning
compilation_unit.add_diagnostic(Diagnostic::new_warning(&location, DiagnosticKind::K, diagnostic_arguments![]));
```

The `diagnostic_arguments![]` literal takes elements in one of the forms:

* `Token(token)`
* `String(string)`

## Custom kinds

If you need to support your own messages, set the `custom_kind` field:

```rust
diagnostic.set_custom_kind(Some(Rc::new(kind)));
```

Where `kind` is an enumeration's variant. You may then later cast the custom kind from `Rc<dyn Any>` to your enumeration through `.downcast::<MyEnum>()`.

If you want, for simple debugging purposes, you may finish formatting your own message with location information by using:

```rust
diagnostic.format_with_message("My message", Some(custom_id_number))
```

For real use cases, calling `.format_with_message()` is not preferred if your application is not solely in English, since it will add categories such as `Warning`.

Moreover, argument variants in `as3_parser::ns::Diagnostic` may not be enough. For example, a compiler may want to store a symbol argument; in this case, an extra layer over `as3_parser::ns::Diagnostic` must be provided, supporting more variants.