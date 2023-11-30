# Progress list

## Compatibility

* [ ] Classes have the ECMAScript `prototype` property
* [ ] ECMAScript mode
* [ ] Strict mode

> From Adobe Flex documentation (`-es`):
>
> Using the ECMAScript edition 3 prototype-based object model lets you use
> untyped properties and functions in your application code. As a result, if you
> set the value of the `es` compiler option to true, you must set the strict
> compiler option to false. Otherwise, the compiler will throw errors.

> From Adobe Flex documentation (`-strict`):
>
> Prints undefined property and function calls; also performs compile-time
type checking on assignments and options supplied to method calls.

## Conditional compilation

* [ ] Support constructs such as `CONFIG::name { ... }` (configuration constant followed by block). When used in a type's block, it expands to the outer block.

## Strictness exceptions

Working with the following types is entirely dynamically typed and produces no errors such as undefined properties, function calls, and incompatible unary or binary operations:

- [ ] `*`
- [ ] `Object`

## Proxy

- [ ] Recognize the `flash_proxy` namespace and the `flash.utils.Proxy` class. The `flash_proxy` namespace is defined by the following URI:

```as3
namespace flash_proxy = "http://www.adobe.com/2006/actionscript/flash/proxy";
```

- [ ] Operations overridable by `flash_proxy` are considered in the respective contexts.

## `static protected`

- [ ] The `protected` access modifier, when combined with `static`, produces a `StaticProtected` namespace kind.

## Namespace definitions

- [ ] `namespace ns1;` produces `ns1` assigned to a new `internal` namespace
- [ ] `namespace ns1 = "http://www.adobe.com";` produces `ns1` assigned to an user namespace

## Expression verification

* [ ] Value expectation
  * [ ] Not all kinds of types are expected as value. Invalidate namespace sets, packages and certain types such as `*`. The result of expecting a value should produce a `Value` type kind.

## `import` behavior

- [ ] The `import` directive contributes to the collection of package imports of the scope `Frame` type. This is important for all imports, whether for wildcard, for recursive or for a specific property, to put package shadowing in effect.

## Fully package qualified references

- [ ] In a `o.x` expression (without `q::x`), imported packages shadows any topmost variable name under a lexical scope, conforming to ActionScript 3 specification.

- [ ] Reproduce the following diagnostic (optional):

```as3
package {
    import flash.display.Sprite;
    import flash.geom.*;
    public class Main extends Sprite {
        public function Main() {
            var flash: F = new F;
            trace(flash.geom.toString());
            // Error: Attempted access of inaccessible method toString through a reference with static type G.
        }
    }
}
package { public class F { public const geom: G = new G; } }
package { public class G { public function toString(): String { return ""; } } }
```

## External libraries

External libraries is typically code originating from an external SWC or ABC such as `playerglobal.swc`.

* [ ] Symbols have an `external` toggle.
* [ ] External mode. When the verifier is in external mode, any symbols it defines are external.

## Built-ins

The verifier does not include any built-in libraries by default. External libraries such as `playerglobal.swc` have to be loaded explicitly.

Global objects are resolved asynchronously and certain properties are required by the verifier occasionally.

## ABC

* [ ] Add functions to contribute definitions from an ABC structure from the `swf` crate to the verifier
  * [ ] Ignore duplicate definitions (common when including a SWC twice)

## Control flow

* [ ] Check control flow from AST

## Type conversions

* [ ] `C(v)` is a type conversion for most classes, except `Array` and `Vector.<T>`.

## Vector type

* [ ] The `Vector` class is treated specially for compatibility. The program or an external library defines the `__AS3__.vec.Vector` class and the compiler automatically turns it into a generic class with a single type parameter.

## Bytecode 

* [ ] Determine slot number of a trait based on hierarchy
* [ ] Determine dynamic dispatch optimization number

## Embed meta data

* [ ] `[Embed]` on variable definition
* [ ] `[Embed]` on class definition