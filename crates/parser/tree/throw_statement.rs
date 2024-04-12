use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ThrowStatement {
    pub location: Location,
    pub expression: Rc<Expression>,
}