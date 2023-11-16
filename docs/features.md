# Features

This ActionScript 3 parser adds several syntax constructs from TypeScript, ECMAScript 4 and Apache Royale Compiler. Notice that some of the descriptions in this document are not specific to this parser but to the verifier, which is not integrated with the parser.

The parser adds the following major enhancements to the language:

* Destructuring
* Arrow functions

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
- function signatures without a return type expression are taken as returning `void`, and
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
- the type expressions `T?` and `?T` indicate a nullable type;
- the type expression `T!` indicates a non-nullable type;
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

Nullability operators consider a value to be null when it is either `undefined` or `null`.

* Postfix `!`
* Optional chaining: `?.`, `?.(...)` and `?.[...]`
* `??`, `??=`

## Destructuring patterns

Destructuring patterns are introduced to variable bindings. A pattern may have a postfix `!` for asserting that a base is non-null.

Examples of destructuring patterns:

```as3
const [x, y] = array; // array
({x, y} = p); // record
```

## Enhanced variables

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
- Use the context keyword `where` to add generic constraints. In a function definition, `where` may appear after the return annotation.

```as3
class C.<T> where T: S + M {}
```

## Reserved words

Reserved words are valid identifiers after dot, `?.`, `::` and `@` at property operators and lexical references.

An identifier may be used as a valid reserved word in destructuring patterns from variable definitions, in function definition names, in an object initializer's fields, and in a record type's fields.

```as3
public function for(): void {
    // "for" function
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

## Static classes

Static classes contain the `static` modifier. Such classes are not allowed to be instantiated at compile time.

## Enums

The `enum` context keyword defines simple enumerations. Algebraic data types are complex and do not fit well with ActionScript, therefore the feature was oversimplified.

Simple enumerations consist of variants mapped to (*string*, *number*) pairs. With the `typeInference` compiler option on, string literals implicitly convert to enumerations.

```as3
// Defines a class `E`.
enum E {
    const V1 = "v1";
    const V2 = "v2";
    const V3 = "v3";

    public function f(): void {}
}
const e: E = "v1";
```

The constant definition of a member can assign a string, a number, a `[string, number]` pair or a `[number, string]` pair to customize the member constants. The string and number can be obtained through `toString()` and `valueOf()` respectively. The number and string are determined automatically if omitted.

```as3
e.valueOf(); // number
e.toString(); // string
```

The number type defaults to `Number`. It can be altered through the `[Number(T)]` metadata.

An enum class defines a static method `from(v)` that takes either a string or a compatible number and returns an object of its type. This method throws a `TypeError` if the value does not match a member; for set enums, it will ignore non matching bits.

```as3
try {
    const e = E.from(value);
} catch (error: TypeError) {
    //
}
```

_`switch`_: A `switch` over an enumeration must be exhaustive and cover all members with a trailling `break` statement.

## Set enums

Set enums consist of combinatory members, represented as bitwise flags. Such enums are defined with the `[Set]` metadata. Set enums benefit from type inference.

```as3
[Set]
enum E {
    const V1;
    const V2;
    const V3;
}
const v: E = ["v1", "v2", "v3"];
const v: E = {v1: true, v2: true, v3: true};
v.include("v2").exclude("v2").toggle("v2").filter("v2").has("v2");
```

## `switch type`

```as3
switch type (v) {
    case (v: T) {
        // v: T
    }
    default {
        //
    }
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

## Plain record type

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

Enhancements to the Vector type:

- You can assign an array initializer directly to a `Vector.<T>` typed variable with type inference.

## String literal type

String literals are valid types, equivalent to `String`, but with additional type checking.

## ASDoc

*ASDoc variant*: ASDoc comments can be configured to use an enhanced format that supports Markdown and facilitates writing comments. Set the compiler option `asdoc` to 2 (that is, ASDoc version 2) to use this facility:

```json
{
    "compilerOptions": {
        "asdoc": 2
    }
}
```

_Format migration_: Sources using ASDoc 1 format can be migrated to sources using ASDoc 2 format through the `asc migrate asdoc2` command.

_Places_: ASDoc comments can be applied to additional places, such as to type aliases and record fields.

## Meta data

Meta data are the same as of the Adobe ActionScript compiler and include syntax such as:

* keyless entries, either in the form `s` or `"s"`, such as in `[D(s)]` and `[D("s")]`;
* qualified names, in the form `q::x`, such as in `[q::x]`.

The compiler will eventually handle all of ActionScript meta-data and document them:

- `SWF`
- `Embed`
- `Event`
- `Exclude`
- Some introduced by Apache Royale, such as `Bindable`

## Type relationship expressions

_Negated_: The context keyword `not` is used to indicate that an `is` or `instanceof` expression is negated. The `instanceof` operator may be preceded by `not`, while the `is` operator may be followed by `not`.

```as3
v is not T;
v not instanceof T;
```

_Right-hand side_: The right-hand side of an `as`, `is` or `instanceof` is still given an expression, not a type expression, despite the introduction of numerous type expressions. Furthermore, the right-hand side is limited to concrete types; types such as union types that map to another type are not allowed as the right-hand side of these operators.

_`as` result type_: The `as` operator returns `T?`.

## `in` operator

_Negated_:

```as3
k not in o;
```

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

## Embed expression

The `embed` context keyword is used by embed expressions. They can be used to directly import local binaries or text.

```as3
const ba: flash.utils.ByteArray = embed "./data.bin";
```

## Include directive

The include directive is altered to not replace source text and instead contribute directives to the respective source, maintaining source locations.

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

## Miscellaneous

*Added type expressions*:

```as3
undefined
never
```

## Migrations

- Migrate code to ASDoc 2
- Migrate code to nullability, by translating existing type expressions to include `null` explicitly and adding `!` assertion where necessary
- Migrate code to enhanced variables