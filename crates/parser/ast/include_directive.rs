use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct IncludeDirective {
    pub location: Location,
    pub source: String,
    #[serde(skip)]
    pub nested_compilation_unit: RefCell<Option<Rc<CompilationUnit>>>,
    pub nested_directives: RefCell<Vec<Rc<Directive>>>,
}