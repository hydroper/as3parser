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
  * [ ] Function expression
  * [ ] Arrow function
    * [ ] Fat arrow subexpression
  * [x] Super expression
  * [x] New expression
  * [ ] `o.x`
    * [ ] Keywords are not reserved after `.`
    * [ ] `public`, `private`, `protected`, `internal` are reserved before `::`
  * [ ] `o?.x`
    * [ ] Keywords are not reserved after `?.`
    * [ ] `public`, `private`, `protected`, `internal` are reserved before `::`
  * [ ] `o?.[k]`
  * [ ] `o?.(...)`
  * [ ] `o[k]`
  * [ ] `o.<...>`
  * [ ] `o.(condition)`
    * [ ] If it is followed by `::` and condition is not a list expression, it is reinterpreted as a qualified identifier.
  * [ ] `o..x`
    * [ ] Keywords are not reserved after `..`
  * [ ] `f()`
  * [ ] Unary operators
    * [ ] Postfix
      * [ ] `!`
      * [ ] `++`
      - [ ] `--`
  * [ ] Binary operators
    * [ ] `not in`
    * [ ] `not instanceof`
    * [ ] `is not`
    * [ ] `??`
  * [ ] Conditional
  * [ ] `pattern = v`
  * [ ] `x, y`
  * [ ] `x: T`
  * [x] Embed expression
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