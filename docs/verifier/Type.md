# Type

The `Type` object represents a type, trait, package, namespace or namespace set.

## Memory management

The `Type` object is garbage-collected using mark-and-sweep.

## Assertion

The `Type` object is an unification of various kinds. To make sure it is a certain type when storing it somewhere, use assertion layers such as `ClassType(t)` and `VariableProperty(t)`.

```rust
use as3_verifier::*;

struct Foo {
    foo_package: Package,
}

let foo = Foo { foo_package: Package(foo_type) };

// Take `Package` back into `Type` object
// by asserting that the contained `Type` is a package
let foo_type: Type = foo.foo_package.get();
```