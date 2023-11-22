# Class type

The `ClassType` type kind represents a class. It consists of:

* A `Name`
* An optional parent `Type`
  * It may be a `Package` type or
* A sequence of type parameters
*
*
*
*
*
*
*
*
*
*
*

## Supported methods

## Supported traits

### `ToString`

The `to_string()` method returns the fully qualified name of the class (including its qualifier namespace) and any type arguments (`.<T1, ...TN>`).