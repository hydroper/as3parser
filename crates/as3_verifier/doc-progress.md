Type kinds:

- [x] Name
- [x] Package
- [x] Namespace
- [x] Namespace set
- [x] Any type
- [x] Undefined type
- [x] Void type
- [x] Never type
- [x] Class type
- [x] Interface type
- [x] Enum type
- [x] Function type
- [ ] Union type
- [ ] Complement type
- [ ] Tuple type
- [ ] Record type
- [ ] Nullable type
- [ ] Non-nullable type
* [ ] Type parameter
- [ ] Type with arguments
- [ ] String literal type 
- [ ] Number literal type
- [ ] Variable property
  - [ ] Read only
- [ ] Variable property from type with arguments
- [ ] Virtual property
- [ ] Virtual property from type with arguments
- [ ] Function property
  - [ ] Sequence of type parameters
- [ ] Function property with type arguments
- [ ] Function property from type with arguments
- [ ] Alias
  - [ ] The `Alias` type kind represents an alias to a type or property.
  - [ ] ASDoc
- [ ] Frame
  - [ ] The `Frame` type kind represents a lexical scope. A `Frame` contains an optional reference to a parent frame, a collection of package imports (specific properties, wildcard and recursive) and a `Names` object.
- [ ] Value
  - [ ] The `Value` type kind represents a value with a static type and various variations.
- [ ] Unresolved
  - [ ] The `Unresolved` type kind represents an unresolved symbol that is eventually replaced by another type as the verifier finishes tasks.

Names:

- [ ] The `Names` object represents a mapping from `Name` kind to `Type`.

Miscellaneous:

- [ ] `SimpleTypedName` (`a: T` as used in a function type)
- [ ] `TypeFactory` (belongs to a `TypeHost`)