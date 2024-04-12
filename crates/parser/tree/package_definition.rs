use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PackageDefinition {
    pub location: Location,
    pub asdoc: Option<Rc<AsDoc>>,
    pub name: Vec<(String, Location)>,
    pub block: Rc<Block>,
}