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
  * [x] Unary operators
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
  * [x] `x, y`
  * [x] `x: T`
    * [x] WithTypeAnnotation is only parsed if `with_type_annotation` is true
    * [x] WithTypeAnnotation is only parsed in postfix precedence
  * [x] Embed expression
* Type expressions
  * [x] Identifier
  * [x] `(x)`
    * If followed by `=>`, reinterpret it as a function type
      * [x] If `(` is followed by `)`, it is a function type
      * [x] If `(` is followed by `...`, it is a function type
      * [x] If subexpression is an identifier token or an `idToken?` type expression and it is followed by either `:` or `,`, it is a function type
  * [x] `o.x`
  * [x] Tuple
  * [x] Record
  * [x] Any
  * [x] Void
  * [x] Never
  * [x] Undefined
  * [x] `T?`
  * [x] `?T`
  * [x] `T!`
  * [x] Function (`(...) => T`)
  * [x] String literal
  * [x] Numeric literal
  * [x] `m1 | m2` (... `| mN`)
  * [x] `|` prefix
  * [x] `x & y`
  * [x] `o.<T1, ...TN>`
* Reserved words
  * [x] Reserved words are valid identifiers in destructuring patterns from variable bindings
  * [x] Reserved words are valid identifiers in function definition names, including regular functions, getters and setters
* Statements
  * Parsing statement returns (*node*, *semicolonInserted*)
  * [x] Empty
  * [x] Super
  * [x] Block
  * [x] If
  * [x] Switch
  * [x] Switch type
  * [x] Do
  * [x] While
  * [x] For
    * [x] `for`
    * [x] `for..in`
    * [x] `for each`
  * [x] With
  * [x] Continue
  * [x] Break
  * [x] Return
  * [x] Throw
  * [x] Try
  * [x] Expression statement
  * [x] Labeled statement
  * [x] Default XML namespace
  * Substatement
    * [x] Simple variable declaration
* Directives
  * [x] Include
  * [x] Import
  * [x] Export
  * [x] Use namespace
  * Parse annotatable definitions from
    * [x] Meta data (`statement.list_meta_data_expressions()` and postprocessing)
    * [x] Modifiers
  * [x] Variable definition
    * [x] Ensure modifiers are correctly specified
  * [x] Function definition
    * [x] Ensure modifiers are correctly specified
    * [x] Constructor
    * [x] Getter
    * [x] Setter
  * [x] Type definition
  * [x] Class definition
  * [x] Enum definition
  * [x] Interface definition
  * [x] Namespace definition
* [x] Program
  * [x] Packages
  * [x] Top level directives
* [x] ASDoc
* `parser_facade`
  * [x] `parse_expression`
  * [x] `parse_type_expression`
  * [x] `parse_program`