use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use as3_parser::ns::*;

#[derive(Serialize, Deserialize)]
struct ParserResult {
    program: Option<Rc<Program>>,
    mxml: Option<Rc<Mxml>>,
    css: Option<Rc<CssDocument>>,
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
pub fn parse(input: &str, source_type: &str) -> String {
    let compilation_unit = CompilationUnit::new(None, input.to_owned());

    let mut program: Option<Rc<Program>> = None;
    let mut mxml: Option<Rc<Mxml>> = None;
    let mut css: Option<Rc<CssDocument>> = None;

    let source_type = source_type.to_lowercase();

    if source_type == "mxml" {
        mxml = Some(ParserFacade(&compilation_unit, default()).parse_mxml());
    } else if source_type == "css" {
        css = Some(CssParserFacade(&compilation_unit, default()).parse_document());
    } else {
        program = Some(ParserFacade(&compilation_unit, default()).parse_program());
    }
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
        mxml,
        css,
        diagnostics,
    }).unwrap()
}