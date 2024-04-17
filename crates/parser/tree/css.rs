use crate::ns::*;
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

/// Supported condition types for [`CssSelectorCondition`].
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CssConditionType {
    /// For example: `s|Label.className`
    Class,
    /// For example: `s|Label#idValue`
    Id,
    /// For example: `s|Label:loadingState`
    Pseudo,
    /// For example: `s|Label::loadingState`
    PseudoElement,
    /// For example: `s|Panel:not(:first-child)`
    Not,
    /// For example: `s|Label[loadingState]`
    Attribute,
}

impl CssConditionType {
    /// Prefix characters of the condition type.
    pub fn prefix(&self) -> String {
        match self {
            Self::Class => ".".into(),
            Self::Id => "#".into(),
            Self::Pseudo => ":".into(),
            Self::PseudoElement => "::".into(),
            Self::Not => "not".into(),
            Self::Attribute => "[".into(),
        }
    }
}

/// Source type enumerations for [`CssFontFace`].
#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CssFontFaceSourceType {
    /// The source value is a URL of the font filename. For example:
    /// ```css
    /// src: url("../assets/MyriadWebPro.ttf");
    /// ```
    Url,
    /// The source value is the system font name. For example:
    /// ```css
    /// src: local("Myriad Web Pro");
    /// ```
    Local,
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CssModelTreeType {
    PropertyValue,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum CssNode {
    ArrayPropertyValue(CssArrayPropertyValue),
    ColorPropertyValue(CssColorPropertyValue),
}

impl CssNode {
    pub fn children(&self) -> Vec<Rc<CssNode>> {
        match self {
            Self::ArrayPropertyValue(v) => v.elements.clone(),
            Self::ColorPropertyValue(_) => vec![],
        }
    }

    /// Source location information.
    pub fn location(&self) -> Location {
        match self {
            Self::ArrayPropertyValue(v) => v.location.clone(),
            Self::ColorPropertyValue(v) => v.location.clone(),
        }
    }

    /// Node's type.
    pub fn operator(&self) -> CssModelTreeType {
        match self {
            Self::ArrayPropertyValue(_) |
            Self::ColorPropertyValue(_) => CssModelTreeType::PropertyValue,
        }
    }

    /// Node's child count.
    pub fn arity(&self) -> usize {
        self.children().len()
    }

    /// Node's nth child given an index.
    pub fn nth_child(&self, index: usize) -> Option<Rc<CssNode>> {
        self.children().get(index).map(|r| r.clone())
    }
}

impl ToString for CssNode {
    fn to_string(&self) -> String {
        match self {
            Self::ArrayPropertyValue(v) => v.elements.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "),
            Self::ColorPropertyValue(v) => v.text(),
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
    pub elements: Vec<Rc<CssNode>>,
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

/// A "combinator" represents a CSS selector that combines with a selector. It
/// has a type value and an associated selector. If selector "A" is written on
/// the left of selector "B", then "A" is the combinator of "B".
///
/// For example, in the following CSS rule:
/// ```css
/// s|HBox s|Button.rounded s|Label {...}
/// ```
/// `s|Label` has an [`CssCombinator`] whose combinator type is
/// "descendant" (space character) and the combined selector is
/// `s|Button.rounded`.
#[derive(Clone, Serialize, Deserialize)]
pub struct CssCombinator {
    /// The selector (`CssSelector`) associated with the combinator. For example:
    /// ```css
    /// s|VBox s|Label
    /// ```
    /// Then, `s|Label` is a combinator whose selector is `s|VBox`.
    pub selector: Rc<CssNode>,
    /// The combinator type.
    pub combinator_type: CssCombinatorType,
}

/// The root object of a CSS DOM. The CSS3 DOM objects serves not only IDE
/// features in code model, but also CSS compilation.
#[derive(Clone, Serialize, Deserialize)]
pub struct CssDocument {
    pub location: Location,
    pub children: Vec<Rc<CssNode>>,
}