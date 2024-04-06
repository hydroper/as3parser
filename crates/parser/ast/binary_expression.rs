use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct BinaryExpression {
    pub location: Location,
    pub operator: Operator,
    pub left: Rc<Expression>,
    pub right: Rc<Expression>,
}