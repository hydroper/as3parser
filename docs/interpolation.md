# Interpolation

If an ActionScript source is interpolated in a place such as inside a XML file, you may want to avoid creating a new compilation unit to parse it while maintaining line numbers and columns.

It is simple to do that: just specify the `byte_range` option, passing the first and last byte offsets of an existing compilation unit:

```rust
ParserFacade(ParserOptions {
    byte_range: Some((i, j)),
    ..default()
})
```

You may then invoke parser methods within that `ParserFacade` object.

<!--

If an ActionScript source is interpolated in a place such as inside a XML file and it is desired to maintain line numbers and columns respective to that file, a technique for handling such use case is to build a prefix of line breaks and initial whitespace to the source text before constructing the `CompilationUnit` object.

The following XML is an example of interpolated ActionScript source:

```xml
<?xml version="1.0"?>
<document>
    <![CDATA[
        override protected function run(): void {}
    ]]>
</document>
```

To determine the prefix for the `CompilationUnit`'s source text, it is necessary to have the initial line number and column beforehand (in this case, the one right after the `<![CDATA[` sequence) and the count of characters at that line until the initial column:

```rust
// `first_line_number` starts at one (1)
let prefix
    = "\n".repeat(first_line_number - 1)
    + &" ".repeat(character_count_at_first_line_until_first_column);
```

The `CompilationUnit` can then be created as follows:

```rust
let compilation_unit = CompilationUnit::new(Some(path), prefix + &source_text, &compiler_options);
```

-->