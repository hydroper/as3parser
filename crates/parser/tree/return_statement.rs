use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ReturnStatement {
    pub location: Location,
    pub expression: Option<Rc<Expression>>,
}