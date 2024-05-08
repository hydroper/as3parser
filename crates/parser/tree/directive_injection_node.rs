use crate::ns::*;
use serde::{Serialize, Deserialize};

/// Node that allows modification to the directive sequence.
#[derive(Clone, Serialize, Deserialize)]
pub struct DirectiveInjectionNode {
    pub location: Location,
    pub directives: RefCell<Vec<Rc<Directive>>>,
}