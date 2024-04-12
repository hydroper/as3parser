use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ParenExpression {
    pub location: Location,
    pub expression: Rc<Expression>,
}