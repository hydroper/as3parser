# Attaching Meaning

ActionScript 3 compilers may need to attach meaning to nodes. They can do so by using the `TreeSemantics` mapping, which works just as a hash map, but using an efficient implementation for the different node types and splitting of dictionaries for large compilation units.

```rust
use as3_parser::ns::*;

// Construct a TreeSemantics structure, where ExampleMeaning
// is any type that implements Clone.
let semantics = TreeSemantics::<ExampleMeaning>::new();

// Retrieve the meaning of a node.
let meaning = semantics.get(&node);

// Assign meaning to a node.
semantics.set(&node, meaning);

// Delete the meaning of a node.
semantics.delete(&node);

// Check whether a node has a meaning.
let has_meaning = semantics.has(&node);
```

Almost every node type that is used behind a `Rc` container may be used as a node type in `TreeSemantics`, such as `Rc<Expression>`, `Rc<Directive>`, and `Rc<FunctionCommon>`.