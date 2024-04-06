# Type

The `Type` object represents a type or one of several traits. Unsupported operations on `Type` either return a placeholder value or panic.

## Memory management

The `Type` object is reference counted in an arena. `Type` itself is a weak reference; it contains an internal `Weak<TypeKind>` inside, and operations over `Type` will upgrade it to a strong reference internally. The `TypeHost` always holds a strong reference to each `TypeKind`.

## Assertion

The `Type` object is an unification of various kinds. To make sure it is a certain type when storing it somewhere, use assertion layers such as `ClassType(t)` and `VariableProperty(t)`.

```rust
use as3_verifier::*;

struct Foo {
    foo_package: Package,
}

let foo = Foo { foo_package: Package(foo_type) };

// Assertion layers implement `Deref` targetting `&Type`.
println!("{:?}", foo.foo_package.parent());

// Take `Package` back into `Type` object
// by asserting that the contained `Type` is a package.
let foo_type: Type = *foo.foo_package;
```