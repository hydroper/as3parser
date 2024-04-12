use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ConfigurationDirective {
    pub location: Location,
    pub directive: Rc<Directive>,
}