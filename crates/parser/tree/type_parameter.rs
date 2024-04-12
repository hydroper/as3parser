use crate::ns::*;
use serde::{Serialize, Deserialize};

/// A type parameter as in `function f.<T>(): void {}`.
#[derive(Clone, Serialize, Deserialize)]
pub struct TypeParameter {
    pub location: Location,
    pub name: (String, Location),
}