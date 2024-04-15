use crate::ns::*;
use serde::{Serialize, Deserialize};

/// Represents a `CONFIG::x ...` directive.
#[derive(Clone, Serialize, Deserialize)]
pub struct NormalConfigurationDirective {
    pub location: Location,
    /// The namespace, most commonly the `CONFIG` identifier.
    pub namespace: (String, Location),
    /// The constant name without including the qualifier.
    pub constant_name: (String, Location),
    pub directive: Rc<Directive>,
}