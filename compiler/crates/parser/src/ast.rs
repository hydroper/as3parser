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
    ObjectInitializer {
        fields: Vec<ObjectFieldOrRest>,
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
    Sequence(Vec<Expression>, Vec<Expression>),

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
    Interpolation(Rc<Expression>),
    Markup(String),
    Text(String),
    XmlElement(XmlElement),
}

pub struct XmlElement {
    pub opening_tag_name: XmlTagName,
    pub attributes: Vec<XmlAttributeOrInterpolation>,
    pub content: Vec<XmlElementContent>,
    pub closing_tag_name: Option<XmlTagName>,
}

pub enum XmlTagName {
    Name(String),
    Interpolation(Rc<Expression>),
}

pub enum XmlAttributeOrInterpolation {
    Attribute(XmlAttribute),
    Interpolation(Rc<Expression>),
}

pub struct XmlAttribute {
    pub name: String,
    pub value: XmlAttributeValueOrInterpolation,
}

pub enum XmlAttributeValueOrInterpolation {
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
    pub kind: FunctionTypeParamKind,
    pub name: (String, Location),
    pub type_annotation: Option<Rc<TypeExpression>>,
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