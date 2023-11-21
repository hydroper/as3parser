# Name

The `Name` type kind represents an unique name consisting of a namespace and a string. `Name` objects are used frequently to represent the name of classes, variables, and miscellaneous other symbols.

`Name` types are interned in the `TypeHost` object.

## Supported methods

### `is_name()`

Returns `true`.

### `namespace()`

Returns the namespace to which the name belongs.

### `name_string()`

Returns the name's string.

## Supported traits

### `ToString`

The `to_string()` method applied to a `Name` type kind returns either of:

* `"anonymousNamespace::x"`
* `"x"`
* `"\"uri\"::x"`