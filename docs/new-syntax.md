# New Syntax

This parser adds certain new syntax to the language. Some come from Apache Royale, some from Samsung HARMAN, and some from other ECMAScript variants.

## Destructuring

```
const { x } = o;
[y] = o1;
```

**Non-null**: non-null operator is supported within destructuring patterns.

## Optional Chaining

```
o?.x
o?.[k]
o?.()
```

## Raw String Literal

Samsung HARMAN added a `@` prefix to string literals, designing them such that escape sequences are uninterpreted.

```
@""
```

## Triple String Literal

ECMAScript 4 introduced a triple string literal that spans multiple lines and ignores indentation.

```
const s =   """
            Text.
            """
s == "Text."
```

## Nullish Coalescing

```
x ?? y
x ??= y
```

## Non-null Operator

```
o!
```

## Keywords as Identifiers

Certain contexts allow for using any reserved word as an identifier, such as function names and variable names.

```
function default(): void {}
```

## Abstract attribute

The `abstract` attribute is valid at classes and methods.

## Static attribute

The `static` attribute is valid at classes.

## Type definition

```
type T1 = T2
```

## Enumeration definition

```
enum E1 {
    const M1
    const M2 = "m2"
    const M3 = ["m3", 10]

    function f1(): void {}
}
```

## Array Initializer

```
// Rest operator
[...o]
```

## Object initializer

```
// Rest operator
( { ...o } );

// Trailing comma
( { x: 10, } );

// Shorthand field
( { x } );

// Brackets field
( { ["x"]: 10 } );
```

## Generators

```
function f() {
    // Yield operator
    yield 10
}
```

## Asynchronous methods

```
function f() {
    // Await operator
    await f1()
}
```

## Switch Type Statement

ECMAScript 4 introduced a `switch type` statement.

```
switch type (o) {
    case (d: Date) {
        trace(d.valueOf())
    }
    default {
        trace("Not a Date")
    }
}
```

## Configuration Directive

`configuration { ... }` means conditional compilation with `if`, `else if` and `else` branches. A limited set of conditional expressions are valid and translate to a different syntactic construct:

```
configuration {
    if (k=3) {
        trace("k=3")
    } else {
        trace("k!=3")
    }
}
```

Conditional expressions:

```actionscript3
// Check whether constant is "true"
q::x
x
// Check whether constant is "v"
k="v"
k=v // QualifiedIdentifier == StringLiteral
// Check whether constant is not "v"
k!="v"
k!=v // QualifiedIdentifier != StringLiteral

x && y
x || y

(x)
!x
```

## Parameterized Types

```
class C1.<T> {}
```

## Function Type Expression

The function type expression is as is in ECMAScript fourth edition, but not including the `this` parameter.

A suffix `=` indicates an optional parameter.

```
type F = function(T, Number=, ...): T
type F = function(T, Number=, ...[T]): T
```

## Meta Properties

```
import.meta
```

## Non-Nullable Type Expression

```
type T1 = T!
```

## Nullable Type Expression

```
type T1 = T?
type T2 = ?T
```

## Void Type Expression

`void` is allowed as a type expression everywhere.

## Array Type Expression

```
type A1 = [T]
```

## Tuple Type Expression

```
type T1 = [E1, E2]
```

## Aliasing Imports

```
import x = ns.y;

// Open ns.* and set "ns1" to ns.*'s public namespace or
// NamespaceSet(public, internal)
// if enclosing package is equal
// ti the aliased package
import ns1 = ns.*;
ns1::y
```

## Negated In/Is

```
k not in o
v is not T
```

## Exponentiation

```
n ** p
```

## Function Body

Function bodies may consist of an expression, in which case, either:

* The offending token of the expression is inline.
* The offending token of the expression is in a line whose indentation is higher than that of the previous token.
* The offending token of the expression is `(`.

```
const f = function(): Number (10)
```

Recommendation: for function definitions consisting of an expression body, it is recommended to use a semicolon to prevent ambiguity with meta-data treated as brackets operators.

## Numeric Literal

* Binary literal
* Underscore separators

```
0b1011
10_000
```

## Regular Expression

* Line terminators allowed within a regular expression literal.

```
/(?:)
./m
```

## Float Suffix

Samsung HARMAN introduced a `f` suffix to the *NumericLiteral* productions.

```
0f
```
