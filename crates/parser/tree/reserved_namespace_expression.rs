use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum ReservedNamespaceExpression {
    Public(Location),
    Private(Location),
    Protected(Location),
    Internal(Location),
}

impl ReservedNamespaceExpression {
    pub fn location(&self) -> Location {
        match self {
            Self::Public(l) => l.clone(),
            Self::Private(l) => l.clone(),
            Self::Protected(l) => l.clone(),
            Self::Internal(l) => l.clone(),
        }
    }

    pub fn to_attribute(&self) -> Attribute {
        match self {
            Self::Public(l) => Attribute::Public(l.clone()),
            Self::Private(l) => Attribute::Private(l.clone()),
            Self::Protected(l) => Attribute::Protected(l.clone()),
            Self::Internal(l) => Attribute::Internal(l.clone()),
        }
    }
}

impl ToString for ReservedNamespaceExpression {
    fn to_string(&self) -> String {
        match self {
            Self::Public(_) => "public".into(),
            Self::Private(_) => "private".into(),
            Self::Protected(_) => "protected".into(),
            Self::Internal(_) => "internal".into(),
        }
    }
}