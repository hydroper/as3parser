use std::collections::BTreeMap;

use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Mxml {
    pub location: Location,
    pub version: XmlVersion,
    pub encoding: String,
    pub content: Vec<Rc<MxmlContent>>,
}

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum XmlVersion {
    /// XML version 1.0.
    Version10,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MxmlElement {
    pub location: Location,
    pub name: MxmlName,
    /// Attribute list, including `xmlns` and `xmlns:` namespace prefixes.
    pub attributes: Vec<Rc<MxmlAttribute>>,
    /// The namespace mapping relative to the XML element.
    #[serde(skip)]
    pub namespace: Rc<MxmlNamespace>,
    pub content: Option<Vec<Rc<MxmlContent>>>,
    pub closing_name: Option<MxmlName>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MxmlAttribute {
    pub location: Location,
    /// Indicates whether the attribute is a `xmlns` or `xmlns:` attribute.
    pub xmlns: bool,
    pub name: MxmlName,
    /// Attribute value. The location data includes the quotes.
    pub value: (String, Location),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MxmlName {
    pub location: Location,
    /// The unresolved prefix of the name.
    pub prefix: Option<String>,
    pub name: String,
}

impl MxmlName {
    pub fn resolve_prefix(&self, namespace: &Rc<MxmlNamespace>) -> Result<String, MxmlNameError> {
        let Some(p) = self.prefix.as_ref() else {
            return if let Some(v) = namespace.get(MxmlNamespace::DEFAULT_NAMESPACE) {
                Ok(v)
            } else {
                Err(MxmlNameError::PrefixNotDefined(MxmlNamespace::DEFAULT_NAMESPACE.into()))
            };
        };
        if let Some(v) = namespace.get(p) {
            Ok(v)
        } else {
            Err(MxmlNameError::PrefixNotDefined(p.clone()))
        }
    }

    pub fn resolve_name(&self, namespace: &Rc<MxmlNamespace>) -> Result<(String, String), MxmlNameError> {
        let p = self.resolve_prefix(namespace)?;
        Ok((p, self.name.clone()))
    }

    pub fn equals_name(&self, other: &Self, namespace: &Rc<MxmlNamespace>) -> Result<bool, MxmlNameError> {
        if self.name != other.name {
            return Ok(false);
        }
        let p1 = self.resolve_prefix(namespace)?;
        let p2 = other.resolve_prefix(namespace)?;
        Ok(&p1 == &p2)
    }

    pub fn to_string(&self, namespace: &Rc<MxmlNamespace>) -> String {
        self.resolve_name(namespace).map(|(uri, localname)| format!("{uri}:{localname}")).unwrap_or("[error]".into())
    }
}

#[derive(Clone)]
pub enum MxmlNameError {
    PrefixNotDefined(String),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum MxmlContent {
    Characters((String, Location)),
    /// A CDATA construct, including the first `<![CDATA[` characters
    /// and the last `]]>` characters.
    CData((String, Location)),
    /// A comment construct, including the first `<!--` characters
    /// and the last `-->` characters.
    Comment((String, Location)),
    ProcessingInstruction {
        location: Location,
        name: String,
        data: Option<String>,
    },
    Element(Rc<MxmlElement>),
}

impl MxmlContent {
    pub fn location(&self) -> Location {
        match self {
            Self::Characters((_, l)) => l.clone(),
            Self::CData((_, l)) => l.clone(),
            Self::Comment((_, l)) => l.clone(),
            Self::ProcessingInstruction { location: l, .. } => l.clone(),
            Self::Element(e) => e.location.clone(),
        }
    }
}

/// Mapping of namespace prefixes.
#[derive(Clone, PartialEq)]
pub struct MxmlNamespace {
    parent: Option<Rc<MxmlNamespace>>,
    mappings: RefCell<BTreeMap<String, String>>,
}

impl Default for MxmlNamespace {
    fn default() -> Self {
        Self::new(None)
    }
}

impl MxmlNamespace {
    /// Returns the prefix used for the default XML namespace.
    pub const DEFAULT_NAMESPACE: &'static str = "";

    /// Constructs an empty set of namespace mappings.
    pub fn new(parent: Option<&Rc<MxmlNamespace>>) -> Self {
        let mut ns = Self {
            parent: parent.map(|p| p.clone()),
            mappings: RefCell::new(BTreeMap::new()),
        };
        if ns.parent.is_none() {
            ns.mappings.get_mut().insert(Self::DEFAULT_NAMESPACE.into(), "".into());
            ns.mappings.get_mut().insert("xmlns".into(), "http://www.w3.org/xml/xmlns".into());
        }
        ns
    }

    pub fn includes(&self, prefix: &str) -> bool {
        self.mappings.borrow().contains_key(prefix) || match &self.parent {
            Some(p) => p.includes(prefix),
            None => false,
        }
    }

    /// Retrieves the value of a prefix either in the actual
    /// set of mappings or in the parent set of mappings.
    pub fn get(&self, prefix: &str) -> Option<String> {
        if let Some(value) = self.mappings.borrow().get(prefix) {
            return Some(value.clone());
        }
        self.parent.as_ref().and_then(|p| p.get(prefix))
    }

    pub fn set(&self, prefix: &str, value: &str) {
        self.mappings.borrow_mut().insert(prefix.to_owned(), value.to_owned());
    }

    pub fn delete(&self, prefix: &str) -> bool {
        self.mappings.borrow_mut().remove(prefix).is_some()
    }

    pub fn clear(&self) {
        self.mappings.borrow_mut().clear();
    }

    /// Returns the actual set of prefix mappings.
    pub fn listing(&self) -> BTreeMap<String, String> {
        self.mappings.borrow().clone()
    }

    /// Returns a concatenation of the parent set of prefix mappings
    /// and the actual set of prefix mappings.
    pub fn full_listing(&self) -> BTreeMap<String, String> {
        let mut listing = self.parent.as_ref().map_or(BTreeMap::new(), |p| p.full_listing());
        listing.extend(self.listing());
        listing
    }
}
