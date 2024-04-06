# Void type

The `VoidType` type kind represents the `void` type. The `void` type is literally different from `undefined`, however both contain the `undefined` value.

## Implicit conversions

`void` and `undefined` implicitly convert to each other.

## Supported methods

### `is_void_type()`

Returns true.

### `asdoc()`

Returns `None`.

## Supported traits

### `ToString`

The `to_string()` method returns `"void"`.