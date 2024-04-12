use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SuperStatement {
    pub location: Location,
    pub arguments: Vec<Rc<Expression>>,
}