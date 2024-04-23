use crate::ns::*;
use serde::{Serialize, Deserialize};

/// Block statement.
#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    pub location: Location,
    pub directives: Vec<Rc<Directive>>,
}