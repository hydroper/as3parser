use std::rc::Rc;
use bitflags::bitflags;
use crate::*;

pub struct QualifiedIdentifier {
    pub attribute: bool,
    pub qualifier: Option<Rc<Expression>>,
    pub name: IdentifierOrBrackets,
}

pub struct NonAttributeQualifiedIdentifier {
    pub qualifier: Option<Rc<Expression>>,
    pub name: IdentifierOrBrackets,
}

pub enum IdentifierOrBrackets {
    Id(String, Location),
    Brackets(Rc<Expression>),
}

pub struct Expression {
    pub location: Location,
    pub kind: ExpressionKind,
}

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
    Paren(Rc<Expression>),
    /// Present as part of an array initializer only.
    /// This expression is not valid in other contexts.
    Rest(Rc<Expression>),
    ArrayInitializer {
        /// Element sequence possibly containing `Rest`s and ellisions.
        elements: Vec<Option<Rc<Expression>>>,
        type_annotation: Option<Rc<TypeExpression>>,
    },
    /// `new <T> []`
    VectorInitializer {
        /// Element sequence possibly containing `Rest`s.
        elements: Vec<Rc<Expression>>,
    },
    ObjectInitializer {
        fields: Vec<ObjectInitializerItem>,
        type_annotation: Option<Rc<TypeExpression>>,
    },
    Function {
        name: Option<(String, Location)>,
        common: Rc<FunctionCommon>,
    },
    ArrowFunction(Rc<FunctionCommon>),
    Super(Option<Vec<Rc<Expression>>>),
    New {
        base: Rc<Expression>,
        arguments: Option<Vec<Expression>>,
    },
    /// The `o.x` expression.
    DotMember {
        base: Rc<Expression>,
        id: QualifiedIdentifier,
    },
    /// The `o[k]` expression.
    BracketsMember {
        base: Rc<Expression>,
        key: Rc<Expression>,
    },
    /// `base.<T1, Tn>`
    WithTypeArguments {
        base: Rc<Expression>,
        arguments: Vec<Rc<Expression>>,
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
        left: Rc<Destructuring>,
        compound: Option<Operator>,
        right: Rc<Expression>,
    },
    /// The `x, y` expression.
    Sequence(Vec<Rc<Expression>>, Vec<Rc<Expression>>),

    /// Expression used internally only.
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

pub enum XmlElementContent {
    Expression(Rc<Expression>),
    Markup(String, Location),
    Text(String, Location),
    Element(XmlElement),
}

pub struct XmlElement {
    pub location: Location,
    pub opening_tag_name: XmlTagName,
    pub attributes: Vec<XmlAttributeOrExpression>,
    pub content: Vec<XmlElementContent>,
    pub closing_tag_name: Option<XmlTagName>,
}

pub enum XmlTagName {
    Name((String, Location)),
    Expression(Rc<Expression>),
}

pub enum XmlAttributeOrExpression {
    Attribute(XmlAttribute),
    Expression(Rc<Expression>),
}

pub struct XmlAttribute {
    pub name: (String, Location),
    pub value: XmlAttributeValueOrExpression,
}

pub enum XmlAttributeValueOrExpression {
    Value(String),
    Expression(Rc<Expression>),
}

pub enum ReservedNamespace {
    Public,
    Private,
    Protected,
    Internal,
}

pub enum ObjectInitializerItem {
    Field {
        key: Rc<(ObjectKey, Location)>,
        /// Used when parsing an object initializer as a destructuring pattern.
        #[doc(hidden)]
        destructuring_non_null: bool,
        /// If `None`, this is a shorthand field.
        value: Option<Rc<Expression>>,
    },
    Rest(Rc<Expression>),
}

pub enum ObjectKey {
    Id(NonAttributeQualifiedIdentifier),
    String(String),
    Number(f64),
    Brackets(Rc<Expression>),
}

pub struct Destructuring {
    pub location: Location,
    pub kind: DestructuringKind,
    /// Indicates whether the pattern asserts that the
    /// destructuring base is not any of `undefined` and `null`.
    /// The patterns use the `!` punctuator to indicate this behavior.
    pub non_null: bool,
    pub type_annotation: Option<Rc<TypeExpression>>,
}

pub enum DestructuringKind {
    Binding {
        name: (String, Location),
    },
    Record(Vec<Rc<RecordDestructuringField>>),
    Array(Vec<Rc<ArrayDestructuringItem>>),
}

pub struct RecordDestructuringField {
    pub location: Location,
    pub key: Rc<(RecordDestructuringKey, Location)>,
    pub non_null: bool,
    pub alias: Option<Rc<Destructuring>>,
}

pub enum RecordDestructuringKey {
    Id(QualifiedIdentifier),
    String(String),
    Number(f64),
    Brackets(Rc<Expression>),
}

pub enum ArrayDestructuringItem {
    Pattern(Rc<Destructuring>),
    Rest(Rc<Destructuring>, Location),
}

pub struct TypeExpression {
    pub location: Location,
    pub kind: TypeExpressionKind,
}

pub enum TypeExpressionKind {
    Id(QualifiedIdentifier),
    DotMember {
        base: Rc<TypeExpression>,
        member: QualifiedIdentifier,
    },
    Tuple(Vec<Rc<TypeExpression>>),
    Record(Vec<RecordTypeField>),
    /// `*`
    Any,
    Void,
    Undefined,
    Nullable(Rc<TypeExpression>),
    NonNullable(Rc<TypeExpression>),
    Function {
        params: Vec<FunctionTypeParam>,
        return_annotation: Rc<TypeExpression>,
    },
    StringLiteral(String),
    NumberLiteral(f64),
    /// `|`
    Union(Vec<Rc<TypeExpression>>),
    /// `&`
    Complement {
        base: Rc<TypeExpression>,
        complement: Rc<TypeExpression>,
    },
    /// `base.<T1, Tn>`
    WithTypeArguments {
        base: Rc<TypeExpression>,
        arguments: Vec<Rc<TypeExpression>>,
    },
}

pub struct FunctionTypeParam {
    pub kind: FunctionParamKind,
    pub name: (String, Location),
    pub type_annotation: Option<Rc<TypeExpression>>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum FunctionParamKind {
    Required,
    Optional,
    Rest,
}

pub struct RecordTypeField {
    pub readonly: bool,
    pub key: Rc<(RecordTypeKey, Location)>,
    pub key_suffix: RecordTypeKeySuffix,
    pub type_annotation: Option<Rc<TypeExpression>>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum RecordTypeKeySuffix {
    None,
    NonNullable,
    Nullable,
}

pub enum RecordTypeKey {
    Id(NonAttributeQualifiedIdentifier),
    String(String),
    Number(f64),
    Brackets(Rc<Expression>),
}

pub struct Statement {
    pub location: Location,
    pub kind: StatementKind,
}

pub enum StatementKind {
    Empty,
    Super(Vec<Rc<Expression>>),
    Block(Block),
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
        block: Block,
        catch_clauses: Vec<CatchClause>,
        finally_clause: FinallyClause,
    },
    Expression(Rc<Expression>),
    Labeled {
        label: (String, Location),
        statement: Rc<Statement>,
    },
    DefaultXmlNamespace(Rc<Expression>),
    SimpleVariableDeclaration(SimpleVariableDeclaration),
}

pub struct CatchClause {
    pub pattern: Rc<Destructuring>,
    pub block: Block,
}

pub struct FinallyClause {
    pub block: Block,
}

pub enum ForInit {
    Variable(SimpleVariableDeclaration),
    Expression(Rc<Expression>),
}

pub enum ForInLeft {
    Variable(SimpleVariableDeclaration),
    Expression(Rc<Expression>),
}

pub struct SimpleVariableDeclaration {
    pub kind: (VariableKind, Location),
    pub bindings: Vec<VariableBinding>,
}

pub struct VariableBinding {
    pub pattern: Rc<Destructuring>,
    pub init: Option<Rc<Expression>>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum VariableKind {
    Var,
    Const,
}

pub struct SwitchCase {
    pub test: Option<Rc<Expression>>,
    pub consequent: Vec<Rc<Directive>>,
}

pub struct SwitchTypeCase {
    pub pattern: Rc<Destructuring>,
    pub block: Block,
}

pub struct Block(pub Vec<Rc<Directive>>);

pub struct Directive {
    pub location: Location,
    pub kind: DirectiveKind,
}

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

pub struct ClassDefinition {
    pub annotations: DefinitionAnnotations,
    pub name: (String, Location),
    pub generics: Generics,
    pub extends_clause: Option<Rc<TypeExpression>>,
    pub implements_clause: Option<Vec<Rc<TypeExpression>>>,
    pub block: Block,
}

pub struct InterfaceDefinition {
    pub annotations: DefinitionAnnotations,
    pub name: (String, Location),
    pub generics: Generics,
    pub extends_clause: Option<Vec<Rc<TypeExpression>>>,
    pub block: Block,
}

pub struct EnumDefinition {
    pub annotations: DefinitionAnnotations,
    pub name: (String, Location),
    pub block: Block,
}

pub struct NamespaceDefinition {
    pub annotations: DefinitionAnnotations,
    pub left: (String, Location),
    pub right: Option<Rc<Expression>>,
}

pub struct IncludeDirective {
    pub source: String,
    pub replaced_by: Vec<Rc<Directive>>,
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
pub struct ImportDirective {
    pub alias: Option<(String, Location)>,
    pub package_name: Vec<(String, Location)>,
    pub import_item: (ImportItem, Location),
}

pub enum ImportItem {
    Wildcard,
    /// `**`
    Recursive,
    Name(String),
}

pub struct VariableDefinition {
    pub annotations: DefinitionAnnotations,
    pub escaped: bool,
    pub kind: VariableKind,
    pub bindings: Vec<VariableBinding>,
}

pub struct FunctionDefinition {
    pub annotations: DefinitionAnnotations,
    pub escaped: bool,
    pub name: (String, Location),
    pub generics: Generics,
    pub common: Rc<FunctionCommon>,
}

pub struct ConstructorDefinition {
    pub annotations: DefinitionAnnotations,
    pub name: (String, Location),
    pub common: Rc<FunctionCommon>,
}

pub struct GetterDefinition {
    pub annotations: DefinitionAnnotations,
    pub escaped: bool,
    pub name: (String, Location),
    pub common: Rc<FunctionCommon>,
}

pub struct SetterDefinition {
    pub annotations: DefinitionAnnotations,
    pub escaped: bool,
    pub name: (String, Location),
    pub common: Rc<FunctionCommon>,
}

pub struct TypeDefinition {
    pub annotations: DefinitionAnnotations,
    pub left: (String, Location),
    pub generics: Generics,
    pub right: Rc<TypeExpression>,
}

pub struct DefinitionAnnotations {
    pub metadata: Vec<Rc<Metadata>>,
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

pub struct Metadata {
    pub location: Location,
    /// The metadata name. The metadata name may contain a single `::` delimiter.
    pub name: (String, Location),
    pub entries: Vec<MetadataEntry>,
}

pub struct MetadataEntry {
    pub key: Option<(String, Location)>,
    pub value: (String, Location),
}

pub struct Generics {
    pub params: Option<Vec<Rc<GenericParam>>>,
    pub where_clause: Option<GenericsWhere>,
}

pub struct GenericParam {
    pub location: Location,
    pub name: (String, Location),
    pub constraints: Vec<Rc<TypeExpression>>,
    pub default_type: Option<Rc<TypeExpression>>,
}

pub struct GenericsWhere {
    pub constraints: Vec<GenericsWhereConstraint>,
}

pub struct GenericsWhereConstraint {
    pub name: (String, Location),
    pub constraint: Rc<TypeExpression>,
}

pub struct FunctionCommon {
    pub flags: FunctionFlags,
    pub params: Vec<FunctionParam>,
    pub return_annotation: Option<Rc<TypeExpression>>,
    pub body: Option<FunctionBody>,
}

pub struct FunctionParam {
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

pub enum FunctionBody {
    Block(Block),
    Expression(Rc<Expression>),
}