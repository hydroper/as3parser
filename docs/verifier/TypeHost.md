# TypeHost

The `TypeHost` object interns and caches built-in types and contains a factory for creating types. The only interned types are names, unions, complements, tuples, nullable types, non-nullable types, types with arguments, and string assigned namespaces; record and function types are structurally checked against the other type.