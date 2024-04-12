use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct ObjectInitializer {
    pub location: Location,
    pub fields: Vec<Rc<InitializerField>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum InitializerField {
    Field {
        name: (FieldName, Location),
        /// Non-null operator used for destructuring.
        non_null: bool,
        value: Option<Rc<Expression>>,
    },
    Rest((Rc<Expression>, Location)),
}

impl InitializerField {
    pub fn location(&self) -> Location {
        match self {
            Self::Field { ref name, ref value, .. } => {
                value.clone().map_or(name.1.clone(), |v| name.1.combine_with(v.location()))
            },
            Self::Rest((_, ref l)) => l.clone(),
        }
    }

    pub fn shorthand(&self) -> Option<&QualifiedIdentifier> {
        if let Self::Field { name, .. } = self {
            if let FieldName::Identifier(qid) = &name.0 {
                Some(qid)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum FieldName {
    Identifier(QualifiedIdentifier),
    Brackets(Rc<Expression>),
    StringLiteral(Rc<Expression>),
    NumericLiteral(Rc<Expression>),
}

impl FieldName {
    pub(crate) fn id(&self) -> Option<&QualifiedIdentifier> {
        let Self::Identifier(id) = &self else {
            return None;
        };
        Some(id)
    }

    pub fn id_equals(&self, name: &str) -> bool {
        self.id().map(|name1| name == name1.to_identifier_name_or_asterisk().map(|id| id.0.clone()).unwrap_or("".into())).unwrap_or(false)
    }
}