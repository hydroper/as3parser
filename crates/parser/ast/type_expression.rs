use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct NullableTypeExpression {
    pub location: Location,
    pub base: Rc<Expression>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NonNullableTypeExpression {
    pub location: Location,
    pub base: Rc<Expression>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AnyTypeExpression {
    pub location: Location,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VoidTypeExpression {
    pub location: Location,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ArrayTypeExpression {
    pub location: Location,
    pub expression: Rc<Expression>,
}

/// A tuple type expression consisting of at least two elements.
#[derive(Clone, Serialize, Deserialize)]
pub struct TupleTypeExpression {
    pub location: Location,
    pub expressions: Vec<Rc<Expression>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FunctionTypeExpression {
    pub location: Location,
    pub signature: FunctionSignature,
}