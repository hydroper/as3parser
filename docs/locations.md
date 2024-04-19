# Working with locations

The `Location` structure includes full first-last locations:

* The compilation unit
* The first and last lines
* The first and last columns
* The first and last offsets

Logically, the `Location` structure stores solely the compilation unit reference and the first and last offsets. Then, it computes lines and columns through that compilation unit and these two offsets.

## Offset

An offset is a zero-based UTF-8 byte position in a source text, as the `str` type in the Rust langauge is UTF-8 encoded.

## Byte ranges

If a source text is situated in a place such as inside a MXML file, you may want to avoid creating a new compilation unit to parse that source text while maintaining line numbers and columns.

It is simple to do that: just specify the `byte_range` option, passing the first and last byte offsets of an existing compilation unit:

```rust
ParserFacade(ParserOptions {
    byte_range: Some((first_offset, last_offset)),
    ..default()
})
```

You may then invoke parser methods within that `ParserFacade` object.

### Determining offsets

The parser attaches full source location information, including within MXML nodes, crucial for the `byte_range` option.

For example:

* MXML attribute values contain `Location` data (`attribute.value.1`) including the quotes.
* MXML CDATA contain `Location` data (`MxmlContent::CData((_, location))`) including the `<![CDATA[` and `]]>` delimiters.