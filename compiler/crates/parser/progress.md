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
    * If followed by `=>`, it is reinterpreted as an arrow function.
  * [ ] XML markup
  * [ ] XML element
  * [ ] XML list
  * [ ] `public`
  * [ ] `private`
  * [ ] `protected`
  * [ ] `internal`
  * [ ] `(x)`
    * If followed by `=>`, it is reinterpreted as an arrow function.
  * [ ] `...x`
  * [ ] Array initializer
  * [ ] Vector initializer (`new <T> [...]`)
  * [ ] Object initializer
  * [ ] Function expression
  * Arrow function
    * [ ] It is parsed from either a non-qualified identifier or a parentheses expression.
    * [ ] Parse empty parameters: `() => x`
  * [ ] Super expression
  * [ ] New expression
  * [ ] `o.x`
  * [ ] `o?.x`
  * [ ] `o?.[k]`
  * [ ] `o?.(...)`
  * [ ] `o[k]`
  * [ ] `o.<...>`
  * [ ] `o.(condition)`
  * [ ] `o..x`
  * [ ] `f()`
  * [ ] Unary operators
    * [ ] Postfix
      * [ ] `!`
      * [ ] `++`
      - [ ] `--`
  * [ ] Binary operators
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