# Removed Syntax

## Include directive

The include directive is temporarily removed.

In the original ActionScript 3 it would concatenate source text from another ActionScript file, which affects line and column information; in other words, it is not a reliable feature.

In the previous version of this parser, the directive would contribute the syntactic structures from another ActionScript file, although it did not allow for the other ActionScript file to contain package definitions.