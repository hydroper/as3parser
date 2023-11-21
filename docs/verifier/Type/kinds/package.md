# Package

The `Package` type kind represents a package as consisting of a name string (excluding dots), an optional parent package, a properties `Names` object, a collection of wildcard package exports (`export q.*;`), a collection of direct subpackages, a set of reserved namespaces (`public`, `internal`), and an optional ASDoc comment.

## Supported methods

### `name_string()`

The name string of the package. This is a single identifier; therefore it does not contain the dot character. For example, it returns `y` for a package `x.y`.

### `parent()`

The parent package of the package, or `None`.

### `properties()`

The properties of the package as a `Names` object.

### `wildcard_package_exports()`

A collection of wildcard package exports, as a vector of `Package` types. A wildcard package export is contributed from an `export` directive that exports a wildcard (`*`) item, such as in:

```as3
export q.*;
```

### `add_wildcard_package_export()`

Adds a wildcard package export. This method is used by the `export` directive.

### `subpackages()`

A collection of direct subpackages. For example:

```as3
package x {}
package x.y {}
package x.y.z {}
```

The `x` package has `x.y` as a direct subpackage.

### `asdoc()`

An optional ASDoc comment applying to the package.

### `set_asdoc()`

A setter for the `asdoc()` property.

### `add_subpackage()`

Adds a new subpackage with the specified name string, returning a `Package` type.

### `get_package()`

Lookups a package given an array of name strings.

```rust
// Lookups a subpackage "x", then a subpackage "y" from the "x" package,
// and finally a "z" subpackage from the "x.y" package.
let xyz: Option<Package> = type_host.global_package().get_package(["x", "y", "z"]);
```

### `get_packages_deep()`

Returns a vector containing the package and all of its subpackages recursively.

### `public_namespace()`

The `public` namespace of the package.

### `internal_namespace()`

The `internal` namespace of the package.

## Supported traits

### `ToString`

The `to_string()` method returns the fully-qualified name of the package using the dot delimiter.