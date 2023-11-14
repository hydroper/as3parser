use clap::Parser;
use file_paths::FlexPath;
use std::{env, fs, io};
use as3_parser::*;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[arg(short, long)]
    source_path: String,
}

fn main() -> io::Result<()> {
    let arguments = Arguments::parse();
    let source_path = FlexPath::from_n_native([env::current_dir().unwrap().to_string_lossy().into_owned().as_ref(), arguments.source_path.as_ref()]).to_string_with_flex_separator();

    // Canonicalize path
    // let source_path = std::path::Path::new(&source_path).canonicalize().unwrap().to_string_lossy().into_owned();

    let source_path_ast_json = FlexPath::new_native(&source_path).change_extension(".ast.json").to_string_with_flex_separator();
    let source_path_diagnostics = FlexPath::new_native(&source_path).change_extension(".diagnostics").to_string_with_flex_separator();
    let source_content = fs::read_to_string(&source_path)?;
    let source = Source::new(Some(source_path), source_content, &CompilerOptions::new());
    if let Some(program) = parser_facade::parse_program(&source) {
        fs::write(&source_path_ast_json, serde_json::to_string_pretty(&program).unwrap())?;
    } else {
        fs::write(&source_path_ast_json, "{}")?;
    }
    let mut diagnostics = vec![];
    source.sort_diagnostics();
    for diagnostic in source.recursive_diagnostics() {
        diagnostics.push(diagnostic.format_default());
    }
    fs::write(&source_path_diagnostics, diagnostics.join("\n"))?;
    Ok(())
}