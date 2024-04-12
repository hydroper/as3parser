use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct VariableDefinition {
    pub location: Location,
    pub asdoc: Option<Rc<AsDoc>>,
    pub attributes: Vec<Attribute>,
    pub kind: (VariableDefinitionKind, Location),
    pub bindings: Vec<Rc<VariableBinding>>,
}

#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VariableDefinitionKind {
    Var,
    Const,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SimpleVariableDefinition {
    pub location: Location,
    pub kind: (VariableDefinitionKind, Location),
    pub bindings: Vec<Rc<VariableBinding>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VariableBinding {
    pub destructuring: TypedDestructuring,
    pub initializer: Option<Rc<Expression>>,
}

impl VariableBinding {
    pub fn location(&self) -> Location {
        self.initializer.as_ref().map_or(self.destructuring.location.clone(), |init| self.destructuring.location.combine_with(init.location()))
    }
}