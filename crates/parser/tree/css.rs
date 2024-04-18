use std::{marker::PhantomData, str::FromStr};

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
    FontFaceList,
    MediaQuery,
    NamespaceList,
    PropertyList,
    RuleList,
    SelectorGroup,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum CssNode {
    Invalidated(CssInvalidatedNode),
    ArrayPropertyValue(CssArrayPropertyValue),
    ColorPropertyValue(CssColorPropertyValue),
    Combinator(CssCombinator),
    Document(CssDocument),
    FontFace(CssFontFace),
    FontFaceList(CssFontFaceList),
    FunctionCallPropertyValue(CssFunctionCallPropertyValue),
    MediaQuery(CssMediaQuery),
    NamespaceList(CssNamespaceList),
    PropertyList(CssPropertyList),
    RuleList(CssRuleList),
    SelectorGroup(CssSelectorGroup),
}

impl CssNode {
    pub fn children(&self) -> Vec<Rc<CssNode>> {
        match self {
            Self::Invalidated(_) => vec![],
            Self::ArrayPropertyValue(v) => v.elements.clone(),
            Self::ColorPropertyValue(_) => vec![],
            Self::Combinator(_) => vec![],
            Self::Document(v) => v.children.clone(),
            Self::FontFace(_) => vec![],
            Self::FontFaceList(v) => v.children.clone(),
            Self::FunctionCallPropertyValue(_) => vec![],
            Self::MediaQuery(v) => v.children.clone(),
            Self::NamespaceList(v) => v.children.clone(),
            Self::PropertyList(v) => v.children.clone(),
            Self::RuleList(v) => v.children.clone(),
            Self::SelectorGroup(v) => v.children.clone(),
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
            Self::FontFaceList(v) => v.location.clone(),
            Self::FunctionCallPropertyValue(v) => v.location.clone(),
            Self::MediaQuery(v) => v.location.clone(),
            Self::NamespaceList(v) => v.location.clone(),
            Self::PropertyList(v) => v.location.clone(),
            Self::RuleList(v) => v.location.clone(),
            Self::SelectorGroup(v) => v.location.clone(),
        }
    }

    /// Node's type.
    pub fn operator(&self) -> CssModelTreeType {
        match self {
            Self::Invalidated(_) => CssModelTreeType::Invalidated,
            Self::ArrayPropertyValue(_) |
            Self::ColorPropertyValue(_) |
            Self::FunctionCallPropertyValue(_) => CssModelTreeType::PropertyValue,
            Self::Combinator(_) => CssModelTreeType::Combinator,
            Self::Document(_) => CssModelTreeType::Document,
            Self::FontFace(_) => CssModelTreeType::FontFace,
            Self::FontFaceList(_) => CssModelTreeType::FontFaceList,
            Self::MediaQuery(_) => CssModelTreeType::MediaQuery,
            Self::NamespaceList(_) => CssModelTreeType::NamespaceList,
            Self::PropertyList(_) => CssModelTreeType::PropertyList,
            Self::RuleList(_) => CssModelTreeType::RuleList,
            Self::SelectorGroup(_) => CssModelTreeType::SelectorGroup,
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
        let CssNode::Property(property) = self else { return None; };
        Some(property)
    }

    pub fn as_array_property_value(&self) -> Option<&CssArrayPropertyValue> {
        let CssNode::ArrayPropertyValue(v) = self else { return None; };
        Some(v)
    }

    pub fn as_function_call_property_value(&self) -> Option<&CssFunctionCallPropertyValue> {
        let CssNode::FunctionCallPropertyValue(v) = self else { return None; };
        Some(v)
    }

    pub fn as_namespace_definition(&self) -> Option<&CssNamespaceDefinition> {
        let CssNode::NamespaceDefinition(d) = self else { return None; };
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
            Self::FunctionCallPropertyValue(v) => {
                if let Some(f) = &v.url_format {
                    format!("{}({}) {}", v.name, v.raw_arguments, f)
                } else {
                    format!("{}({})", v.name, v.raw_arguments)
                }
            },
            _ => "[object CSSNode]".into(),
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
    pub children: Vec<Rc<CssNode>>,
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
        let mut children: Vec<Rc<CssNode>> = vec![];
        let cursor_loc = Location::with_offset(&location.compilation_unit(), location.first_offset());
        let ns_children = at_namespaces.iter().map(|v| v.clone()).collect::<Vec<_>>();
        let fon_children = font_faces.iter().map(|v| v.clone()).collect::<Vec<_>>();
        let rules_children = rules.iter().map(|v| v.clone()).collect::<Vec<_>>();

        children.push(Rc::new(CssNode::NamespaceList(CssNamespaceList {
            location: if ns_children.is_empty() { cursor_loc.clone() } else {
                Location::with_offsets(&ns_children[0].location().compilation_unit(), ns_children[0].location().first_offset(), ns_children.last().unwrap().location().last_offset())
            },
            children: ns_children,
        })));

        children.push(Rc::new(CssNode::FontFaceList(CssFontFaceList {
            location: if fon_children.is_empty() { cursor_loc.clone() } else {
                Location::with_offsets(&fon_children[0].location().compilation_unit(), fon_children[0].location().first_offset(), fon_children.last().unwrap().location().last_offset())
            },
            children: fon_children,
        })));

        children.push(Rc::new(CssNode::RuleList(CssRuleList {
            location: if rules_children.is_empty() { cursor_loc.clone() } else {
                Location::with_offsets(&rules_children[0].location().compilation_unit(), rules_children[0].location().first_offset(), rules_children.last().unwrap().location().last_offset())
            },
            children: rules_children,
        })));

        Self {
            location,
            children,
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
        Ok(CssFunctionCallPropertyValue::get_single_argument_from_raw(&self.source.as_function_call_property_value().unwrap().raw_arguments))
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CssFontFaceList {
    pub location: Location,
    /// List of `CssNode::FontFace`.
    pub children: Vec<Rc<CssNode>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CssMediaQuery {
    pub location: Location,
    /// List of `CssNode::MediaQueryCondition`.
    pub children: Vec<Rc<CssNode>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CssNamespaceList {
    pub location: Location,
    /// List of `CssNode::NamespaceDefinition`.
    pub children: Vec<Rc<CssNode>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CssPropertyList {
    pub location: Location,
    /// List of `CssNode::Property`.
    pub children: Vec<Rc<CssNode>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CssRuleList {
    pub location: Location,
    /// List of `CssNode::Rule`.
    pub children: Vec<Rc<CssNode>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CssSelectorGroup {
    pub location: Location,
    /// List of `CssNode::Selector`.
    pub children: Vec<Rc<CssNode>>,
}

/// CSS function call property value.
///
/// For example:
///
/// ```css
/// Embed("bg.png")
/// ```
#[derive(Clone, Serialize, Deserialize)]
pub struct CssFunctionCallPropertyValue {
    pub location: Location,
    /// Name of the function.
    pub name: String,
    /// Raw arguments text excluding the parentheses.
    pub raw_arguments: String,
    /// If the function call is in the `url("") format("")` form,
    /// indicates the `format` function call characters.
    pub url_format: Option<String>,

    _nothing: PhantomData<()>,
}

impl CssFunctionCallPropertyValue {
    /// Function name for `ClassReference("")`.
    pub const CLASS_REFERENCE: &'static str = "ClassReference";
    /// Function name for `PropertyReference("")`.
    pub const PROPERTY_REFERENCE: &'static str = "PropertyReference";
    /// Function name for `Embed("")`.
    pub const EMBED: &'static str = "Embed";
    /// Function name for `url("")`.
    pub const URL: &'static str = "url";

    /// Constructs the node.
    ///
    /// # Parameters
    /// 
    /// - `raw_arguments`: raw arguments text including parentheses and any quotes.
    pub fn new(location: Location, name: String, raw_arguments: &str, url_format: Option<&str>) -> Self {
        Self {
            location,
            name,
            raw_arguments: raw_arguments[1..raw_arguments.len() - 1].to_owned(),
            url_format: url_format.map(|f| f.to_owned()),
            _nothing: PhantomData::default(),
        }
    }

    pub fn get_single_argument_from_raw(raw_arguments: &str) -> String {
        if (raw_arguments.starts_with('"') && raw_arguments.ends_with('"'))
        || (raw_arguments.starts_with('\'') && raw_arguments.ends_with('\'')) {
            raw_arguments[1..raw_arguments.len() - 1].to_owned()
        } else{
            raw_arguments.to_owned()
        }
    }
}