use std::env;
use as3_parser::ns::*;

fn main() {
    // Define source path
    let source_path = env::current_dir().unwrap().join("Example.as").to_string_lossy().into_owned();

    // Read source content
    let source_content = include_str!("Example.as").to_owned();

    // Create compilation unit
    let compilation_unit = CompilationUnit::new(Some(source_path), source_content, &CompilerOptions::default());

    // Parse program
    let program = ParserFacade(&compilation_unit, default()).parse_program();
    visit_program(&program);

    // Report diagnostics
    compilation_unit.sort_diagnostics();
    for diagnostic in compilation_unit.nested_diagnostics() {
        println!("{}", diagnostic.format_english());
    }
}

fn visit_program(program: &Rc<Program>) {
    for package in program.packages.iter() {
        for directive in package.block.directives.iter() {
            // directive: Rc<Directive>

            match directive.as_ref() {
                Directive::ClassDefinition(defn) => {
                    visit_class(&defn);
                },

                _ => {},
            }
        }
    }
}

fn visit_class(defn: &ClassDefinition) {
    for directive in defn.block.directives.iter() {
        if let Directive::VariableDefinition(defn) = directive.as_ref() {
            // Print any found main body and @private tags
            if let Some(asdoc) = &defn.asdoc {
                print_asdoc(asdoc);
            }
        }
    }
}

fn print_asdoc(asdoc: &Rc<AsDoc>) {
    if let Some((text, loc)) = &asdoc.main_body {
        println!("Found main body at {}:{}\n\n{}\n\n", loc.first_line_number(), loc.first_column() + 1, text);
    }
    for (tag, loc) in &asdoc.tags {
        if matches!(tag, AsDocTag::Private) {
            println!("Found @private tag at {}:{}\n\n", loc.first_line_number(), loc.first_column() + 1);
        }
    }
}