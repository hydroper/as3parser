# Working with embedded ActionScript

If an ActionScript source is embedded in a place such as inside a MXML file and you wish to maintain line numbers and columns respective to that file, you may simply add a prefix of line breaks and initial whitespace to the source text before constructing the `Source` object.

Here is an example of embedded ActionScript:

```xml
<document>
  <![CDATA[
    protected function f(): void {}
  ]]>
</document>
```

To determine the prefix for the `Source` text, you need to have the initial line number and column beforehand (in this case, the one right after the `<![CDATA[` sequence) and the count of characters at that line until the initial column:

```rust
// `first_line_number` starts at one (1)
let prefix
    = "\n".repeat(first_line_number - 1)
    + &" ".repeat(character_count_at_first_line_until_first_column);
```

The `Source` object can then be created as:

```rust
let source = Source::new(Some(path), prefix + &source_text, &compiler_options);
```