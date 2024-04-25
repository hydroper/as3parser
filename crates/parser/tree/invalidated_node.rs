use crate::ns::*;
use serde::{Serialize, Deserialize};

/// Represents a construct that failed to parse.
#[derive(Clone, Serialize, Deserialize)]
pub struct InvalidatedNode {
    pub location: Location,
}