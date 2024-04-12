use crate::ns::*;
use serde::{Serialize, Deserialize};

/// Super expression.
///
/// The super expression must always be followed by a property operator.
/// When the super expression appears in evaluation, the immediately
/// following property operator is limited to access a property from the base class
/// or invoke a method of the base class.
/// 
/// ```
/// super.f()
/// ```
#[derive(Clone, Serialize, Deserialize)]
pub struct SuperExpression {
    pub location: Location,
    pub object: Option<Vec<Rc<Expression>>>,
}