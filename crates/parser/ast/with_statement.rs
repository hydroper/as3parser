use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct WithStatement {
    pub location: Location,
    pub object: Rc<Expression>,
    pub body: Rc<Directive>,
}