use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct EnumDefinition {
    pub location: Location,
    pub asdoc: Option<Rc<AsDoc>>,
    pub attributes: Vec<Attribute>,
    pub is_set: bool,
    pub name: (String, Location),
    pub as_clause: Option<Rc<Expression>>,
    pub block: Rc<Block>,
}