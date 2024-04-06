use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct AsDoc {
    pub location: Location,
    pub main_body: Option<(String, Location)>,
    pub tags: Vec<(AsDocTag, Location)>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum AsDocTag {
    Copy(Rc<AsDocReference>),
    Default(String),
    Deprecated {
        message: Option<String>,
    },
    Event {
        name: String,
        description: String,
    },
    EventType(Rc<Expression>),
    Example(String),
    InheritDoc,
    Internal(String),
    Param {
        name: String,
        description: String,
    },
    Private,
    Return(String),
    See {
        reference: Rc<AsDocReference>,
        display_text: Option<String>,
    },
    Throws {
        class_reference: Rc<Expression>,
        description: Option<String>,
    },
}

/// An ASDoc reference consisting of an optional base and
/// an optional instance property fragment (`#x`).
#[derive(Clone, Serialize, Deserialize)]
pub struct AsDocReference {
    /// Base expression.
    pub base: Option<Rc<Expression>>,
    /// Instance property fragment following the hash character.
    pub instance_property: Option<String>,
}