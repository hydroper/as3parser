use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct IncludeDirective {
    pub location: Location,
    pub source: String,
    #[serde(skip)]
    pub nested_compilation_unit: Rc<CompilationUnit>,
    pub nested_packages: Vec<Rc<PackageDefinition>>,
    pub nested_directives: Vec<Rc<Directive>>,
}