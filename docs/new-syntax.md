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
[Set]
enum E2 {
    const M1, M2
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

`configuration { ... }` means conditional compilation. A limited set of expressions are valid and translate to a different syntactic construct:

* `q::x` translates to an identifier whose name is literally `"q::x"` without a qualifier.
* `x` asks whether a constant `x` is present.
* `k="v"` translates to `k == "v"`.
* `k=v` translates to `k == "v"`.
* `k!="v"` goes as is.
* `k!=v` translates to `k != "v"`.

```
configuration {
    if (k=3) {
        trace("k=3")
    } else {
        trace("k!=3")
    }
}
```

## Embed Expression

```
const o = embed {
    source: "data.bin",
    type: ByteArray,
}
```

## Parameterized Types

```
class C1.<T> {}
```

## Function Type Expression

```
type F = function(a: T, b?: Number): void;
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
