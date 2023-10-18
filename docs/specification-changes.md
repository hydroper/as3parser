# Language specification changes

## Lexical structure

- Added punctuators:
  - `**`
    - Used by the power expression and by package recursive import aliases
  - `**=`
    - Used by power assignment
  - `?.`
    - Used by optional chaining
  - `??`
    - Null coalescing
  - `??=`
    - Null coalescing assignment
  - `=>`
    - Used by arrow functions
- Added context keywords:
  - `enum`
  - `not`
  - `readonly`
  - `type`
- Added literals:
  - Triple string literal
- Added escapes:
  - Unicode scalar escape (`\u{}`), present in string literals and identifiers

## Expressions

- Optional chaining
  - Creates a placeholder node from which further postfix operators may execute.