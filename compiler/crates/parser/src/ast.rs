use std::rc::Rc;
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
    Identifier(String, Location),
    Brackets(Rc<Expression>),
}

pub struct Expression {
    pub location: Location,
    pub kind: ExpressionKind,
}

pub enum ExpressionKind {
    NullLiteral,
    BooleanLiteral(bool),
    NumericLiteral(f64),
    StringLiteral(String),
    ThisLiteral,
    RegExpLiteral {
        body: String,
        flags: String,
    },
    QualifiedIdentifier(QualifiedIdentifier),
    XMLMarkup(String),
    XMLElement(XMLElement),
    XMLList(Vec<XMLElementContent>),
    ReservedNamespace(ReservedNamespace),
    ParenExpression(Rc<Expression>),
    /// Present as part of an array initializer only.
    /// This expression is not valid in other contexts.
    RestExpression(Rc<Expression>),
    ArrayInitializer {
        /// Element sequence possibly containing RestExpressions and ellisions.
        elements: Vec<Option<Rc<Expression>>>,
        type_annotation: Option<Rc<TypeAnnotation>>,
    },
    ObjectInitializer {
        fields: Vec<ObjectFieldOrRest>,
        type_annotation: Option<Rc<TypeAnnotation>>,
    },
    FunctionExpression {
        name: Option<(String, Location)>,
        common: Rc<FunctionCommon>,
    },
    SuperExpression(Option<Vec<Rc<Expression>>>),

    /// Expression containing an optional chaining operator.
    OptionalChaining {
        base: Rc<Expression>,
        /// Postfix operators that execute if the base is not `null`
        /// and not `undefined`. The topmost node in this field is
        /// [`ExpressionKind::OptionalChainingHost`], which holds
        /// a non-null value.
        operations: Rc<Expression>,
    },

    /// The topmost expression from which postfix operators
    /// follow in an [`ExpressionKind::OptionalChaining`] expression
    /// inside the `operations` field.
    OptionalChainingHost,
}

pub enum XMLElementContent {
    Interpolation(Rc<Expression>),
    Markup(String),
    Text(String),
    XMLElement(XMLElement),
}

pub struct XMLElement {
    pub opening_tag_name: XMLTagName,
    pub attributes: Vec<XMLAttributeOrInterpolation>,
    pub content: Vec<XMLElementContent>,
    pub closing_tag_name: Option<XMLTagName>,
}

pub enum XMLTagName {
    Name(String),
    Interpolation(Rc<Expression>),
}

pub enum XMLAttributeOrInterpolation {
    Attribute(XMLAttribute),
    Interpolation(Rc<Expression>),
}

pub struct XMLAttribute {
    pub name: String,
    pub value: XMLAttributeValueOrInterpolation,
}

pub enum XMLAttributeValueOrInterpolation {
    Value(String),
    Interpolation(Rc<Expression>),
}

pub enum ReservedNamespace {
    Public,
    Private,
    Protected,
    Internal,
}

pub enum ObjectFieldOrRest {
    Field {
        key: Rc<(ObjectKey, Location)>,
        /// A key suffix that has effect solely when parsing
        /// an object initializer as a destructuring pattern.
        #[doc(hidden)]
        key_suffix: ObjectKeySuffix,
        /// If `None`, this is a shorthand field.
        value: Option<Rc<Expression>>,
    },
    Rest(Rc<Expression>),
}

pub enum ObjectKeySuffix {
    None,
    NonNull,
}

pub enum ObjectKey {
    NonAttributeQualifiedIdentifier(NonAttributeQualifiedIdentifier),
    String(String),
    Number(f64),
    Brackets(Rc<Expression>),
}

pub struct TypeAnnotation {
    pub location: Location,
    pub kind: TypeAnnotationKind,
}

pub enum TypeAnnotationKind {
    QualifiedIdentifier(QualifiedIdentifier),
    Member {
        base: Rc<TypeAnnotation>,
        member: QualifiedIdentifier,
    },
    Tuple(Vec<Rc<TypeAnnotation>>),
    Record(Vec<RecordTypeField>),
    Void,
    Nullable(Rc<TypeAnnotation>),
    NonNullable(Rc<TypeAnnotation>),
    FunctionType {
        params: Vec<FunctionTypeParam>,
        return_annotation: Rc<TypeAnnotation>,
    },
    StringLiteral(String),
    NumberLiteral(f64),
    Union(Vec<Rc<TypeAnnotation>>),
    /// `&`
    Complement {
        base: Rc<TypeAnnotation>,
        complement: Rc<TypeAnnotation>,
    },
    /// `base.<T1, Tn>`
    TypeArguments {
        base: Rc<TypeAnnotation>,
        arguments: Vec<Rc<TypeAnnotation>>,
    },
}

pub struct FunctionTypeParam {
    pub kind: FunctionTypeParamKind,
    pub name: (String, Location),
    pub type_annotation: Option<Rc<TypeAnnotation>>,
}

pub enum FunctionTypeParamKind {
    Required,
    Optional,
    Rest,
}

pub struct RecordTypeField {
    pub key: Rc<(RecordTypeKey, Location)>,
    pub key_suffix: RecordTypeKeySuffix,
}

pub enum RecordTypeKeySuffix {
    None,
    NonNullable,
    Nullable,
}

pub enum RecordTypeKey {
    NonAttributeQualifiedIdentifier(NonAttributeQualifiedIdentifier),
    String(String),
    Number(f64),
    Brackets(Rc<Expression>),
}