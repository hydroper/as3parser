# TypeHost

The `TypeHost` object interns and caches built-in types. The only interned types are names, unions, complements, tuples, nullable types, non-nullable types and types with arguments; record and function types are structurally checked against the other type.