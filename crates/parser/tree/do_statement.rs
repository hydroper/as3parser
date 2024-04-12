use crate::ns::*;
use serde::{Serialize, Deserialize};

/// The `do..while` statement.
#[derive(Clone, Serialize, Deserialize)]
pub struct DoStatement {
    pub location: Location,
    pub body: Rc<Directive>,
    pub test: Rc<Expression>,
}