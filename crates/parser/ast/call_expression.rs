use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CallExpression {
    pub location: Location,
    pub base: Rc<Expression>,
    pub arguments: Vec<Rc<Expression>>,
}