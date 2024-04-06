# Enum type

The `EnumType` type kind represents an enum. It consists of:

* A `Name`
* An optional parent `Type`
  * It may be a `Package` type
* A `[Set]` modifier
* A number type
* Static properties `Names` object
  * ECMA-262 `prototype` is a static read-only `prototype: *` property
* Prototype `Names` delegate
* Mapping from member String to member Number
* Reserved namespaces (`private`)
* An external toggle
* Meta data

## Supported methods

### `is_enum_type()`

Returns true.

### `name()`

The qualified name of this enum.

### `parent()`

The parent of the enum, or `None`. If any, it is a `Package` type.

### `set_parent()`

A setter for the `parent()` property.

### `is_set()`

Whether the enum is a set enum. Set enums are indicated by the `[Set]` meta data, representing a combination of members, using bitwise representation.

### `enum_number_type()`

The enum's number type. Non set enums use `Number` as the default number type, and set enums use `uint` as the default number type. It can be changed through the `[Number(numberType)]` meta data, where `numberType` is one of the language's supported numeric types.

### `static_properties()`

Static properties of the enum, as a `Names` object. It always defines:

```as3
public static const prototype: * = omittedInitializer;
```

### `prototype()`

An enum instance delegate, also known as the enum prototype object. This is not equivalent to the static `prototype` property, but rather the compile-time delegate of the enum.

This returns a `Names` object.

### `enum_member_string_to_number()`

A mapping from a member's String to that member's Number.

### `private_namespace()`

The `private` namespace of the enum.

### `is_external()`

Whether the enum is external.

### `set_is_external()`

Setter for the `is_external()` property.

### `metadata()`

Collection of meta data.

### `add_metadata()`

Adds meta data.

## Supported traits

### `ToString`

The `to_string()` method returns the fully qualified name of the enum (including its qualifier namespace).