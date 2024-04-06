use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ContinueStatement {
    pub location: Location,
    pub label: Option<(String, Location)>,
}