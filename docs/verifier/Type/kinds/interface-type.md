# Interface type

The `InterfaceType` type kind represents an `interface` definition. It consists of:

* A `Name`
* An optional parent `Type`
  * It may be a `Package` type
* A sequence of type parameters
* Prototype `Names` delegate
* A set of super interfaces
* A set of known implementors
  * Used by the ASDoc tool
* Reserved namespace (`interface_block_namespace`)
  * Similiar to `public`, but an user namespace and not a system namespace
* An external toggle
* Meta data

## Supported methods

### `name()`

The qualified name of this interface.

### `parent()`

The parent of the interface, or `None`. If any, it is a `Package` type.

### `set_parent()`

A setter for the `parent()` property.

### `type_params()`

An optional sequence of type parameters.

### `set_type_params()`

A setter for the `type_params()` property.

### `prototype()`

An interface instance delegate, also known as the interface prototype object.

This returns a `Names` object.

### `super_interfaces()`

A collection of super interfaces, possibly containing `Unresolved` types.

### `add_super_interface()`

Adds a super interface.

### `replace_super_interface()`

Replaces a super interface. This is required when replacing `Unresolved` by another type.

### `known_implementors()`

Set of known implementors. This set is used by the ASDoc tool.

### `add_known_implementor()`

Adds a known implementor.

### `interface_block_namespace()`

The namespace used in the `interface` block. It is an user namespace, not a system namespace.

### `is_external()`

Whether the interface is external.

### `set_is_external()`

Setter for the `is_external()` property.

### `metadata()`

Collection of meta data.

### `add_metadata()`

Adds meta data.

## Supported traits

### `ToString`

The `to_string()` method returns the fully qualified name of the interface (including its qualifier namespace) and any type arguments (`.<T1, ...TN>`).