use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ComputedMemberExpression {
    pub location: Location,
    pub base: Rc<Expression>,
    /// ASDoc. Always ignore this field; it is used solely
    /// when parsing meta-data.
    pub asdoc: Option<Rc<AsDoc>>,
    pub key: Rc<Expression>,
}