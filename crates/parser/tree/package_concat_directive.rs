use crate::ns::*;
use serde::{Serialize, Deserialize};

/// The `public += ns.*;` directive.
#[derive(Clone, Serialize, Deserialize)]
pub struct PackageConcatDirective {
    pub location: Location,
    pub package_name: Vec<(String, Location)>,
    pub import_specifier: ImportSpecifier,
}