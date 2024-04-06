# Undefined type

The `UndefinedType` type kind represents the `undefined` type. The `undefined` type is literally different from `void`, however both contain the `undefined` value.

## Implicit conversions

`void` and `undefined` implicitly convert to each other.

## Supported methods

### `is_undefined_type()`

Returns true.

### `asdoc()`

Returns `None`.

## Supported traits

### `ToString`

The `to_string()` method returns `"undefined"`.