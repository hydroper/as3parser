# TypeHost

> **NOTICE**: the verifier is not available.

The `TypeHost` object interns and caches built-in types. The only interned types are unions, complements, tuples, non-nullable types and types with arguments; record and function types are structurally checked against the other type.