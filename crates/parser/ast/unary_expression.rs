use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct UnaryExpression {
    pub location: Location,
    pub operator: Operator,
    pub expression: Rc<Expression>,
}