# Processing Deviations

## Include Directive

In the original ActionScript 3, the `include` directive concatenates source text from another ActionScript file, which affects line and column information; in other words, it is not a reliable feature.

In this parser, the `include` directive contributes the syntactic structures from another ActionScript file, although the included ActionScript file is not allowed to contain zero or more `package` definitions.