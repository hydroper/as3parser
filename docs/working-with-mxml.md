# Working with MXML

Average parsing of XML documents such as MXML is provided within this project, deviating slightly from the XML specification, which is a bit large to read for a proper implementation.

The parser supports parsing only the UTF-8 encoding and XML version 1.0.

Here is an example:

```rust
use as3_parser::ns::*;

let text = r#"
<?xml version="1.0" encoding="UTF-8"?>
<s:Application
    xmlns:fx="http://ns.adobe.com/flex"
    xmlns:s="library://ns.adobe.com/flex/spark"
    creationComplete="initialize()">
</s:Application>
"#;
let compilation_unit = CompilationUnit::new(None, text.into());

let parser_options = ParserOptions {
    // Ignore whitespace chunks in a node list when at least one
    // element appears.
    ignore_xml_whitespace: true,
    ..default()
};

let document: Rc<Mxml> = ParserFacade(&compilation_unit, parser_options).parse_mxml();
```

The nodes used for ECMAScript for XML (E4X) and MXML are distinct. For example, MXML uses `MxmlElement` instead of `XmlElement`, and `MxmlContent` instead of `XmlContent`.

## Qualified names

Every element stores a reference to a semantic namespace set (`Rc<MxmlNamespace>`) consisting of mappings from prefixes to URIs.

Resolve a plain `XmlName` node to a (*uri*, *name*) string group by invoking `name.resolve_name(&namespace)`, where `namespace` is usually `&element.namespace`.

There are additional useful methods available within `XmlName` other than `resolve_name()`.

## Default namespace

The prefix for the default namespace is the empty string, but referred to by the `MxmlNamespace::DEFAULT_NAMESPACE` constant.
