use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct TypeDefinition {
    pub location: Location,
    pub asdoc: Option<Rc<AsDoc>>,
    pub attributes: Vec<Attribute>,
    pub left: (String, Location),
    pub right: Rc<Expression>,
}