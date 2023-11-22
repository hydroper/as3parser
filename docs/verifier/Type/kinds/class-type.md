# Class type

The `ClassType` type kind represents a class. It consists of:

* A `Name`
* An optional parent `Type`
  * It may be a `Package` type or
* A sequence of type parameters
* Static properties `Names` object
  * `prototype` is a static read-only `prototype: *` property
* An instance `Delegate`
* Optional super class `Type`
  * The only class with no super class is `Object`
* A sequence of implemented interfaces
* Limited known subclasses
* Constructor function property
* Modifiers such as `final` and `static`
* Reserved namespaces (`public`, `private`, `protected`, `internal`)
* An external toggle
* Meta data

## Supported methods

## Supported traits

### `ToString`

The `to_string()` method returns the fully qualified name of the class (including its qualifier namespace) and any type arguments (`.<T1, ...TN>`).