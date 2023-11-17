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