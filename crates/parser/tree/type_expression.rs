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
    pub parameters: Vec<Rc<FunctionTypeParameter>>,
    pub result_type: Option<Rc<Expression>>,
}

/// ```plain
/// function(T, T=, ...)
/// function(...[T])
/// ```
#[derive(Clone, Serialize, Deserialize)]
pub struct FunctionTypeParameter {
    pub location: Location,
    pub kind: ParameterKind,
    /// Possibly `None` for the rest parameter.
    pub type_expression: Option<Rc<Expression>>,
}