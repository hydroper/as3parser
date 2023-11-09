# Parsing

Details:

* [ ] Expressions
  * [ ] `ExpressionContext`
    * [ ] `min_precedence`
    * [ ] `allow_in`
    * [ ] `allow_assignment`
    * [ ] `with_type_annotation`
      * Conditional
      * Miscellaneous other cases in the other language project

Syntax:

* Expressions
  * [ ] `null`
  * [ ] `false`
  * [ ] `true`
  * [ ] Numeric
  * [ ] String
  * [ ] `this`
  * [ ] Regular expression
  * [ ] Qualified identifier
    * [ ] If followed by `=>`, it is reinterpreted as an arrow function.
    * [ ] Keywords are not reserved after `::`
    * [ ] Keywords are not reserved after `@`
  * [ ] XML markup
  * [ ] XML element
  * [ ] XML list
  * [ ] `public`
  * [ ] `private`
  * [ ] `protected`
  * [ ] `internal`
  * [ ] `(x)`
    * [ ] If followed by `=>`, it is reinterpreted as an arrow function. It is done inside primary expressions and requires an operator precedence test first.
    * [ ] If after `(` is `)` (that is, `()`) and the operator precedence includes arrow functions, parse an arrow function, carefully consuming the tokens.
  * [ ] `...x`
  * [ ] Array initializer
  * [ ] Vector initializer (`new <T> [...]`)
  * [ ] Object initializer
  * [ ] Function expression
  * [ ] Arrow function
    * [ ] It is parsed from either a non qualified identifier or a parentheses expression, as described in the previous items of this list.
  * [ ] Super expression
  * [ ] New expression
  * [ ] `o.x`
    * [ ] Keywords are not reserved after `.`
    * [ ] `public`, `private`, `protected`, `internal` are reserved before `::`
    * [ ] Keywords are not reserved after `@`
  * [ ] `o?.x`
    * [ ] Keywords are not reserved after `?.`
    * [ ] `public`, `private`, `protected`, `internal` are reserved before `::`
  * [ ] `o?.[k]`
  * [ ] `o?.(...)`
  * [ ] `o[k]`
  * [ ] `o.<...>`
  * [ ] `o.(condition)`
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