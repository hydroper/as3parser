use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct VectorLiteral {
    pub location: Location,
    pub element_type: Rc<Expression>,
    pub elements: Vec<Element>,
}