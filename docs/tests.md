# Tests

The parser is not fully tested yet. You can contribute single tests by running the following in the repository's root directory:

```sh
# Generates two files:
# - testName.ast.json
# - testName.diagnostics
cargo run --bin as3_parser -- --source-path tests/testName/testName.as
```

The content of the generated test files have the following characteristics:

- If `testName.as` was invalidated by a syntax error, `testName.ast.json` should be an empty `{}` object.
- If `testName.as` is valid, `testName.diagnostics` should be empty or contain only warnings.