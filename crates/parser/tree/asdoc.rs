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
    Author(String),
    Copy(Rc<AsDocReference>),
    Created(String),
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
    Langversion(String),
    Param {
        name: String,
        description: String,
    },
    Playerversion(String),
    Private,
    Productversion(String),
    Return(String),
    See {
        reference: Rc<AsDocReference>,
        display_text: Option<String>,
    },
    Throws {
        class_reference: Rc<Expression>,
        description: Option<String>,
    },
    Version(String),
}

/// An ASDoc reference consisting of an optional base and
/// an optional instance property fragment (`#x`).
#[derive(Clone, Serialize, Deserialize)]
pub struct AsDocReference {
    /// Base expression.
    pub base: Option<Rc<Expression>>,
    /// Instance property fragment following the hash character.
    pub instance_property: Option<(String, Location)>,
}