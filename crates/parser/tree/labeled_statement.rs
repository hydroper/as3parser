use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct LabeledStatement {
    pub location: Location,
    pub label: (String, Location),
    pub substatement: Rc<Directive>,
}