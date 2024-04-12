use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ForStatement {
    pub location: Location,
    pub init: Option<ForInitializer>,
    pub test: Option<Rc<Expression>>,
    pub update: Option<Rc<Expression>>,
    pub body: Rc<Directive>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ForInitializer {
    Expression(Rc<Expression>),
    VariableDefinition(Rc<SimpleVariableDefinition>),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ForInStatement {
    pub location: Location,
    pub each: bool,
    pub left: ForInBinding,
    pub right: Rc<Expression>,
    pub body: Rc<Directive>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ForInBinding {
    Expression(Rc<Expression>),
    VariableDefinition(Rc<SimpleVariableDefinition>),
}