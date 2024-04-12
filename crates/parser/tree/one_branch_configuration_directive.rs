use crate::ns::*;
use serde::{Serialize, Deserialize};

/// Represents a `CONFIG::x {}` directive.
#[derive(Clone, Serialize, Deserialize)]
pub struct OneBranchConfigurationDirective {
    pub location: Location,
    /// The qualifier, most commonly the `CONFIG` identifier.
    pub qualifier: (String, Location),
    /// The constant name without including the qualifier.
    pub constant_name: (String, Location),
    pub block: Rc<Block>,
}