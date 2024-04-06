use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct XmlExpression {
    pub location: Location,
    pub element: Rc<XmlElement>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct XmlMarkupExpression {
    pub location: Location,
    pub markup: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct XmlListExpression {
    pub location: Location,
    pub content: Vec<Rc<XmlElementContent>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct XmlElement {
    pub location: Location,
    pub name: XmlTagName,
    pub attributes: Vec<Rc<XmlAttribute>>,
    pub attribute_expression: Option<Rc<Expression>>,
    pub content: Option<Vec<Rc<XmlElementContent>>>,
    pub closing_name: Option<XmlTagName>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum XmlTagName {
    Name((String, Location)),
    Expression(Rc<Expression>),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct XmlAttribute {
    pub location: Location,
    pub name: (String, Location),
    pub value: XmlAttributeValue,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum XmlAttributeValue {
    Value((String, Location)),
    Expression(Rc<Expression>),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum XmlElementContent {
    XmlText((String, Location)),
    XmlMarkup((String, Location)),
    XmlElement(Rc<XmlElement>),
    Expression(Rc<Expression>),
}