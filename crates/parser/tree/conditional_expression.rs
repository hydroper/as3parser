use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ConditionalExpression {
    pub location: Location,
    pub test: Rc<Expression>,
    pub consequent: Rc<Expression>,
    pub alternative: Rc<Expression>,
}