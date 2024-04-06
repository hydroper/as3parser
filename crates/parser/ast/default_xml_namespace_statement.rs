use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct DefaultXmlNamespaceStatement {
    pub location: Location,
    pub right: Rc<Expression>,
}