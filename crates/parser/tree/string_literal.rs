use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct StringLiteral {
    pub location: Location,
    pub value: String,
}