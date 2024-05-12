use crate::ns::*;
use serde::{Serialize, Deserialize};

/// The `configuration {}` directive.
///
/// # Syntax
///
/// The directive consists of a block of `if..else` branches.
/// The top-level if statement takes a block, as well as its
/// optional else clause. The `configuration` directive
/// may consist of solely a block, in which case it is redundant.
#[derive(Clone, Serialize, Deserialize)]
pub struct ConfigurationDirective {
    pub location: Location,
    pub directive: Rc<Directive>,
}