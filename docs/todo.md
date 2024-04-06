To do:

(Migration from the Jet parser to an ActionScript 3 parser.)

- [x] Jet => ActionScript 3
- [x] jet => as3
- [x] Add nested compilation units to CompilationUnit and useful methods such as sorting and retrieving nested diagnostics
- [x] Add a "custom_id" field to Diagnostic holding an `Option<String>`
  - [x] `custom_id()`
  - [x] `set_custom_id(id: Option<&str>)`
- [x] Add a "format_with_message(message: &str)" method to Diagnostic
- [x] Support ASDoc `@inheritDoc` tag
- [x] Support ASDoc `@copy reference` tag
- [x] Remove the semantics
- [x] Remove the verifier
- [x] Remove proxies
- [x] Remove nested multi-line comments
- [x] Meta-data consists of simple string key-values
- [x] Meta-data ASDoc
- [x] Attribute combination allows for "id", "q.x", and "q[k]" expression access modifiers
- [x] Object initializer allows for non-attribute qualified identifier keys
- [x] Namespace definition
  - [x] Detect "namespace" keyword everywhere where "type" is detected
- [x] Use namespace definition
- [x] `new <T> []` expression
- [ ] Include directive
  - [ ] Prevent include cycle
  - [ ] Contribute nested compilation unit to surrounding compilation unit
- [ ] Update "New Syntax" documentation
  - [ ] Destructuring, optional chaining, raw string literals, nullish coalescing, triple string literals, keywords as identifiers in certain contexts, enumerations, and a lot more.