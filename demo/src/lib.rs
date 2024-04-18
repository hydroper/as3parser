use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use as3_parser::ns::*;

#[derive(Serialize, Deserialize)]
struct ParserResult {
    program: Option<Rc<Program>>,
    diagnostics: Vec<ParserDiagnosticResult>,
}

#[derive(Serialize, Deserialize)]
struct ParserDiagnosticResult {
    warning: bool,
    column1: usize,
    column2: usize,
    line1: usize,
    line2: usize,
    message: String,
}

#[wasm_bindgen]
pub fn parse(input: &str) -> String {
    let compilation_unit = CompilationUnit::new(None, input.to_owned(), &CompilerOptions::default());
    let program = ParserFacade(&compilation_unit, default()).parse_program();
    let mut diagnostics = vec![];
    compilation_unit.sort_diagnostics();
    for diagnostic in compilation_unit.nested_diagnostics() {
        diagnostics.push(ParserDiagnosticResult {
            warning: diagnostic.is_warning(),
            column1: diagnostic.location().first_column() + 1,
            column2: diagnostic.location().last_column() + 1,
            line1: diagnostic.location().first_line_number(),
            line2: diagnostic.location().last_line_number(),
            message: diagnostic.format_message_english(),
        });
    }
    serde_json::to_string_pretty(&ParserResult {
        program,
        diagnostics,
    }).unwrap()
}