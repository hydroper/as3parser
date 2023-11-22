# Class type

The `ClassType` type kind represents a class. It consists of:

* A `Name`
* An optional parent `Type`
  * It may be a `Package` type or
* A sequence of type parameters
* Static properties `Names` object
  * ECMA-262 `prototype` is a static read-only `prototype: *` property
* Prototype `Names` delegate
* Optional super class `Type`
  * The only class with no super class is `Object`
* A set of implemented interfaces
* A set of known subclasses
  * Used by the ASDoc tool
* Constructor function property
* Modifiers such as `final` and `static`
* Reserved namespaces (`public`, `private`, `protected`, `internal`)
* An external toggle
* Meta data

## Supported methods

### `name()`

The qualified name of this class.

### `parent()`

The parent of the class, or `None`. If any, it is a `Package` type.

### `set_parent()`

A setter for the `parent()` property.

### `type_params()`

An optional sequence of type parameters.

### `set_type_params()`

A setter for the `type_params()` property.

### `static_properties()`

Static properties of the class, as a `Names` object. It always defines:

```as3
public static const prototype: * = omittedInitializer;
```

### `prototype()`

A class instance delegate, also known as the class prototype object. This is not equivalent to the static `prototype` property, but rather the compile-time delegate of the class.

This returns a `Names` object.

## Supported traits

### `ToString`

The `to_string()` method returns the fully qualified name of the class (including its qualifier namespace) and any type arguments (`.<T1, ...TN>`).