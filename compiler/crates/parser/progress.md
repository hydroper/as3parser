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
  * [ ] XML element
  * [ ] XML list
  * [x] `public`
  * [x] `private`
  * [x] `protected`
  * [x] `internal`
  * [x] `(x)`
    * [x] If followed by `=>`, it is reinterpreted as an arrow function. It is done inside primary expressions and requires an operator precedence test first.
    * [x] If after `(` is `)` (that is, `()`) and the operator precedence includes arrow functions, parse an arrow function, carefully consuming the tokens.
    * [x] If followed by `::` and `x` is not a list expression, it is reinterpreted as a qualified identifier.
  * [ ] `...x`
  * [ ] Array initializer
  * [ ] Vector initializer (`new <T> [...]`)
  * [ ] Object initializer
  * [ ] Function expression
  * [ ] Arrow function
    * [ ] Fat arrow subexpression
  * [ ] Super expression
  * [ ] New expression
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
  * [ ] Embed expression
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