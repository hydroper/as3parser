# Feature syntax summary

The parser's major features are arrow functions and destructuring patterns from the ECMA 2015 edition. It implements the parsing phase of:

> Notice: this list is non-exhaustive.

* annotatable directives;
* ASDoc main body and tags recognition;
* directives;
* statements;
* expressions, E4X operations and type expressions.
* generics and constraints.

New annotatable directives:

* `TypeDefinition` (`type T2 = T1;`)
* `EnumDefinition : "enum" Identifier Block`

New statements:

* ECMA 4th `switch type`

New type expressions:

* complement (`R1 & R2`)
* union (`M1 | M2`)
* record (`{...}`)
* tuple (`[T1, T2, ...TN]`)
* function (`() => T`)
* ECMA 4th non-nullable (`T!`)
* ECMA 4th nullable (`T?`)
* `undefined`, `null`, `never`, `StringLiteral`, `NumericLiteral`, and parenthesized

Operators:

* Power (`**`, `**=`)
* Nullish-coalescing (`??`, `??=`)
* Optional chaining (`o?.qid`, `o?.[k]`, `o?.(...)`)

Enhancements:

* From Apache Royale Compiler
  * `import N2 = org.foo.bar.N;`
* `import q = org.foo.bar.*;` — `public` alias and opening it
  * Usage: `q::x`
* `import q = org.foo.bar.**;` — Recursive `public` namespace set alias and opening it
* Object initializer — Shorthand fields, brackets key, rest and trailing comma
* Array initializer — Rest

Non standard syntax:

* `o!` — Non-nullish assertion
* `x is not T`
* `x not instanceof T`
* `k not in o`
* `embed "filePath"` — Anonymous embedded data (`: T` or type inference)
* From the Royale Compiler
  * `IdentifierName` instead of `Identifier` for `VariableBinding` and `FunctionDefinition` names, excluding the case of arrow functions

Deviations:

* `IncludeDirective` resolves to `Directives`, and not a source text replacement. This is a benefit because lines and columns are kept, however `PackageDefinition` cannot appear in the included file currently.