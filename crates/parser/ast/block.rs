use crate::ns::*;
use serde::{Serialize, Deserialize};

/// Block statement.
///
/// Block statements may occasionally be assigned a `BlockStatementSymbol`
/// to attach preprocessed plain meta-data.
#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    pub location: Location,
    /// Block meta-data for block statements.
    pub metadata: Option<Vec<Attribute>>,
    pub directives: Vec<Rc<Directive>>,
}