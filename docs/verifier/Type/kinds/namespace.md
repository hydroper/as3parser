# Namespace

The `Namespace` type kind represents a namespace used as a name qualifier in the ActionScript language. A namespace is either a reserved namespace, an anonymous namespace (`namespace N1;`), or a string assigned namespace (`namespace N1 = "https://foo.com";`). A namespace may contain an ASDoc comment.

String assigned namespaces are interned in the `TypeHost` object.

## Supported methods

### `is_namespace()`

Returns `true`.

### `is_reserved_namespace()`

Returns whether the namespace is a reserved namespace.

### `is_anonymous_namespace()`

Returns whether the namespace is an anonymous namespace.

### `is_string_assigned_namespace()`

Returns whether the namespace is a string assigned namespace.

### `reserved_namespace()`

Returns the kind of reserved namespace the namespace is, if it is a reserved namespace; otherwise `None`.

### `name_string()`

Returns the name string of the namespace, if it is an anonymous namespace. For example, the namespace `namespace N1;` has a `name_string() == "N1"`.

### `namespace_assigned_string()`

Returns the assigned string of the namespace, if it is a string assigned namespace. For example, the namespace `namespace w3c = "https://www.w3c.org";` has a `namespace_assigned_string() == "https://www.w3c.org"`.

### `asdoc()`

An optional ASDoc comment applying to the namespace.

### `set_asdoc()`

A setter for the `asdoc()` property.

## Supported traits

### `ToString`

The `to_string()` method returns either of:

* `"public"`
* `"private"`
* `"protected"`
* `"internal"`
* `"\"assignedString\""`
* `"anonymousNamespaceName"`