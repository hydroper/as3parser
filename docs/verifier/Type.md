# Type

The `Type` object represents a type, trait, package, namespace or namespace set.

## Memory management

The `Type` object is reference counted, therefore a type `WeakTypeRef` is supported for breaking circular references.

## Type safety

The `Type` object is an unification of various kinds. To make sure it is a certain type when storing it somewhere, use assertion layers such as `Class(t)` and `Variable(t)`.

```rust
use as3_verifier::*;

struct Data {
    foo_package: Package,
}

let data = Data { foo_package: Package(foo_type) };

// Take `Package` back into `Type` object
// by asserting that the contained `Type` is a package
let foo_type: Type = data.foo_package.get()
```