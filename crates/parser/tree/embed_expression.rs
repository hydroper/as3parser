use crate::ns::*;
use serde::{Serialize, Deserialize};

/// The `embed {...}` expression.
/// 
/// It is semantically assigned an `EmbedValue` symbol.
#[derive(Clone, Serialize, Deserialize)]
pub struct EmbedExpression {
    pub location: Location,
    pub description: ObjectInitializer,
}