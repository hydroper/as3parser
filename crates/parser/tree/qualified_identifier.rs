use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct QualifiedIdentifier {
    pub location: Location,
    pub attribute: bool,
    pub qualifier: Option<Rc<Expression>>,
    pub id: QualifiedIdentifierIdentifier,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum QualifiedIdentifierIdentifier {
    Id((String, Location)),
    Brackets(Rc<Expression>),
}

impl QualifiedIdentifier {
    pub fn to_identifier_name_or_asterisk(&self) -> Option<(String, Location)> {
        if self.attribute || self.qualifier.is_some() {
            None
        } else {
            if let QualifiedIdentifierIdentifier::Id(id) = &self.id {
                Some(id.clone())
            } else {
                None
            }
        }
    }

    pub fn to_identifier_name(&self) -> Option<(String, Location)> {
        if self.attribute || self.qualifier.is_some() {
            None
        } else {
            if let QualifiedIdentifierIdentifier::Id(id) = &self.id {
                if id.0 == "*" { None } else { Some(id.clone()) }
            } else {
                None
            }
        }
    }

    pub fn is_identifier_token(&self) -> bool {
        self.qualifier.is_none() && !self.attribute && match &self.id {
            QualifiedIdentifierIdentifier::Id((id, _)) => id != "*",
            _ => false,
        }
    }
}