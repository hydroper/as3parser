use crate::ns::*;
use serde::{Serialize, Deserialize};

/// Expression attached with a source location.
#[derive(Clone, Serialize, Deserialize)]
pub enum Expression {
    QualifiedIdentifier(QualifiedIdentifier),
    Embed(EmbedExpression),
    Paren(ParenExpression),
    NullLiteral(NullLiteral),
    BooleanLiteral(BooleanLiteral),
    NumericLiteral(NumericLiteral),
    StringLiteral(StringLiteral),
    ThisLiteral(ThisLiteral),
    RegExpLiteral(RegExpLiteral),
    Xml(XmlExpression),
    XmlMarkup(XmlMarkupExpression),
    XmlList(XmlListExpression),
    ArrayLiteral(ArrayLiteral),
    VectorLiteral(VectorLiteral),
    ObjectInitializer(ObjectInitializer),
    Function(FunctionExpression),
    ImportMeta(ImportMeta),
    New(NewExpression),
    Member(MemberExpression),
    ComputedMember(ComputedMemberExpression),
    Descendants(DescendantsExpression),
    Filter(FilterExpression),
    Super(SuperExpression),
    Call(CallExpression),
    WithTypeArguments(ExpressionWithTypeArguments),
    Unary(UnaryExpression),
    OptionalChaining(OptionalChainingExpression),
    OptionalChainingPlaceholder(OptionalChainingPlaceholder),
    Binary(BinaryExpression),
    Conditional(ConditionalExpression),
    Assignment(AssignmentExpression),
    Sequence(SequenceExpression),
    NullableType(NullableTypeExpression),
    NonNullableType(NonNullableTypeExpression),
    AnyType(AnyTypeExpression),
    VoidType(VoidTypeExpression),
    ArrayType(ArrayTypeExpression),
    TupleType(TupleTypeExpression),
    FunctionType(FunctionTypeExpression),
    Invalidated(InvalidatedExpression),
    ReservedNamespace(ReservedNamespaceExpression),
}

impl Expression {
    pub fn location(&self) -> Location {
        match self {
            Self::QualifiedIdentifier(e) => e.location.clone(),
            Self::Embed(e) => e.location.clone(),
            Self::Paren(e) => e.location.clone(),
            Self::NullLiteral(e) => e.location.clone(),
            Self::BooleanLiteral(e) => e.location.clone(),
            Self::NumericLiteral(e) => e.location.clone(),
            Self::StringLiteral(e) => e.location.clone(),
            Self::ThisLiteral(e) => e.location.clone(),
            Self::RegExpLiteral(e) => e.location.clone(),
            Self::Xml(e) => e.location.clone(),
            Self::XmlMarkup(e) => e.location.clone(),
            Self::XmlList(e) => e.location.clone(),
            Self::ArrayLiteral(e) => e.location.clone(),
            Self::VectorLiteral(e) => e.location.clone(),
            Self::ObjectInitializer(e) => e.location.clone(),
            Self::Function(e) => e.location.clone(),
            Self::ImportMeta(e) => e.location.clone(),
            Self::New(e) => e.location.clone(),
            Self::Member(e) => e.location.clone(),
            Self::ComputedMember(e) => e.location.clone(),
            Self::Descendants(e) => e.location.clone(),
            Self::Filter(e) => e.location.clone(),
            Self::Super(e) => e.location.clone(),
            Self::Call(e) => e.location.clone(),
            Self::WithTypeArguments(e) => e.location.clone(),
            Self::Unary(e) => e.location.clone(),
            Self::OptionalChaining(e) => e.location.clone(),
            Self::OptionalChainingPlaceholder(e) => e.location.clone(),
            Self::Binary(e) => e.location.clone(),
            Self::Conditional(e) => e.location.clone(),
            Self::Assignment(e) => e.location.clone(),
            Self::Sequence(e) => e.location.clone(),
            Self::NullableType(e) => e.location.clone(),
            Self::NonNullableType(e) => e.location.clone(),
            Self::AnyType(e) => e.location.clone(),
            Self::VoidType(e) => e.location.clone(),
            Self::ArrayType(e) => e.location.clone(),
            Self::TupleType(e) => e.location.clone(),
            Self::FunctionType(e) => e.location.clone(),
            Self::Invalidated(e) => e.location.clone(),
            Self::ReservedNamespace(e) => e.location(),
        }
    }

    pub(crate) fn to_metadata(&self, parser: &Parser) -> Result<Option<Vec<Attribute>>, MetadataRefineError1> {
        match self {
            Self::ArrayLiteral(ArrayLiteral { elements, asdoc, .. }) => {
                if elements.len() != 1 {
                    return Ok(None);
                }
                if let Element::Expression(ref exp) = elements[0] {
                    Ok(Some(vec![Attribute::Metadata(parser.refine_metadata(exp, asdoc.clone()).map_err(|e| MetadataRefineError1(e, exp.location()))?)]))
                } else {
                    Ok(None)
                }
            },
            Self::ComputedMember(ComputedMemberExpression { base, asdoc, key, .. }) => {
                let a = base.to_metadata(parser)?;
                if a.is_none() {
                    return Ok(None);
                }
                let mut a = a.unwrap();
                if matches!(key.as_ref(), Self::Sequence(_)) {
                    return Ok(None);
                }
                a.push(Attribute::Metadata(parser.refine_metadata(key, asdoc.clone()).map_err(|e| MetadataRefineError1(e, key.location()))?));
                Ok(Some(a))
            },
            _ => Ok(None),
        }
    }

    pub fn to_identifier_name_or_asterisk(&self) -> Option<(String, Location)> {
        match self {
            Self::QualifiedIdentifier(id) => id.to_identifier_name_or_asterisk(),
            _ => None,
        }
    }

    pub fn to_identifier_name(&self) -> Option<(String, Location)> {
        match self {
            Self::QualifiedIdentifier(id) => id.to_identifier_name(),
            _ => None,
        }
    }

    pub fn valid_access_modifier(&self) -> bool {
        match self {
            Self::QualifiedIdentifier(id) => id.is_identifier_token(),
            Self::Member(e) => e.base.valid_access_modifier(),
            Self::ComputedMember(e) => e.base.valid_access_modifier(),
            _ => false,
        }
    }

    pub(crate) fn to_reserved_namespace_string(&self) -> Option<String> {
        if let Self::ReservedNamespace(e) = self {
            Some(e.to_string())
        } else {
            None
        }
    }

    pub(crate) fn to_reserved_namespace_attribute(&self) -> Option<Attribute> {
        if let Self::ReservedNamespace(e) = self {
            Some(e.to_attribute())
        } else {
            None
        }
    }

    pub fn is_invalidated(&self) -> bool {
        matches!(self, Self::Invalidated(_))
    }

    pub fn is_non_null_operation(&self) -> bool {
        match self {
            Self::Unary(expr) => expr.operator == Operator::NonNull,
            _ => false,
        }
    }

    pub fn is_valid_assignment_left_hand_side(&self) -> bool {
        match self {
            Self::Invalidated(_) => true,
            Self::Unary(e) => e.expression.is_valid_assignment_left_hand_side(),
            Self::ArrayLiteral(_) | Self::ObjectInitializer(_) => self.is_valid_destructuring(),
            _ => true,
        }
    }

    pub fn is_valid_destructuring(&self) -> bool {
        match self {
            Self::Invalidated(_) => true,
            Self::QualifiedIdentifier(id) => !id.attribute && id.qualifier.is_none() && match &id.id {
                QualifiedIdentifierIdentifier::Id(id) => id.0 != "*",
                _ => false,
            },
            Self::ArrayLiteral(expr) => {
                for el in &expr.elements {
                    match el {
                        Element::Elision => {},
                        Element::Expression(expr) => {
                            if !expr.is_valid_destructuring() {
                                return false;
                            }
                        },
                        Element::Rest((expr, _)) => {
                            if !expr.is_valid_destructuring() {
                                return false;
                            }
                        },
                    }
                }
                true
            },
            Self::ObjectInitializer(init) => {
                for field in init.fields.iter() {
                    match field.as_ref() {
                        InitializerField::Field { value, .. } => {
                            if let Some(val) = value {
                                if !val.is_valid_destructuring() {
                                    return false;
                                }
                            }
                        },
                        InitializerField::Rest((expr, _)) => {
                            if !expr.is_valid_destructuring() {
                                return false;
                            }
                        },
                    }
                }
                true
            },
            Self::Unary(expr) => expr.operator == Operator::NonNull && expr.expression.is_valid_destructuring(),
            _ => false,
        }
    }

    /// `CONFIG::VAR_NAME`
    pub(crate) fn to_one_branch_configuration_identifier(&self, parser: &Parser) -> Result<Option<((String, Location), (String, Location), Vec<Attribute>)>, MetadataRefineError1> {
        if let Self::QualifiedIdentifier(id) = self {
            if id.attribute {
                return Ok(None);
            }
            if let Some(q) = &id.qualifier {
                if let Some(q) = q.to_identifier_name() {
                    if let QualifiedIdentifierIdentifier::Id(id) = &id.id {
                        return Ok(Some((q, id.clone(), vec![])));
                    }
                }
            }
        }
        if let Self::ComputedMember(ComputedMemberExpression { base, asdoc, key, .. }) = self {
            let a = base.to_one_branch_configuration_identifier(parser)?;
            if a.is_none() {
                return Ok(None);
            }
            let (ns, name, mut a) = a.unwrap();
            if matches!(key.as_ref(), Self::Sequence(_)) {
                return Ok(None);
            }
            a.push(Attribute::Metadata(parser.refine_metadata(key, asdoc.clone()).map_err(|e| MetadataRefineError1(e, key.location()))?));
            return Ok(Some((ns, name, a)));
        }
        Ok(None)
    }

    /// `CONFIG::VAR_NAME`
    pub(crate) fn to_one_branch_configuration_identifier_no_metadata(&self) -> Option<((String, Location), (String, Location))> {
        if let Self::QualifiedIdentifier(id) = self {
            if id.attribute {
                return None;
            }
            if let Some(q) = &id.qualifier {
                if let Some(q) = q.to_identifier_name() {
                    if let QualifiedIdentifierIdentifier::Id(id) = &id.id {
                        return Some((q, id.clone()));
                    }
                }
            }
        }
        None
    }
}