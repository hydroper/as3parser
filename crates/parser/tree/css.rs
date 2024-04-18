use std::str::FromStr;

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

impl FromStr for CssFontFaceSourceType {
    type Err = ParserError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "url" => Ok(Self::Url),
            "local" => Ok(Self::Local),
            _ => Err(ParserError::Common),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CssModelTreeType {
    Invalidated,
    PropertyValue,
    Combinator,
    Document,
    FontFace,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum CssNode {
    Invalidated(CssInvalidatedNode),
    ArrayPropertyValue(CssArrayPropertyValue),
    ColorPropertyValue(CssColorPropertyValue),
    Combinator(CssCombinator),
    Document(CssDocument),
    FontFace(CssFontFace),
}

impl CssNode {
    pub fn children(&self) -> Vec<Rc<CssNode>> {
        match self {
            Self::Invalidated(_) => vec![],
            Self::ArrayPropertyValue(v) => v.elements.clone(),
            Self::ColorPropertyValue(_) => vec![],
            Self::Combinator(_) => vec![],
            Self::Document(v) => {
                let mut list = vec![];
                list.extend(v.at_namespaces.iter().map(|v| v.clone()));
                list.extend(v.font_faces.iter().map(|v| v.clone()));
                list.extend(v.rules.iter().map(|v| v.clone()));
                list
            },
            Self::FontFace(v) => vec![],
        }
    }

    /// Source location information.
    pub fn location(&self) -> Location {
        match self {
            Self::Invalidated(v) => v.location.clone(),
            Self::ArrayPropertyValue(v) => v.location.clone(),
            Self::ColorPropertyValue(v) => v.location.clone(),
            Self::Combinator(v) => v.location.clone(),
            Self::Document(v) => v.location.clone(),
            Self::FontFace(v) => v.location.clone(),
        }
    }

    /// Node's type.
    pub fn operator(&self) -> CssModelTreeType {
        match self {
            Self::Invalidated(_) => CssModelTreeType::Invalidated,
            Self::ArrayPropertyValue(_) |
            Self::ColorPropertyValue(_) => CssModelTreeType::PropertyValue,
            Self::Combinator(_) => CssModelTreeType::Combinator,
            Self::Document(_) => CssModelTreeType::Document,
            Self::FontFace(_) => CssModelTreeType::FontFace,
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

    pub fn as_property(&self) -> Option<&CssProperty> {
        let CssNode::Property(property) = self else { None };
        Some(property)
    }

    pub fn as_array_property_value(&self) -> Option<&CssArrayPropertyValue> {
        let CssNode::ArrayPropertyValue(v) = self else { None };
        Some(v)
    }

    pub fn as_function_call_property_value(&self) -> Option<&CssFunctionCallPropertyValue> {
        let CssNode::FunctionCallPropertyValue(v) = self else { None };
        Some(v)
    }

    pub fn as_namespace_definition(&self) -> Option<&CssNamespaceDefinition> {
        let CssNode::NamespaceDefinition(d) = self else { None };
        Some(d)
    }
}

impl ToString for CssNode {
    fn to_string(&self) -> String {
        match self {
            Self::Invalidated(_) => "".into(),
            Self::ArrayPropertyValue(v) => v.elements.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", "),
            Self::ColorPropertyValue(v) => v.text(),
            Self::Combinator(v) => v.location.text(),
            Self::Document(v) => {
                let mut list = Vec::new();
                list.push(v.at_namespaces.iter().map(|v| v.to_string()).collect::<Vec<_>>().join("\n"));
                list.push(v.font_faces.iter().map(|v| v.to_string()).collect::<Vec<_>>().join("\n"));
                list.push(v.rules.iter().map(|v| v.to_string()).collect::<Vec<_>>().join("\n"));
                list.join("\n")
            },
            Self::FontFace(v) => {
                let mut s = String::new();
                s.push_str("@font-face {\n");
                s.push_str(&format!("    src : {};\n", v.source.to_string()));
                s.push_str(&format!("    fontFamily : {};\n", &v.font_family));
                s.push_str(&format!("    embedAsCFF : {};\n", v.embed_as_cff.to_string()));
                s.push_str(&format!("    advancedAntiAliasing : {};\n", v.advanced_anti_aliasing.to_string()));
                s.push_str(&format!("    fontStyle : {};\n", &v.font_style));
                s.push_str(&format!("    fontWeight : {};\n", &v.font_weight));
                s.push_str("}\n");
                s
            },
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CssInvalidatedNode {
    pub location: Location,
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
    pub location: Location,
    /// The selector (`CssNode::Selector`) associated with the combinator. For example:
    /// ```css
    /// s|VBox s|Label
    /// ```
    /// Then, `s|Label` is a combinator whose selector is `s|VBox`.
    pub selector: Rc<CssNode>,
    /// The combinator type.
    pub combinator_type: CssCombinatorType,
}

/// The root object of a CSS DOM. The CSS3 DOM objects serve not only IDE
/// features in code model, but also CSS compilation.
#[derive(Clone, Serialize, Deserialize)]
pub struct CssDocument {
    pub location: Location,
    /// List of `CssNode::Rule` nodes, CSS rules in their declaration order.
    pub rules: Vec<Rc<CssNode>>,
    /// List of `CssNode::NamespaceDefinition` nodes, namespace prefix declarations.
    pub at_namespaces: Vec<Rc<CssNode>>,
    /// List of `CssNode::FontFace` nodes, font face statements in their declaration order.
    pub font_faces: Vec<Rc<CssNode>>,

    namespaces_lookup: HashMap<String, Rc<CssNode>>,
}

impl CssDocument {
    /// The short name for the default namespace is an empty string.
    const DEFAULT_NAMESPACE_SHORT_NAME: &'static str = "";

    pub fn new(location: Location, at_namespaces: Vec<Rc<CssNode>>, font_faces: Vec<Rc<CssNode>>, rules: Vec<Rc<CssNode>>) -> Self {
        let mut namespaces_lookup = HashMap::<String, Rc<CssNode>>::new();
        for ns in at_namespaces.iter() {
            let ns1 = ns.as_namespace_definition().unwrap();
            let key = if let Some(p) = &ns1.prefix { p.clone() } else { Self::DEFAULT_NAMESPACE_SHORT_NAME.to_owned() };
            namespaces_lookup.insert(key, ns.clone());
        }
        Self {
            location,
            at_namespaces,
            font_faces,
            rules,
            namespaces_lookup,
        }
    }

    /// Gets the `CssNode::NamespaceDefinition` from its associated prefix name.
    pub fn get_namespace_definition(&self, prefix: &str) -> Option<Rc<CssNode>> {
        self.namespaces_lookup.get(prefix).map(|node| node.clone())
    }

    /// Gets the default `CssNode::NamespaceDefinition` for this document.
    /// The default namespace's short name is an empty string `""`.
    pub fn get_default_namespace_definition(&self) -> Option<Rc<CssNode>> {
        self.namespaces_lookup.get(Self::DEFAULT_NAMESPACE_SHORT_NAME).map(|node| node.clone())
    }
}

/// CSS DOM for an `@font-face` statement.
#[derive(Clone, Serialize, Deserialize)]
pub struct CssFontFace {
    pub location: Location,
    /// `CssNode::FunctionCallPropertyValue`
    source: Rc<CssNode>,
    /// List of CSS property values for multiple `src` properties.
    pub sources: Vec<Rc<CssNode>>,
    /// The `fontFamily` property sets the alias for the font that you use to apply
    /// the font in style sheets. This property is required. If you embed a font
    /// with a family name that matches the family name of a system font, the
    /// Flex compiler gives you a warning. You can disable this warning by
    /// setting the `show-shadows-system-font-warnings` compiler option to false.
    /// 
    /// Returns the font family name of this `font-face` statement. The font family
    /// name can be used in the later CSS rulesets' `font-family`
    /// properties.
    pub font_family: String,
    /// The "style" type face value of the font. Possible values are
    /// "normal", "italic" and "oblique". If the value is not set in the CSS
    /// document, this method returns the default font style "normal".
    pub font_style: String,
    /// Get the "weight" type face value of the font. Possible values are
    /// "normal", "bold" and "heavy". If the value is not set in the CSS
    /// document, this method returns the default font style "normal".
    pub font_weight: String,
    /// The `advancedAntiAliasing` property determines whether to include the
    /// advanced anti-aliasing information when embedding the font. This property
    /// is optional. The default value is true.
    ///
    /// @return True if this font face uses advanced anti-aliasing.
    pub advanced_anti_aliasing: bool,
    /// Value of the `embedAsCFF` property.
    pub embed_as_cff: bool,
}

impl CssFontFace {
    /// Construct a `CssNode::FontFace` from a list of properties. The parser
    /// doesn't validate if the properties are acceptable by the
    /// `@font-face` statement, so that we don't need to update the grammar
    /// when new properties are added to `@font-face` statement.
    pub fn new(location: Location, properties: Vec<Rc<CssNode>>) -> Self {
        let mut sources = Vec::<Rc<CssNode>>::new();
        let mut src_value: Option<Rc<CssNode>> = None;
        let mut font_family_value: Option<Rc<CssNode>> = None;
        let mut font_style_value: Option<Rc<CssNode>> = None;
        let mut font_weight_value: Option<Rc<CssNode>> = None;
        let mut embed_as_cff_value: Option<Rc<CssNode>> = None;
        let mut advanced_aa_value: Option<Rc<CssNode>> = None;

        for property in properties.iter() {
            let property = property.as_property().unwrap();
            let name: &str = &property.name;
            let value: &Rc<CssNode> = &property.value;

            match name {
                "src" => {
                    sources.push(value.clone());
                    src_value = Some(value.clone());
                },
                "fontFamily" => {
                    font_family_value = Some(value.clone());
                },
                "fontStyle" => {
                    font_style_value = Some(value.clone());
                },
                "fontWeight" => {
                    font_weight_value = Some(value.clone());
                },
                "embedAsCFF" => {
                    embed_as_cff_value = Some(value.clone());
                },
                "advancedAntiAliasing" => {
                    advanced_aa_value = Some(value.clone());
                },
                _ => {
                    // Ignore unknown properties.
                },
            }
        }

        let source: Rc<CssNode>;
        assert!(src_value.is_some(), "'src' is required in @font-face");
        let src_value = src_value.unwrap();
        if let Some(v) = src_value.as_array_property_value() {
            source = src_value.nth_child(0).unwrap();
        } else {
            source = src_value;
        }

        assert!(font_family_value.is_some(), "'fontFamily' is required in @font-face");

        Self {
            location,
            source,
            sources,
            font_family: font_family_value.unwrap().to_string(),
            font_style: if let Some(v) = font_style_value { v.to_string() } else { "normal".into() },
            font_weight: if let Some(v) = font_weight_value { v.to_string() } else { "normal".into() },
            embed_as_cff: if let Some(v) = embed_as_cff_value { v.to_string().to_lowercase() == "true" } else { true },
            advanced_anti_aliasing: if let Some(v) = advanced_aa_value { v.to_string().to_lowercase() == "true" } else { true },
        }
    }

    pub fn source_type(&self) -> Result<CssFontFaceSourceType, ParserError> {
        CssFontFaceSourceType::from_str(&self.source.as_function_call_property_value().unwrap().name)
    }

    pub fn source_value(&self) -> Result<String, ParserError> {
        let _ = self.source_type()?;
        if let Some(v) = CssFunctionCallPropertyValue::get_single_argument_from_raw(&self.source.as_function_call_property_value().unwrap().raw_arguments) {
            Ok(v)
        } else {
            Err(ParserError::Common)
        }
    }
}