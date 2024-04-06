# TypeHost

The `TypeHost` object interns or caches miscellaneous types and contains a factory for creating types. The only interned types are names, unions, complements, tuples, nullable types, non-nullable types, types with arguments, user namespaces, and explicit namespaces; record and function types are structurally checked against the other type.