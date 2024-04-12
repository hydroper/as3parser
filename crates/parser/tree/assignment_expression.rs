use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct AssignmentExpression {
    pub location: Location,
    pub compound: Option<Operator>,
    /// Assignment left-hand side.
    /// 
    /// If the left-hand side is an `ObjectInitializer` or an `ArrayLiteral`
    /// and there is no compound assignment, it is a destructuring pattern.
    pub left: Rc<Expression>,
    pub right: Rc<Expression>,
}