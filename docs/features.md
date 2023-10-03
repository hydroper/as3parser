# Features

This project adds a number of features to ActionScript 3.0. Some of them must be enabled explicitly as they affect existing sources.

## Global

The _globalConstant_ compiler option adds a `global` constant identifying the top-level package, which allows resolving ambiguities in the lexical scope.

```json
{
    "compilerOptions": {
        "globalConstant": true
    }
}
```

## Type inference

The _typeInference_ compiler option adds type inference for specific contexts such that:

- constants from tagged enums implicitly convert to tagged enums,
- variable bindings have the type of the assigned expression, and
- function signatures without a return type annotation are taken as returning `void`.

The compiler with the _typeInference_ option being true still emits warnings for untyped function parameters except:

- for every setter preceded by a corresponding getter, and
- for every getter preceded by a corresponding setter.

```json
{
    "compilerOptions": {
        "typeInference": true
    }
}
```

## No switch fallthroughs

The _noSwitchFallthroughs_ compiler option implicitly adds a `break` statement to non-empty switch cases.

```json
{
    "compilerOptions": {
        "noSwitchFallthroughs": true
    }
}
```

## Nullability

The _nullability_ compiler option has the following effects:

- types exclude `null` by default;
- the type annotations `T?` and `?T` indicate a nullable type;
- the type annotation `T!` indicates a non-nullable type;
- accessing a property from a nullable type first requires either a postfix `!` or an optional chaining operator;
- the postfix `!` operator asserts that a value is non-null.

```json
{
    "compilerOptions": {
        "nullability": true
    }
}
```

## Destructuring patterns

Destructuring patterns are introduced to variable bindings. A pattern may have a postfix `!` for asserting that a base is non-null.

Examples of destructuring patterns:

```as3
const [x, y] = array; // array
({x, y} = p); // record
```

## Let variables

Let variables are block-scoped and can shadow others in the same scope. `let` is used for writable variables; `let const` is used for read-only variables. Let variables must not shadow when directly within a type's block.

```as3
let x = 0;
let const y = 0;

// shadowing
let x: Number = +Infinity;
let x: String = "";
```

## Import alias

```as3
import n2 = q.n1;

// Open `public` from `q` and alias it
import q2 = q.*;
q2::x
```

## Type alias

The `type` context keyword is used for type alias definitions. A type alias may be generic.

```as3
type N2 = N1;
```

## Tuples

The tuple type is equivalent to an `Array` with additional compile-time type checking. It is expressed as `[E1, E2, ...En]`.

## Generics

A type or function may be generic.

- `.<P1, ...Pn>`
- `where`

```as3
class C.<T> {}
```

## Keywords

Keywords are valid identifiers after dot and `?.`.

## Asynchronous and generators

A function containing the `await` operator is implicitly asynchronous; a function containing the `yield` operator is implicitly a generator.

- `await`
- `yield`
- `Promise.<T>`

## Collections

- Proper `Map.<K, V>` and `Set.<T>` types and their equivalents.
  - When K is string, due to conflicts, uses `$` prefix internally.
- Iterators

## Primitive types

Proper aliases are provided for existing primitive types:

```as3
Int == int
NonNegInt == uint
Double == Number
```

If AVM introduces a single-precision floating point in the future, it is additionally aliased as `Single`.

## Enums

The `enum` context keyword is used for a versatile enum definition:

- An enum definition can be used for algebraic data types and tagged enums;
- An enum definition can contain user function definitions in its block.

With the `typeInference` compiler option on, constants implicitly convert to tagged enums.

```as3
// Defines a class `E` with two functions `X(...)`, `Y(...)` and `Z()`.
enum E {
    X [ Number ];
    Y { x: E, y: Number };
    Z;
}

let const e = E.X([64]);
let const e = E.Y({ x: e, y: 64 });
let const e = E.Z();

// Pattern matching
let const r = switch enum (e) {
    // Exhaustive
    E.X [x] => "Got E.X",

    // Non-exhaustive
    E.Y {x: E.Y [xx], y} => "Got E.Y",

    // `default` can be used if all the previous patterns
    // are non-exhaustive.
    default => "Got anything else",
};

enum K {
    W = "w";
    S = "s";
}
```

## Arrow functions

```as3
() => {}
```

## Triple string literal

The triple string literal allows for multiple lines and a specific indentation.

```as3
const s =
    """
    Broken
    paragraph.
    """;
```

## String.format

```as3
"{x}".format({ x: 10 })
"{1}".format([ 10 ])
```

## Record type

The record type is simply a plain `Object` with compile-time type checking. Any field whose type accepts `undefined` — including nullable types — is optional.

```as3
type R = {};
```

## Nullability operators

- Postfix `!`
- Optional chaining: `?.`, `?.(...)` and `?.[...]`
- `??`