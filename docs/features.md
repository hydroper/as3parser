# Features

This project would add a number of features to ActionScript 3.0. Some of them must be enabled explicitly as they affect existing sources and some allow for format migration, such as ASDoc comments.

## Global

A `global` constant, defined in a parent anonymous scope of the source, identifies the top-level package, which allows to resolve ambiguities in the lexical scope:

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

- constants from discriminant enums implicitly convert to discriminant enums,
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

The _noSwitchFallthroughs_ compiler option, when `true`, requires that non-empty switch cases contain a trailling `break` statement.

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

## Improved variables

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

The `enum` context keyword defines simple enumerations. Algebraic data types are complex and do not fit well with ActionScript, therefore the feature was oversimplified.

_Discriminant enumerations_: Discriminant enumerations consist of variants mapped to constants. With the `typeInference` compiler option on, constants such as string literals implicitly convert to discriminant enumerations.

```as3
// Defines a class `E`.
enum E {
    V1 = "v1";
    V2 = "v2";
    V3 = "v3";

    // The class representing the enum
    class {
        function f(): void {}
    }
}
const e: E = "v1";
```

_Subclass enumerations_: Subclass enumerations attach a subclass to the variants.

```as3
enum E {
    V1("v1") {
        override function f(): void {}
    }

    // Super class of all variants
    class {
        public const s: String;

        function E(s: String) {
            this.s = s;
        }

        function f(): void {}
    }
}
```

_`switch`_: A `switch` over an enumeration must be exhaustive and cover all variants with a trailling `break` statement.

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

    ["default"]: *,
};
```

A field may be read-only by preceding it with the context keyword `readonly`.

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

- `SWF`
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

## Arrow functions

Arrow functions with destructuring patterns are supported. They have the same semantics from ECMAScript arrow functions.

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

The sources are supplied to the compiler only through `sources.include` and `sources.exclude` arrays taking globbing patterns, and the main class for a SWF is specified by `sources.mainClass`:

```json
{
    "sources": {
        "include": ["src"],
        "exclude": [],
        "mainClass": "Main"
    }
}
```

A source may consist of multiple definitions in multiple packages.

## Migrations

- Migrate code to ASDoc 2
- Migrate code to nullability, by translating existing type annotations to include `null` explicitly
- Migrate code to improved variables