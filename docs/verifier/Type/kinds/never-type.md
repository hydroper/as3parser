# Never type

The `NeverType` type kind represents the `never` type. The `never` type contains no value and is a result of a function that always throws or that does not interrupt execution annotated with the `never` type. Such function's control flow has to be analyzed beforehand to determine if it is valid.

## Supported methods

### `is_never_type()`

Returns true.

### `asdoc()`

Returns `None`.

## Supported traits

### `ToString`

The `to_string()` method returns `"never"`.