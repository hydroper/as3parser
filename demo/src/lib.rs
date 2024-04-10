use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use as3_parser::ns::*;

#[derive(Serialize, Deserialize)]
struct ParserResult {
    program: Option<Rc<Program>>,
    diagnostics: Vec<String>,
}

#[wasm_bindgen]
pub fn parse(input: &str) -> String {
    let compilation_unit = CompilationUnit::new(None, input.to_owned(), &CompilerOptions::new());
    let program = ParserFacade::parse_program(&compilation_unit);
    let mut diagnostics = vec![];
    compilation_unit.sort_diagnostics();
    for diagnostic in compilation_unit.nested_diagnostics() {
        diagnostics.push(diagnostic.format_english());
    }
    serde_json::to_string_pretty(&ParserResult {
        program,
        diagnostics,
    }).unwrap()
}