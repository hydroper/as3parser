# Features

This project adds a number of features to ActionScript 3.0. Some of them must be enabled explicitly as they affect existing sources and some allow for format migration, such as ASDoc comments.

## Global

The _globalConstant_ compiler option adds a `global` constant identifying the top-level package, which allows resolving ambiguities in the lexical scope.

```json
{
    "compilerOptions": {
        "globalConstant": true
    }
}
```

Example:

```as3
package com.q {
    public class Vector {}
}
import com.q.*;

com.q.Vector;
global.Vector;
```

## Global import

The global package (also called top-level) is imported into a parent anonymous scope, differently from the previous ActionScript compilers. This does not break compatibility and the goal is to allow overriding names in the top-level package.

## Type inference

The _typeInference_ compiler option adds type inference for specific contexts such that:

- constants from tagged enums implicitly convert to tagged enums,
- variable bindings have the type of the assigned expression,
- function signatures without a return type annotation are taken as returning `void`, and
- `for` and `for each` perform type inference for the left variable based on the iterable.

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

## Nullability operators

- Postfix `!`
- Optional chaining: `?.`, `?.(...)` and `?.[...]`
- `??`, `??=`

## Destructuring patterns

Destructuring patterns are introduced to variable bindings. A pattern may have a postfix `!` for asserting that a base is non-null.

Examples of destructuring patterns:

```as3
const [x, y] = array; // array
({x, y} = p); // record
```

## Block-scoped variables

Variables are block-scoped and can shadow others in the same scope, if the compiler option `variables` is `"next"`:

```json
{
    "compilerOptions": {
        "variables": "next"
    }
}
```

A program demonstrating the effects of the above setting:

```as3
const x = 0;
{
    const x = 0;
}

// Shadowing
var x: Number = +Infinity;
var x: String = "";
```

## Import alias

```as3
import n2 = q.n1;

// Open `public` from `q` and alias it
import q2 = q.*;
q2::x

// Open `public` from `q` and its recursive subpackages
// and alias `q` and its recursive subpackages as `q3`
import q3 = q.**;
```

## Power expression

The following expression is equivalent to `Math.pow(n, p)`:

```as3
n ** p
```

## Type alias

The `type` context keyword is used for type alias definitions. A type alias may be generic.

```as3
type N2 = N1;
```

## Tuples

The tuple type is equivalent to an `Array` with additional compile-time type checking. It is expressed as `[E1, E2, ...En]`.

## Generics

A type or function may be generic. Type parameters in the output bytecode are simply the any type.

- `.<P1, ...Pn>`
- `where`

```as3
class C.<T> {}
```

## Keywords

_Dot tokens_: Keywords are valid identifiers after dot and `?.`.

_Intrinsics definitions_: The package `as3.intrinsics` allows defining properties whose name is a possibly reserved word.

Intrinsic definitions can be defined through `as3.intrinsics.define`. This function is processed by the compiler and is equivalent to either a function, variable, or virtual property definition. The usage is as follows:

```as3
// `var variableName: T;`
as3.intrinsics.define.<T>(public, "variableName", {
    // Optional setting: whether it is a constant variable.
    readOnly: true,

    // Optional setting: list of definition modifiers as strings
    modifiers: [],

    // Optional setting: initial value.
    value: initialValue,
});

// Virtual property
as3.intrinsics.define.<T>(public, "propertyName", {
    // Optional setting: list of definition modifiers as strings
    modifiers: [],

    // Getter
    get: () => v,

    // Setter
    set: v => {},
});

// Function
as3.intrinsics.define(public, "functionName", {
    // Optional setting: list of definition modifiers as strings
    modifiers: [],

    // Required setting
    signature: as3.intrinsics::Type.<SignatureType>,

    // Optional setting: the body. It must be specified
    // or omitted depending on the `native` modifier.
    body: signature => body,

    // Optional setting
    generics: {
        // Optional parameters to introduce as a
        // sequence of strings.
        params: [],

        // Optional map of default types to the parameters, as a
        // record from parameter string to an assigned type.
        // The assigned type may additionally be expressed through
        // `as3.intrinsics::Type.<T>`, where `T` is the target type.
        defaults: {},

        // Optional constraints as a record from parameter string
        // to a a list of types.
        constraints: {},
    },
});
```

The following `f` definition:

```as3
as3.intrinsics.define(public, "f", {
    signature: as3.intrinsics::Type.<() => void>,
    body: () => {},
    generics: {
        params: ["T"],
        defaults: {
            T: as3.intrinsics::Type.<() => void>,
        },
        constraints: {
            T: IEatable,
        },
    },
});
```

Is equivalent to the following `f` definition:

```as3
public function f.<T = () => void>(): void
    where T: IEatable
{}
```

## Namespace definition

The following code:

```as3
public namespace Q {
    public const x: Number = 64;
    public class C {}
}
```

translates to:

```as3
public class Q_C {}
public class Q {
    public static const x: Number = 64;
    public static const C: Class = Q_C;
}
```

## Object initializer

_Shorthand_: Shorthand fields equivalent to ECMAScript shorthand fields are added.

_Rest_: Rest components equivalent to ECMAScript rest components are added, taking a compatible iterable value.

_Trailling comma_: The object initializer is allowed to contain a trailling comma.

## Array initializer

_Rest_: Rest components equivalent to ECMAScript rest components are added, taking a compatible iterable value.

## Asynchronous and generators

A function containing the `await` operator is implicitly asynchronous; a function containing the `yield` operator is implicitly a generator.

- `await`
- `yield`
- `Promise.<T>`

## Collections

- Proper `Map.<K, V>` and `Set.<T>` types and their equivalents.
  - When K is string, due to conflicts, `Map` uses `$` prefix internally.
  - `Map.isEmpty` and `Map.nonEmpty` should be efficient and just use AVM `nextnameindex` once.
- Iterators

## Enums

The `enum` context keyword is used for a versatile enum definition:

- An enum definition can be used for algebraic data types and tagged enums;
- An enum definition can contain user function definitions in its block.

With the `typeInference` compiler option on, constants implicitly convert to tagged enums.

```as3
// Defines a class `E` with three static functions `X(...)`, `Y(...)` and `Z()`,
// and an instance method `f`.
enum E {
    X [ Number ];
    Y { x: E, y: Number };
    Z;

    // Custom definitions
    class {
        function f(): void {}
    }
}

const e = E.X([64]);
const e = E.Y({ x: e, y: 64 });
const e = E.Z();

// Pattern matching expression
const r = switch enum (e) {
    // Exhaustive
    case E.X [x] => "Got E.X",

    // Non-exhaustive
    case E.Y {x: E.Z, y} => "Got E.Y",

    // `default` can be used if all the previous patterns
    // are non-exhaustive.
    default => "Got anything else",
};

// Pattern matching if statement
if (const E.X[v] = e) {
    // v: Number
}

// Irrefutable pattern matching
const E.X[v] = e else {
    throw new Error("Unreachable");
};

// Pattern matching statement
switch enum (e) {
    case E.X [_] {
        trace("X");
    }
    default {
        trace("Y or Z");
    }
}

enum K {
    W = "w";
    S = "s";
}

const k: K = "w";
```

Matching patterns may be combined with a `|` separator as in `P1 | P2 | Pn`.

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

## Record type

The record type is simply a plain `Object` with compile-time type checking. Any field whose type accepts `undefined` — including nullable types — is optional.

```as3
type R = {
    // Optional field
    x?: String,
};
```

## Plain object record type

The compile-time `Record.<K, V>` type allows typing plain objects at compile-time. It is equivalent to `*`. The `K` type must be `String` or `Number`.

```as3
var o: Record.<Number, String> = {
    1: "s",
};
```

## Union type

The union type is simply the any type (`*`) with compile-time type checking.

```as3
type U = MemberA | MemberB;
type U2 =
    | MemberA
    | MemberB
    | MemberC;
```

## Complement type

The complement type is simply the any type (`*`) with compile-time type checking. It is used for adding properties to a set of existing record types.

All types contained in a complement type must be record types.

```as3
type C = R & {};
```

## Function type

The function type is simply the `Function` type with compile-time type checking.

```as3
type F = (a: T, b?: T, ...c) => void;
```

## Vector

Some improvements to the Vector type:

- You can assign an array initializer directly to a `Vector.<T>` typed variable.

## Template literal

The ECMAScript template literal is available:

```as3
f `${x}`;
```

## String literal type

String literals are valid types, equivalent to `String`, but with additional type checking.

## ASDoc

_ASDoc variant_: ASDoc comments can be configured to use an improved format that supports Markdown and facilitates writing comments. Set the compiler option `asdoc` to 2 (that is, ASDoc version 2) to use this facility:

```json
{
    "compilerOptions": {
        "asdoc": 2
    }
}
```

_Format migration_: Sources using ASDoc 1 format can be migrated to sources using ASDoc 2 format through the `asc migrate asdoc2` command.

_Places_: ASDoc comments can be applied to additional places, such as to type aliases and record fields.

## Meta-data

The compiler will eventually handle all of ActionScript meta-data and document them:

- `Embed`
- `Event`
- Some introduced by Apache Royale, such as `Bindable`

## Type relationship expressions

_Negated_: The context keyword `not` is used to indicate that an `is` or `instanceof` expression is negated. The `instanceof` operator may be preceded by `not`, while the `is` operator may be followed by `not`.

```as3
v is not T;
v not instanceof T;
```

_Right-hand side_: The right-hand side of an `as`, `is` or `instanceof` is still given an expression, not a type expression, despite the introduction of numerous type annotations. Furthermore, the right-hand side is limited to concrete types; types such as union types that map to another type are not allowed as the right-hand side of these operators.

_`as` result type_: The `as` operator returns `T?`.

## Iterators

Proper iterators may be supported futurely as this relies on the ActionScript API getting some updates.

Regardless, the following types are recognized as iterable:

- `Array`
- `Vector.<T>`
- `Proxy` subclasses that override `flash_proxy::nextNameIndex` and `flash_proxy::nextValue`.
- `Map.<K, V>`, yielding `[K, V]`
- `MapEntries.<K, V>`, yielding `[K, V]`
- `MapKeys.<K, V>`, yielding `K`
- `MapValues.<K, V>`, yielding `V`
- `WeakMap.<K, V>` and its iterators, yielding similiarly to `Map.<K, V>` and its iterators
- `Set.<T>`, yielding `T`
- `SetValues.<T>`, yielding `T`
- `WeakSet.<T>` and its iterators, yielding similiarly to `Set.<T>` and its iterators

## Sources

The sources are supplied to the compiler only through `sources.include` and `sources.exclude` arrays taking globbing patterns:

```json
{
    "sources": {
        "include": ["src"],
        "exclude": [],
    }
}
```

A source may consist of multiple definitions in multiple packages.