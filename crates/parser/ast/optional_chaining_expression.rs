use crate::ns::*;
use serde::{Serialize, Deserialize};

/// An expression followed by optional chaining operations.
#[derive(Clone, Serialize, Deserialize)]
pub struct OptionalChainingExpression {
    pub location: Location,
    pub base: Rc<Expression>,
    /// Optional chaining operations.
    /// 
    /// An `OptionalChainingPlaceholder` node is is the topmost expression
    /// in the `expression` field.
    pub expression: Rc<Expression>,
}

/// Internal expression used as the topmost expression
/// of a sequence of optional chaining operations.
#[derive(Clone, Serialize, Deserialize)]
pub struct OptionalChainingPlaceholder {
    pub location: Location,
}