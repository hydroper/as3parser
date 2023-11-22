# Namespace

The `Namespace` type kind represents a namespace used as a name qualifier in the ActionScript language. A namespace is either a system namespace, an user namespace (`namespace ns1 = "http://www.adobe.com";`), or an explicit namespace. A namespace may contain an ASDoc comment.

## User namespaces

In the ActionScript 3 bytecode, user namespaces map to `namespace_info` structures with the `CONSTANT_Namespace` constant.

User namespaces are interned in the `TypeHost` object.

## Explicit namespaces

The origin of explicit namespaces is currently unknown, therefore explicit namespaces are unexplored, but they exist at the ActionScript 3 bytecode. In the ActionScript 3 bytecode, explicit namespaces map to `namespace_info` structures with the `CONSTANT_ExplicitNamespace` constant.

As with user namespaces, explicit namespaces are interned in the `TypeHost` object.

## Supported methods

### `is_namespace()`

Returns `true`.

### `is_system_namespace()`

Returns whether the namespace is a system namespace.

### `is_user_namespace()`

Returns whether the namespace is an user namespace.

### `is_explicit_namespace()`

Returns whether the namespace is an explicit namespace.

### `is_string_assigned_namespace()`

Returns whether the namespace is a string assigned namespace.

### `system_namespace()`

Returns the kind of system namespace the namespace is, if it is a system namespace; otherwise `None`.

It includes not only `public`, `private`, `protected`, or `internal`, but also `static protected`.

### `namespace_string()`

Returns the assigned string of the namespace, if it is an user namespace or explicit namespace. For example, the namespace `namespace w3c = "http://www.w3c.org";` has a `namespace_string() == "http://www.w3c.org"`.

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
* `"\"string\""`