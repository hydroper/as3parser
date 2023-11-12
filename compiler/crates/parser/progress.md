# Parsing

Details:

* [x] Expressions
  * [x] `ExpressionContext`
    * [x] `min_precedence`
    * [x] `allow_in`
    * [x] `allow_assignment`
    * [x] `with_type_annotation`
      * Conditional
      * Miscellaneous other cases in the other language project

Syntax:

* Expressions
  * [x] `null`
  * [x] `false`
  * [x] `true`
  * [x] Numeric
  * [x] String
  * [x] `this`
  * [x] Regular expression
  * [x] Lexical qualified identifier
    * [x] Starts with
      * [x] `(` (handled in the parentheses expression)
      * [x] Reserved namespace
      * [x] Identifier
      * [x] `@`
      * [x] `*`
  * [x] XML markup
  * [x] XML element
  * [x] XML list
  * [x] `public`
  * [x] `private`
  * [x] `protected`
  * [x] `internal`
  * [x] `(x)`
    * [x] If followed by `=>`, it is reinterpreted as an arrow function. It is done inside primary expressions and requires an operator precedence test first.
    * [x] If after `(` is `)` (that is, `()`) and the operator precedence includes arrow functions, parse an arrow function, carefully consuming the tokens.
    * [x] If followed by `::` and `x` is not a list expression, it is reinterpreted as a qualified identifier.
  * [x] `...x`
  * [x] Array initializer
  * [x] Vector initializer (`new <T> [...]`)
  * [x] Object initializer
  * [x] Function expression
  * [x] Arrow function
  * [x] Super expression
  * [x] New expression
  * [x] `o.x`
  * [x] `o?.x`
  * [x] `o?.[k]`
  * [x] `o?.(...)`
  * [x] `o[k]`
  * [x] `o.<...>`
  * [x] `o.(condition)`
    * [x] If it is followed by `::` and condition is not a list expression, it is reinterpreted as a qualified identifier.
  * [x] `o..x`
  * [x] `f()`
  * [ ] Unary operators
    * [x] `await`
    * [x] `yield`
    * [x] Prefix
    * [x] Postfix
      * [x] `!`
      * [x] `++`
      * [x] `--`
  * [x] Binary operators
    * [x] `not in`
    * [x] `not instanceof`
    * [x] `is not`
    * [x] `??`
  * [x] Conditional
  * [x] `pattern = v`
    * Only parse assignment if `allow_assignment` is true
  * [x] `pattern compound= v`
  * [ ] `x, y`
  * [ ] `x: T`
    * [ ] WithTypeAnnotation is only parsed if `with_type_annotation` is true
    * [ ] WithTypeAnnotation is only parsed in postfix precedence
  * [x] Embed expression
* Reserved words
  * [ ] Reserved words are valid identifiers in destructuring patterns from variable bindings
  * [ ] Reserved words are valid identifiers in function definition names, including regular functions, getters and setters
* FunctionCommon
  * [ ] When parsing it, push and pop from the `activations` stack
  * [ ] Call `validate_function_parameter_list`
* Statements
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
* Directives
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
* [ ] ASDoc
  * After parsing a ASDoc annotatable item, parse its ASDoc comment after checking if there is at least one comment in the Source and whether `last_comment.is_asdoc(item_location)` is true, resulting in an `AsDoc` value.