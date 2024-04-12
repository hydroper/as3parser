use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct UseNamespaceDirective {
    pub location: Location,
    pub expression: Rc<Expression>,
}