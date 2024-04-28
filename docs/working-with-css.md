# Working with CSS

You can use the `CssParserFacade` to parse CSS style sheets from Flex components.

*Note: if anything is missing from Apache Royale's latest CSS, you may create an issue or pull request about it.*

Here is an example:

```rust
use as3_parser::ns::*;

let text = r#"
.style1 {
    backgroundImage: Embed("../assets/flower.gif");
    backgroundAlpha: .2
}
"#;

let compilation_unit = CompilationUnit::new(None, text.into());

let document: Rc<CssDocument> = CssParserFacade(&compilation_unit, ParserOptions::default()).parse_document();
```