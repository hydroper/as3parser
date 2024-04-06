# Function type

The `FunctionType` represents a function type. A function type is equivalent to the `Function` class with additional type checking for its parameters and its return type.

## Implicit conversions

The `Function` class and function types implicitly convert to each other.

## Supported methods

### `is_function_type()`

Returns true.

### `function_params()`

Parameter sequence containing zero or more required parameters, zero or more optional parameters, and an optional rest parameter. The parameters consist of a simple unqualified name.

### `function_return()`

The function's return type.

## Supported traits

### `ToString`

The `to_string()` method returns a string in the form `(...) => T`.