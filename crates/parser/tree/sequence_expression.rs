use crate::ns::*;
use serde::{Serialize, Deserialize};

/// Sequence expression (`x, y`).
#[derive(Clone, Serialize, Deserialize)]
pub struct SequenceExpression {
    pub location: Location,
    pub left: Rc<Expression>,
    pub right: Rc<Expression>,
}