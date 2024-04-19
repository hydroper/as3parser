use crate::ns::*;
use serde::{Serialize, Deserialize};

/// The `configuration {}` directive.
///
/// # Syntax
///
/// The directive consists of a block
/// of `if..else` branches, whose
/// condition is one of the following expressions:
/// 
/// ```plain
/// // Check whether constant is "true"
/// q::x
/// x
/// // Check whether constant is "v"
/// k="v"
/// k=v // QualifiedIdentifier == StringLiteral
/// // Check whether constant is not "v"
/// k!="v"
/// k!=v // QualifiedIdentifier != StringLiteral
///
/// x && y
/// x || y
///
/// (x)
/// !x
/// ```
#[derive(Clone, Serialize, Deserialize)]
pub struct ConfigurationDirective {
    pub location: Location,
    pub directive: Rc<Directive>,
}