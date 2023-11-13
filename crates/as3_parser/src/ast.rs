use std::rc::Rc;
use bitflags::bitflags;
use serde::{Serialize, Deserialize};
use crate::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct QualifiedIdentifier {
    pub attribute: bool,
    pub qualifier: Option<Rc<Expression>>,
    pub name: IdentifierOrBrackets,
}

impl QualifiedIdentifier {
    /// Converts the qualified identifier to an Identifier token.
    pub fn to_identifier(&self) -> Option<(String, Location)> {
        if self.attribute || self.qualifier.is_some() {
            return None;
        }
        if let IdentifierOrBrackets::Id(id, location) = self.name {
            if id != "*" { Some((id, location.clone())) } else { None }
        } else {
            None
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NonAttributeQualifiedIdentifier {
    pub qualifier: Option<Rc<Expression>>,
    pub name: IdentifierOrBrackets,
}



impl NonAttributeQualifiedIdentifier {
    /// Converts the qualified identifier to an Identifier token.
    pub fn to_identifier(&self) -> Option<(String, Location)> {
        if self.qualifier.is_some() {
            return None;
        }
        if let IdentifierOrBrackets::Id(id, location) = self.name {
            if id != "*" { Some((id, location.clone())) } else { None }
        } else {
            None
        }
    }

    /// Converts the qualified identifier to an Identifier token or a wildcard (`*`) token.
    pub fn to_identifier_or_wildcard(&self) -> Option<(String, Location)> {
        if self.qualifier.is_some() {
            return None;
        }
        if let IdentifierOrBrackets::Id(id, location) = self.name {
            Some((id, location.clone()))
        } else {
            None
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum IdentifierOrBrackets {
    Id(String, Location),
    Brackets(Rc<Expression>),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Expression {
    pub location: Location,
    pub kind: ExpressionKind,
}

impl Expression {
    pub(crate) fn list_metadata_expressions(self: &Rc<Self>) -> Option<Vec<Rc<Self>>> {
        match self.kind {
            ExpressionKind::ArrayInitializer { .. } => Some(vec![Rc::clone(self)]),
            ExpressionKind::BracketsMember { base, key, .. } => {
                let mut result = base.list_metadata_expressions()?;
                result.push(Rc::clone(&key));
                Some(result)
            },
            _ => None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ExpressionKind {
    Null,
    Boolean(bool),
    Numeric(f64),
    String(String),
    This,
    RegExp {
        body: String,
        flags: String,
    },
    Id(QualifiedIdentifier),
    XmlMarkup(String),
    XmlElement(XmlElement),
    XmlList(Vec<XmlElementContent>),
    ReservedNamespace(ReservedNamespace),
    /// `()`. Used solely internally for arrow functions.
    EmptyParen,
    Paren(Rc<Expression>),
    /// Present as part of an array initializer only.
    /// This expression is not valid in other contexts.
    Rest(Rc<Expression>),
    ArrayInitializer {
        metadata_asdoc: Option<AsDoc>,
        /// Element sequence possibly containing `Rest`s and ellisions.
        elements: Vec<Option<Rc<Expression>>>,
    },
    /// `new <T> []`
    VectorInitializer {
        element_type: Rc<TypeExpression>,
        /// Element sequence possibly containing `Rest`s.
        elements: Vec<Rc<Expression>>,
    },
    ObjectInitializer {
        fields: Vec<Rc<ObjectField>>,
    },
    Function {
        name: Option<(String, Location)>,
        common: Rc<FunctionCommon>,
    },
    ArrowFunction(Rc<FunctionCommon>),
    Super(Option<Vec<Rc<Expression>>>),
    New {
        base: Rc<Expression>,
        arguments: Option<Vec<Rc<Expression>>>,
    },
    /// The `o.x` expression.
    DotMember {
        base: Rc<Expression>,
        id: QualifiedIdentifier,
    },
    /// The `o[k]` expression.
    BracketsMember {
        base: Rc<Expression>,
        metadata_asdoc: Option<AsDoc>,
        key: Rc<Expression>,
    },
    /// `base.<T1, Tn>`
    WithTypeArguments {
        base: Rc<Expression>,
        arguments: Vec<Rc<TypeExpression>>,
    },
    /// The `o.(condition)` expression.
    Filter {
        base: Rc<Expression>,
        condition: Rc<Expression>,
    },
    /// The `o..x` expression.
    Descendants {
        base: Rc<Expression>,
        id: QualifiedIdentifier,
    },
    Call {
        base: Rc<Expression>,
        arguments: Vec<Rc<Expression>>,
    },
    Unary {
        base: Rc<Expression>,
        operator: Operator,
    },
    Binary {
        left: Rc<Expression>,
        operator: Operator,
        right: Rc<Expression>,
    },
    Conditional {
        test: Rc<Expression>,
        consequent: Rc<Expression>,
        alternative: Rc<Expression>,
    },
    Assignment {
        left: AssignmentLeft,
        compound: Option<Operator>,
        right: Rc<Expression>,
    },
    /// The `x, y` expression.
    Sequence(Rc<Expression>, Rc<Expression>),

    /// Expression used internally only. It is used for parsing
    /// arrow functions with typed parameters and return annotation.
    WithTypeAnnotation {
        base: Rc<Expression>,
        type_annotation: Rc<TypeExpression>,
    },

    Embed {
        source: String,
        type_annotation: Option<Rc<TypeExpression>>,
    },

    /// Expression containing an optional chaining operator.
    OptionalChaining {
        base: Rc<Expression>,
        /// Postfix operators that execute if the base is not `null`
        /// and not `undefined`. The topmost node in this field is
        /// [`ExpressionKind::OptionalChainingHost`], which holds
        /// a non-null and not-undefined value.
        operations: Rc<Expression>,
    },

    /// The topmost expression from which postfix operators
    /// follow in an [`ExpressionKind::OptionalChaining`] expression
    /// inside the `operations` field.
    OptionalChainingHost,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum AssignmentLeft {
    Expression(Rc<Expression>),
    Destructuring(Rc<Destructuring>),
}

impl AssignmentLeft {
    pub fn location(&self) -> Location {
        match self {
            Self::Expression(exp) => exp.location.clone(),
            Self::Destructuring(destr) => destr.location.clone(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum XmlElementContent {
    Expression(Rc<Expression>),
    Markup(String, Location),
    Text(String, Location),
    Element(XmlElement),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct XmlElement {
    pub location: Location,
    pub opening_tag_name: XmlTagName,
    pub attributes: Vec<XmlAttributeOrExpression>,
    pub content: Vec<XmlElementContent>,
    pub closing_tag_name: Option<XmlTagName>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum XmlTagName {
    Name((String, Location)),
    Expression(Rc<Expression>),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum XmlAttributeOrExpression {
    Attribute(XmlAttribute),
    Expression(Rc<Expression>),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct XmlAttribute {
    pub name: (String, Location),
    pub value: XmlAttributeValueOrExpression,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum XmlAttributeValueOrExpression {
    Value(String),
    Expression(Rc<Expression>),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ReservedNamespace {
    Public,
    Private,
    Protected,
    Internal,
}

impl ToString for ReservedNamespace {
    fn to_string(&self) -> String {
        match self {
            Self::Public => "public".into(),
            Self::Private => "private".into(),
            Self::Protected => "protected".into(),
            Self::Internal => "internal".into(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ObjectField {
    Field {
        key: (ObjectKey, Location),
        /// Used when parsing an object initializer as a destructuring pattern.
        /// This is the result of consuming the `!` punctuator.
        #[doc(hidden)]
        destructuring_non_null: bool,
        /// If `None`, this is a shorthand field.
        value: Option<Rc<Expression>>,
    },
    Rest(Rc<Expression>, Location),
}

impl ObjectField {
    pub fn location(&self) -> Location {
        match self {
            Self::Field { key, value, .. } => {
                if let Some(value) = value {
                    key.1.combine_with(value.location.clone())
                } else {
                    key.1.clone()
                }
            },
            Self::Rest(_, location) => location.clone(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ObjectKey {
    Id(NonAttributeQualifiedIdentifier),
    String(String, Location),
    Number(f64, Location),
    Brackets(Rc<Expression>),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Destructuring {
    pub location: Location,
    pub kind: DestructuringKind,
    /// Indicates whether the pattern asserts that the
    /// destructuring base is not any of `undefined` and `null`.
    /// The patterns use the `!` punctuator to indicate this behavior.
    pub non_null: bool,
    pub type_annotation: Option<Rc<TypeExpression>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum DestructuringKind {
    Binding {
        name: (String, Location),
    },
    Record(Vec<Rc<RecordDestructuringField>>),
    Array(Vec<Option<ArrayDestructuringItem>>),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RecordDestructuringField {
    pub location: Location,
    pub key: (ObjectKey, Location),
    pub non_null: bool,
    pub alias: Option<Rc<Destructuring>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ArrayDestructuringItem {
    Pattern(Rc<Destructuring>),
    Rest(Rc<Destructuring>, Location),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TypeExpression {
    pub location: Location,
    pub kind: TypeExpressionKind,
}

impl TypeExpression {
    pub(crate) fn to_function_type_param(&self) -> Option<FunctionTypeParam> {
        match self.kind {
            TypeExpressionKind::Id(id) => {
                if let Some(name) = id.to_identifier() {
                    Some(FunctionTypeParam {
                        kind: FunctionParamKind::Required,
                        name,
                        type_annotation: None,
                    })
                } else {
                    None
                }
            },
            TypeExpressionKind::Nullable(subexp) => {
                match subexp.kind {
                    TypeExpressionKind::Id(id) => {
                        if let Some(name) = id.to_identifier() {
                            Some(FunctionTypeParam {
                                kind: FunctionParamKind::Optional,
                                name,
                                type_annotation: None,
                            })
                        } else {
                            None
                        }
                    },
                    _ => None,
                }
            },
            _ => None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum TypeExpressionKind {
    Id(NonAttributeQualifiedIdentifier),
    DotMember {
        base: Rc<TypeExpression>,
        id: NonAttributeQualifiedIdentifier,
    },
    Tuple(Vec<Rc<TypeExpression>>),
    Record(Vec<Rc<RecordTypeField>>),
    /// `(x)`
    Paren(Rc<TypeExpression>),
    /// `*`
    Any,
    Void,
    Never,
    Undefined,
    Nullable(Rc<TypeExpression>),
    NonNullable(Rc<TypeExpression>),
    Function {
        params: Vec<FunctionTypeParam>,
        return_annotation: Rc<TypeExpression>,
    },
    StringLiteral(String),
    NumericLiteral(f64),
    /// `|`
    Union(Vec<Rc<TypeExpression>>),
    /// `&`
    Complement(Rc<TypeExpression>, Rc<TypeExpression>),
    /// `base.<T1, Tn>`
    WithTypeArguments {
        base: Rc<TypeExpression>,
        arguments: Vec<Rc<TypeExpression>>,
    },
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FunctionTypeParam {
    pub kind: FunctionParamKind,
    pub name: (String, Location),
    pub type_annotation: Option<Rc<TypeExpression>>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[repr(u32)]
pub enum FunctionParamKind {
    Required = 1,
    Optional = 2,
    Rest = 3,
}

impl FunctionParamKind {
    pub fn may_be_followed_by(&self, other: Self) -> bool {
        (*self as u32) <= (other as u32)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RecordTypeField {
    pub asdoc: Option<AsDoc>,
    pub readonly: bool,
    pub key: (ObjectKey, Location),
    pub nullability: FieldNullability,
    pub type_annotation: Rc<TypeExpression>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum FieldNullability {
    Unspecified,
    NonNullable,
    Nullable,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Statement {
    pub location: Location,
    pub kind: StatementKind,
}

impl Statement {
    pub(crate) fn to_identifier(&self) -> Option<(String, Location)> {
        if let StatementKind::Expression { expression, .. } = &self.kind {
            if let ExpressionKind::Id(id) = &expression.kind {
                id.to_identifier()
            } else {
                None
            }
        } else {
            None
        }
    }

    pub(crate) fn list_metadata_expressions(&self) -> Option<Vec<Rc<Expression>>> {
        if let StatementKind::Expression { expression, .. } = &self.kind {
            expression.list_metadata_expressions()
        } else {
            None
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum StatementKind {
    Empty,
    Super(Vec<Rc<Expression>>),
    Block(Rc<Block>),
    If {
        condition: Rc<Expression>,
        consequent: Rc<Statement>,
        alternative: Option<Rc<Statement>>,
    },
    Switch {
        discriminant: Rc<Expression>,
        cases: Vec<SwitchCase>,
    },
    SwitchType {
        discriminant: Rc<Expression>,
        cases: Vec<SwitchTypeCase>,
    },
    Do {
        body: Rc<Statement>,
        test: Rc<Expression>,
    },
    While {
        test: Rc<Expression>,
        body: Rc<Statement>,
    },
    For {
        init: Option<ForInit>,
        test: Option<Rc<Expression>>,
        update: Option<Rc<Expression>>,
        body: Rc<Statement>,
    },
    ForIn {
        each: bool,
        left: ForInLeft,
        right: Rc<Expression>,
        body: Rc<Statement>,
    },
    With {
        object: Rc<Expression>,
        body: Rc<Statement>,
    },
    Continue {
        label: Option<String>,
    },
    Break {
        label: Option<String>,
    },
    Return {
        expression: Option<Rc<Expression>>,
    },
    Throw {
        expression: Rc<Expression>,
    },
    Try {
        block: Rc<Block>,
        catch_clauses: Vec<CatchClause>,
        finally_clause: Option<FinallyClause>,
    },
    Expression {
        asdoc: Option<AsDoc>,
        expression: Rc<Expression>,
    },
    Labeled {
        label: (String, Location),
        statement: Rc<Statement>,
    },
    DefaultXmlNamespace(Rc<Expression>),
    SimpleVariableDeclaration(SimpleVariableDeclaration),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CatchClause {
    pub pattern: Rc<Destructuring>,
    pub block: Rc<Block>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FinallyClause(pub Rc<Block>);

#[derive(Clone, Serialize, Deserialize)]
pub enum ForInit {
    Variable(SimpleVariableDeclaration),
    Expression(Rc<Expression>),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ForInLeft {
    Variable(VariableKind, VariableBinding),
    Expression(Rc<Expression>),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SimpleVariableDeclaration {
    pub kind: (VariableKind, Location),
    pub bindings: Vec<VariableBinding>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VariableBinding {
    pub pattern: Rc<Destructuring>,
    pub init: Option<Rc<Expression>>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum VariableKind {
    Var,
    Const,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SwitchCase {
    pub expression: Option<Rc<Expression>>,
    pub consequent: Vec<Rc<Directive>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SwitchTypeCase {
    pub pattern: Option<Rc<Destructuring>>,
    pub block: Rc<Block>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Block(pub Vec<Rc<Directive>>);

#[derive(Clone, Serialize, Deserialize)]
pub struct Directive {
    pub location: Location,
    pub kind: DirectiveKind,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum DirectiveKind {
    Statement(Rc<Statement>),
    Include(Rc<IncludeDirective>),
    Import(Rc<ImportDirective>),
    UseNamespace(Rc<Expression>),
    VariableDefinition(Rc<VariableDefinition>),
    FunctionDefinition(Rc<FunctionDefinition>),
    ConstructorDefinition(Rc<ConstructorDefinition>),
    GetterDefinition(Rc<GetterDefinition>),
    SetterDefinition(Rc<SetterDefinition>),
    TypeDefinition(Rc<TypeDefinition>),
    ClassDefinition(Rc<ClassDefinition>),
    EnumDefinition(Rc<EnumDefinition>),
    InterfaceDefinition(Rc<InterfaceDefinition>),
    NamespaceDefinition(Rc<NamespaceDefinition>),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ClassDefinition {
    pub asdoc: Option<AsDoc>,
    pub annotations: DefinitionAnnotations,
    pub name: (String, Location),
    pub generics: Generics,
    pub extends_clause: Option<Rc<TypeExpression>>,
    pub implements_clause: Option<Vec<Rc<TypeExpression>>>,
    pub block: Rc<Block>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct InterfaceDefinition {
    pub asdoc: Option<AsDoc>,
    pub annotations: DefinitionAnnotations,
    pub name: (String, Location),
    pub generics: Generics,
    pub extends_clause: Option<Vec<Rc<TypeExpression>>>,
    pub block: Rc<Block>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EnumDefinition {
    pub asdoc: Option<AsDoc>,
    pub annotations: DefinitionAnnotations,
    pub name: (String, Location),
    pub block: Rc<Block>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NamespaceDefinition {
    pub asdoc: Option<AsDoc>,
    pub annotations: DefinitionAnnotations,
    pub left: (String, Location),
    pub right: Option<Rc<Expression>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct IncludeDirective {
    pub source: String,
    pub replaced_by: Vec<Rc<Directive>>,
    #[serde(skip)]
    pub replaced_by_source: Rc<Source>,
}

/// An import directive.
/// 
/// If it is an alias with a wildcard import item,
/// it is a package alias that opens the public namespace
/// and aliases it.
/// 
/// If it is an alias with a package recursive import item,
/// it is a package set alias that opens the public namespace of
/// all the respective packages and aliases them into a namespace set.
#[derive(Clone, Serialize, Deserialize)]
pub struct ImportDirective {
    pub alias: Option<(String, Location)>,
    pub package_name: Vec<(String, Location)>,
    pub import_item: (ImportItem, Location),
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ImportItem {
    Wildcard,
    /// `**`
    Recursive,
    Name(String, Location),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct VariableDefinition {
    pub asdoc: Option<AsDoc>,
    pub annotations: DefinitionAnnotations,
    pub kind: VariableKind,
    pub bindings: Vec<VariableBinding>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub asdoc: Option<AsDoc>,
    pub annotations: DefinitionAnnotations,
    pub name: (String, Location),
    pub generics: Generics,
    pub common: Rc<FunctionCommon>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ConstructorDefinition {
    pub asdoc: Option<AsDoc>,
    pub annotations: DefinitionAnnotations,
    pub name: (String, Location),
    pub common: Rc<FunctionCommon>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GetterDefinition {
    pub asdoc: Option<AsDoc>,
    pub annotations: DefinitionAnnotations,
    pub name: (String, Location),
    pub common: Rc<FunctionCommon>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SetterDefinition {
    pub asdoc: Option<AsDoc>,
    pub annotations: DefinitionAnnotations,
    pub name: (String, Location),
    pub common: Rc<FunctionCommon>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TypeDefinition {
    pub asdoc: Option<AsDoc>,
    pub annotations: DefinitionAnnotations,
    pub left: (String, Location),
    pub generics: Generics,
    pub right: Rc<TypeExpression>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DefinitionAnnotations {
    pub metadata: Vec<Rc<Metadata>>,
    #[serde(skip)]
    pub flag_modifiers: DefinitionModifiersFlags,
    pub access_modifier: Option<Rc<Expression>>,
}

bitflags! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct DefinitionModifiersFlags: u32 {
        const OVERRIDE  = 0b00000001;
        const FINAL     = 0b00000010;
        const DYNAMIC   = 0b00000100;
        const NATIVE    = 0b00001000;
        const STATIC    = 0b00010000;
    }
}

impl Default for DefinitionModifiersFlags {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub asdoc: Option<AsDoc>,
    pub location: Location,
    /// The metadata name. The metadata name may contain a single `::` delimiter.
    pub name: (String, Location),
    pub entries: Vec<MetadataEntry>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MetadataEntry {
    pub key: Option<(String, Location)>,
    pub value: (String, Location),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Generics {
    pub params: Option<Vec<Rc<GenericParam>>>,
    pub where_clause: Option<GenericsWhere>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GenericParam {
    pub location: Location,
    pub name: (String, Location),
    pub constraints: Vec<Rc<TypeExpression>>,
    pub default_type: Option<Rc<TypeExpression>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GenericsWhere {
    pub constraints: Vec<GenericsWhereConstraint>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GenericsWhereConstraint {
    pub name: (String, Location),
    pub constraints: Vec<Rc<TypeExpression>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FunctionCommon {
    #[serde(skip)]
    pub flags: FunctionFlags,
    pub params: Vec<FunctionParam>,
    pub return_annotation: Option<Rc<TypeExpression>>,
    pub body: Option<FunctionBody>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FunctionParam {
    pub location: Location,
    pub kind: FunctionParamKind,
    pub binding: VariableBinding,
}

bitflags! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub struct FunctionFlags: u32 {
        const AWAIT     = 0b00000001;
        const YIELD     = 0b00000010;
    }
}

impl Default for FunctionFlags {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum FunctionBody {
    Block(Rc<Block>),
    /// The function body is allowed to be an expression
    /// in arrow functions.
    Expression(Rc<Expression>),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AsDoc {
    pub main_body: String,
    pub tags: Vec<AsDocTag>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum AsDocTag {
    Copy(String),
    Default(String),
    EventType(Rc<TypeExpression>),
    Example(String),
    ExampleText(String),
    InheritDoc,
    Internal(String),
    Param {
        name: String,
        description: String,
    },
    Private,
    Return(String),
    See {
        reference: String,
        display_text: Option<String>,
    },
    Throws {
        class_name: Rc<TypeExpression>,
        description: Option<String>,
    },
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PackageDefinition {
    pub asdoc: Option<AsDoc>,
    pub location: Location,
    pub id: Vec<(String, Location)>,
    pub block: Rc<Block>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Program {
    pub packages: Vec<Rc<PackageDefinition>>,
    pub directives: Vec<Rc<Directive>>,
}