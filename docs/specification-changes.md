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
  - `enum`
  - `not`
  - `readonly`
  - `type`
- Added literals:
  - Triple string literal
    - Line breaks produce `\n`
    - Features a particular indentation handling
- Added escapes:
  - Unicode scalar escape (`\u{}`), present in string literals and identifiers
- String literal
  - Allowed escape sequences of line terminator

## Expressions

- Optional chaining
  - Creates a placeholder node from which further postfix operators may execute.