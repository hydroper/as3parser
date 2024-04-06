# Parsing tests

To test parsing a program producing output to the command line, run:

```
cargo run --bin as3_parser_test -- --source-path tests/parser/Demo.as
```

To test parsing a program producing output to two files `.ast.json` and `.diag`, run:

```
cargo run --bin as3_parser_test -- --source-path tests/parser/Demo.as --file-log
```