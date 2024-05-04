use crate::ns::*;
use serde::{Serialize, Deserialize};

/// The `public += ns.*;` directive.
/// Its `ImportSpecifier` is either `Wildcard` or `Recursive`.
#[derive(Clone, Serialize, Deserialize)]
pub struct PackageConcatDirective {
    pub location: Location,
    pub package_name: Vec<(String, Location)>,
    pub import_specifier: ImportSpecifier,
}