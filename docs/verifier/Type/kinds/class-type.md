# Class type

The `ClassType` type kind represents a class. It consists of:

* A `Name`
* An optional parent `Type`
  * It may be a `Package` type
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
* Reserved namespaces (`private`, `protected`, `static protected`)
* An external toggle
* Meta data

## Implicit conversions

* `Object` and any other type implicitly convert to each other.
* Subclass implicitly converts to super class
* Implementor implicitly converts to implementing interface
* `C.<T>` implicitly converts to `C.<*>`

## Generics

* Referring to `C` is equivalent to `C.<*>`.
* `C.<T>` is equivalent to `C`.

## Supported methods

### `is_class_type()`

Returns true.

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

### `super_class()`

The super class or `None`. The only class with no super class is `Object`.

The super class may be an `Unresolved` type.

### `set_super_class()`

A setter for the `super_class()` property.

### `implemented_interfaces()`

Set of implemented interfaces, possibly containing `Unresolved` types.

### `add_implemented_interface()`

Adds an implemented interface to the end of the `implemented_interfaces()` collection.

### `replace_implemented_interface()`

Replaces an implemented interface. This is used for replacing `Unresolved` by another type.

### `known_subclasses()`

Set of known subclasses. This set is used by the ASDoc tool.

### `add_known_subclass()`

Adds a known subclass.

### `constructor_function()`

An optional constructor function.

### `set_constructor_definition()`

Setter for the `constructor_function()` property.

### `is_static()`

Whether the class is static.

### `set_is_static()`

Setter for the `is_static()` property.

### `is_final()`

Whether the class is final.

### `set_is_final()`

Setter for the `is_final()` property.

### `private_namespace()`

The `private` namespace of the class.

### `protected_namespace()`

The `protected` namespace of the class.

### `static_protected_namespace()`

The `static protected` namespace of the class.

### `is_external()`

Whether the class is external.

### `set_is_external()`

Setter for the `is_external()` property.

### `metadata()`

Collection of meta data.

### `add_metadata()`

Adds meta data.

## Supported traits

### `ToString`

The `to_string()` method returns the fully qualified name of the class (including its qualifier namespace) and any type arguments (`.<T1, ...TN>`).