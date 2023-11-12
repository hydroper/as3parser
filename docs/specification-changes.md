# Language specification changes

This document is incomplete yet.

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
  - `embed`
  - `enum`
  - `not`
  - `readonly`
  - `type`
  - `where`
- Added literals:
  - Triple string literal
    - Line breaks produce `\n`
    - Features a particular indentation handling based on ECMAScript 4
- Added escapes:
  - Unicode scalar escape (`\u{}`), present in string literals and identifiers
- String literal
  - Allowed escape sequences of line terminator

## Expressions

- Previously undocumented expression: `new <T>[]`
- Optional chaining
  - Creates a base node from which postfix operators may execute.

## Miscellaneous

- Destructuring