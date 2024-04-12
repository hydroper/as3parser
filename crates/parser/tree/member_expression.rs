use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct MemberExpression {
    pub location: Location,
    pub base: Rc<Expression>,
    pub identifier: QualifiedIdentifier,
}