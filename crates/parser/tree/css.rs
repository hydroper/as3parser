use std::{marker::PhantomData, str::FromStr};

use crate::ns::*;
use num_traits::ToPrimitive;
use serde::{Serialize, Deserialize};

/// CSS3 selector combinators. Only *descendant* is supported at the
/// moment.
/// 
/// See also: [CSS3 selectors: combinators](http://www.w3.org/TR/css3-selectors/#combinators).
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CssCombinatorType {
    Descendant,
    Child,
    Preceded,
    Sibling,
}

impl ToString for CssCombinatorType {
    /// Symbol that represents the combinator type.
    fn to_string(&self) -> String {
        match self {
            Self::Descendant => " ".into(),
            Self::Child => ">".into(),
            Self::Preceded => "+".into(),
            Self::Sibling => "~".into(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum CssDirective {
    Invalidated(InvalidatedNode),
    FontFace(CssFontFace),
    NamespaceDefinition(CssNamespaceDefinition),
    Rule(CssRule),
}

impl CssDirective {
    pub fn location(&self) -> Location {
        match self {
            Self::Invalidated(v) => v.location.clone(),
            Self::FontFace(v) => v.location.clone(),
            Self::NamespaceDefinition(v) => v.location.clone(),
            Self::Rule(v) => v.location.clone(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum CssPropertyValueNode {
    Invalidated(InvalidatedNode),
    Array(CssArrayPropertyValue),
    Color(CssColorPropertyValue),
    Number(CssNumberPropertyValue),
    RgbColor(CssRgbColorPropertyValue),
    String(CssStringPropertyValue),
    Text(CssTextPropertyValue),

    ClassReference(CssClassReferencePropertyValue),
    PropertyReference(CssPropertyReferencePropertyValue),
    Url(CssUrlPropertyValue),
    Local(CssLocalPropertyValue),
    Embed(CssEmbedPropertyValue),
}

impl CssPropertyValueNode {
    pub fn location(&self) -> Location {
        match self {
            Self::Invalidated(v) => v.location.clone(),
            Self::Array(v) => v.location.clone(),
            Self::Color(v) => v.location.clone(),
            Self::Number(v) => v.location.clone(),
            Self::RgbColor(v) => v.location.clone(),
            Self::String(v) => v.location.clone(),
            Self::Text(v) => v.location.clone(),
            Self::ClassReference(v) => v.location.clone(),
            Self::PropertyReference(v) => v.location.clone(),
            Self::Url(v) => v.location.clone(),
            Self::Local(v) => v.location.clone(),
            Self::Embed(v) => v.location.clone(),
        }
    }

    pub fn as_array(&self) -> Option<&CssArrayPropertyValue> {
        let Self::Array(v) = self else { return None; };
        Some(v)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum CssSelector {
    Invalidated(InvalidatedNode),
    Base(CssBaseSelector),
    Combinator(CssCombinatorSelector),
}

impl CssSelector {
    pub fn location(&self) -> Location {
        match self {
            Self::Invalidated(v) => v.location.clone(),
            Self::Base(v) => v.location.clone(),
            Self::Combinator(v) => v.location.clone(),
        }
    }
}

/// Array type property values are comma-separated values in CSS properties.
///
/// For example:
///
/// ```css
/// fillColors: #FFFFFF, #CCCCCC, #FFFFFF, #EEEEEE;
/// ```
#[derive(Clone, Serialize, Deserialize)]
pub struct CssArrayPropertyValue {
    pub location: Location,
    /// List of `CssPropertyValue`s.
    pub elements: Vec<Rc<CssDirective>>,
}

/// A CSS base selector.
#[derive(Clone, Serialize, Deserialize)]
pub struct CssBaseSelector {
    pub location: Location,
    pub namespace_prefix: Option<(String, Location)>,
    pub element_name: Option<(String, Location)>,
    pub conditions: Vec<Rc<CssSelectorCondition>>,
}

/// Supported condition types for [`CssSelectorCondition`].
#[derive(Clone, Serialize, Deserialize)]
pub enum CssSelectorCondition {
    Invalidated(InvalidatedNode),
    /// For example: `s|Label.className`
    Class((String, Location)),
    /// For example: `s|Label#idValue`
    Id((String, Location)),
    /// For example: `s|Label:loadingState`
    Pseudo((String, Location)),
    /// For example: `s|Label::loadingState`
    PseudoElement((String, Location)),
    /// For example: `s|Panel:not(:first-child)`
    Not {
        location: Location,
        condition: Rc<CssSelectorCondition>,
    },
    /// For example: `s|Label[loadingState]`
    Attribute {
        location: Location,
        name: (String, Location),
        operator: CssAttributeOperator,
        value: Option<(String, Location)>,
    },
}

impl CssSelectorCondition {
    pub fn location(&self) -> Location {
        match self {
            Self::Invalidated(v) => v.location.clone(),
            Self::Class((_, l)) => l.clone(),
            Self::Id((_, l)) => l.clone(),
            Self::Pseudo((_, l)) => l.clone(),
            Self::PseudoElement((_, l)) => l.clone(),
            Self::Not { location, .. } => location.clone(),
            Self::Attribute { location, .. } => location.clone(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CssAttributeOperator {
    Equals,
    BeginsWith,
    EndsWith,
    Contains,
    ListMatch,
    HreflangMatch,
}

impl ToString for CssAttributeOperator {
    fn to_string(&self) -> String {
        match self {
            Self::Equals => "=".into(),
            Self::BeginsWith => "^=".into(),
            Self::EndsWith => "$=".into(),
            Self::Contains => "*=".into(),
            Self::ListMatch => "~=".into(),
            Self::HreflangMatch => "|=".into(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CssColorPropertyValue {
    pub location: Location,
    pub color_int: u32,
}

impl CssColorPropertyValue {
    pub fn from_hex(location: Location, token_text: &str) -> Self {
        assert!(token_text.starts_with("#"), "Invalid color: {token_text}");
        let mut token_text = token_text.to_owned();
        if token_text.len() == 4 {
            let mut six = String::new();
            let chars: Vec<_> = token_text.chars().collect();
            six.push('#');
            six.push(chars[1]);
            six.push(chars[1]);
            six.push(chars[2]);
            six.push(chars[2]);
            six.push(chars[3]);
            six.push(chars[3]);
            token_text = six;
        }
        Self {
            location,
            color_int: u32::from_str_radix(&token_text, 16).unwrap(),
        }
    }

    pub fn text(&self) -> String {
        self.location.text()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CssNumberPropertyValue {
    pub location: Location,
    pub value: f64,
    pub unit: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CssRgbColorPropertyValue {
    pub location: Location,
    pub color_int: u32,
}

impl CssRgbColorPropertyValue {
    pub fn from_raw_arguments(location: &Location, raw_arguments: &[String]) -> Result<Self, ParserError> {
        Ok(CssRgbColorPropertyValue {
            location: location.clone(),
            color_int: (Self::parse_component(&raw_arguments[0])? << 16)
                    |  (Self::parse_component(&raw_arguments[1])? << 8)
                    |   Self::parse_component(&raw_arguments[2])?,
        })
    }

    fn parse_component(input: &str) -> Result<u32, ParserError> {
        let i = input.find('%');
        let v: u32;
        if let Some(i) = i {
            let percent = f64::from_str(&input[..i]).map_err(|_| ParserError::Common)?.clamp(0.0, 100.0);
            v = (255.0 * (percent / 100.0)).round().to_u32().ok_or(ParserError::Common)?;
        } else if input.contains('.') {
            let ratio = f64::from_str(input).map_err(|_| ParserError::Common)?.clamp(0.0, 1.0);
            v = (255.0 * ratio).round().to_u32().ok_or(ParserError::Common)?;
        } else {
            v = u32::from_str(input).map_err(|_| ParserError::Common)?;
        }
        Ok(v.clamp(0, 255))
    }
}

/// A CSS text is a string value written without quotes.
#[derive(Clone, Serialize, Deserialize)]
pub struct CssStringPropertyValue {
    pub location: Location,
    pub value: String,
}

/// A CSS text is a string value written without quotes.
#[derive(Clone, Serialize, Deserialize)]
pub struct CssTextPropertyValue {
    pub location: Location,
    pub value: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CssClassReferencePropertyValue {
    pub location: Location,
    /// Name or "null".
    pub name: (String, Location),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CssPropertyReferencePropertyValue {
    pub location: Location,
    pub name: (String, Location),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CssUrlPropertyValue {
    pub location: Location,
    pub url: (String, Location),
    pub format: Option<(String, Location)>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CssLocalPropertyValue {
    pub location: Location,
    pub name: (String, Location),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CssEmbedPropertyValue {
    pub location: Location,
    pub entries: Vec<Rc<CssEmbedEntry>>,
}

/// Represents a key-value entry for an `Embed` function call property value.
/// It may be a keyless entry.
#[derive(Clone, Serialize, Deserialize)]
pub struct CssEmbedEntry {
    pub key: Option<(String, Location)>,
    pub value: (String, Location),
}

/// A CSS selector containing a combinator.
#[derive(Clone, Serialize, Deserialize)]
pub struct CssCombinatorSelector {
    pub location: Location,
    pub left: Rc<CssDirective>,
    pub right: Rc<CssDirective>,
    pub combinator_type: CssCombinatorType,
}

/// The root object of a CSS DOM. The CSS3 DOM objects serve not only IDE
/// features in code model, but also CSS compilation.
#[derive(Clone, Serialize, Deserialize)]
pub struct CssDocument {
    pub location: Location,
    /// List of rules, `@namespace` definitions, and `@font-face` definitions.
    pub children: Vec<Rc<CssDirective>>,
}

/// CSS DOM for an `@font-face` statement.
#[derive(Clone, Serialize, Deserialize)]
pub struct CssFontFace {
    pub location: Location,
    // List of `CssDirective::Property` variants.
    pub properties: Vec<Rc<CssDirective>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CssProperty {
    pub location: Location,
    pub name: (String, Location),
    pub value: Rc<CssPropertyValueNode>,
    _phantom: PhantomData<()>,
}

impl CssProperty {
    pub fn new(location: Location, name: (String, Location), value: Rc<CssPropertyValueNode>) -> Self {
        Self {
            location,
            name: (Self::normalize(&name.0), name.1),
            value,
            _phantom: PhantomData::default(),
        }
    }

    /// Normalize CSS property names to camel-case style names. Names already in
    /// camel-case will be returned as-is.
    fn normalize(name: &str) -> String {
        let mut split = name.split('-').map(|s| s.to_owned()).collect::<Vec<_>>();
        let mut v = split[0].chars();
        let mut v1 = String::new();
        if let Some(ch) = v.next() {
            v1.push_str(&ch.to_lowercase().to_string());
            for ch in v {
                v1.push(ch);
            }
        }
        split[0] = v1;
        for i in 1..split.len() {
            let mut v = split[i].chars();
            let mut v1 = String::new();
            if let Some(ch) = v.next() {
                v1.push_str(&ch.to_uppercase().to_string());
                for ch in v {
                    v1.push(ch);
                }
            }
            split[i] = v1;
        }
        split.join("")
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CssMediaQuery {
    pub location: Location,
    pub conditions: Vec<Rc<CssMediaQueryCondition>>,
    pub rules: Vec<Rc<CssRule>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum CssMediaQueryCondition {
    Invalidated(InvalidatedNode),
    Base((Rc<CssMediaQueryCondition>, Location)),
    And((Rc<CssMediaQueryCondition>, Location)),
    Not((Rc<CssMediaQueryCondition>, Location)),
    Only((Rc<CssMediaQueryCondition>, Location)),
}

impl CssMediaQueryCondition {
    pub fn location(&self) -> Location {
        match self {
            Self::Invalidated(v) => v.location.clone(),
            Self::Base((_, l)) => l.clone(),
            Self::And((_, l)) => l.clone(),
            Self::Not((_, l)) => l.clone(),
            Self::Only((_, l)) => l.clone(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CssRule {
    pub location: Location,
    pub selectors: Vec<Rc<CssSelector>>,
    pub properties: Vec<Rc<CssProperty>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CssNamespaceDefinition {
    pub location: Location,
    pub prefix: String,
    pub uri: String,
}