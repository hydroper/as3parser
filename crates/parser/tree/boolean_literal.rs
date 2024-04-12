use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct BooleanLiteral {
    pub location: Location,
    pub value: bool,
}