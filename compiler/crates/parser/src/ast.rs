use std::rc::Rc;
use crate::*;

pub struct QualifiedIdentifier {
    pub attribute: bool,
    pub qualifier: Option<Rc<Expression>>,
    pub name: IdentifierOrBrackets,
}

pub enum IdentifierOrBrackets {
    Identifier(String),
    Brackets(Rc<Expression>),
}

pub struct Expression {
    pub location: Location,
    pub kind: ExpressionKind,
}

pub enum ExpressionKind {
    NullLiteral,
    BooleanLiteral(bool),
    NumericLiteral(f64),
    StringLiteral(String),
    ThisLiteral,
    RegExpLiteral {
        body: String,
        flags: String,
    },
    QualifiedIdentifier(QualifiedIdentifier),
    XMLMarkup(String),
    XMLElement(XMLElement),
    XMLList(Vec<XMLElementContent>),
    ReservedNamespace(ReservedNamespace),
    ParenExpression(Rc<Expression>),
    /// Present as part of an array initializer only.
    /// This expression is not valid in other contexts.
    RestExpression(Rc<Expression>),
    ArrayInitializer {
        /// Element sequence possibly containing RestExpressions.
        elements: Vec<Rc<Expression>>,
        type_annotation: Option<Rc<TypeAnnotation>>,
    },

    /// Used as a base for optional chaining operators from
    /// which subsequent postfix operators may evaluate.
    OptionalChainingPlaceholder,
}

pub enum XMLElementContent {
    Interpolation(Rc<Expression>),
    Markup(String),
    Text(String),
    XMLElement(XMLElement),
}

pub struct XMLElement {
    pub opening_tag_name: XMLTagName,
    pub attributes: Vec<XMLAttributeOrInterpolation>,
    pub content: Vec<XMLElementContent>,
    pub closing_tag_name: Option<XMLTagName>,
}

pub enum XMLTagName {
    Name(String),
    Interpolation(Rc<Expression>),
}

pub enum XMLAttributeOrInterpolation {
    Attribute(XMLAttribute),
    Interpolation(Rc<Expression>),
}

pub struct XMLAttribute {
    pub name: String,
    pub value: XMLAttributeValueOrInterpolation,
}

pub enum XMLAttributeValueOrInterpolation {
    Value(String),
    Interpolation(Rc<Expression>),
}

pub enum ReservedNamespace {
    Public,
    Private,
    Protected,
    Internal,
}