use crate::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Operator {
    PostIncrement,
    PostDecrement,
    NonNull,
    Delete,
    Void,
    Typeof,
    Await,
    Yield,
    PreIncrement,
    PreDecrement,
    Positive,
    Negative,
    BitwiseNot,
    LogicalNot,

    Power,
    Multiply,
    Divide,
    Remainder,
    Add,
    Subtract,
    ShiftLeft,
    ShiftRight,
    ShiftRightUnsigned,
    Lt,
    Gt,
    Le,
    Ge,
    In,
    NotIn,
    Instanceof,
    NotInstanceof,
    Is,
    IsNot,
    As,
    Equals,
    NotEquals,
    StrictEquals,
    StrictNotEquals,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    LogicalAnd,
    LogicalXor,
    LogicalOr,
    NullCoalescing,
}

impl Operator {
    pub fn binary_position(&self) -> Option<(OperatorPrecedence, BinaryAssociativity)> {
        match *self {
            Self::Multiply => Some((OperatorPrecedence::Multiplicative, BinaryAssociativity::LeftToRight)),
            Self::Divide => Some((OperatorPrecedence::Multiplicative, BinaryAssociativity::LeftToRight)),
            Self::Remainder => Some((OperatorPrecedence::Multiplicative, BinaryAssociativity::LeftToRight)),
            Self::Add => Some((OperatorPrecedence::Additive, BinaryAssociativity::LeftToRight)),
            Self::Subtract => Some((OperatorPrecedence::Additive, BinaryAssociativity::LeftToRight)),
            Self::ShiftLeft => Some((OperatorPrecedence::Shift, BinaryAssociativity::LeftToRight)),
            Self::ShiftRight => Some((OperatorPrecedence::Shift, BinaryAssociativity::LeftToRight)),
            Self::ShiftRightUnsigned => Some((OperatorPrecedence::Shift, BinaryAssociativity::LeftToRight)),
            Self::Lt => Some((OperatorPrecedence::Relational, BinaryAssociativity::LeftToRight)),
            Self::Gt => Some((OperatorPrecedence::Relational, BinaryAssociativity::LeftToRight)),
            Self::Le => Some((OperatorPrecedence::Relational, BinaryAssociativity::LeftToRight)),
            Self::Ge => Some((OperatorPrecedence::Relational, BinaryAssociativity::LeftToRight)),
            Self::In => Some((OperatorPrecedence::Relational, BinaryAssociativity::LeftToRight)),
            Self::NotIn => Some((OperatorPrecedence::Relational, BinaryAssociativity::LeftToRight)),
            Self::Instanceof => Some((OperatorPrecedence::Relational, BinaryAssociativity::LeftToRight)),
            Self::NotInstanceof => Some((OperatorPrecedence::Relational, BinaryAssociativity::LeftToRight)),
            Self::Is => Some((OperatorPrecedence::Relational, BinaryAssociativity::LeftToRight)),
            Self::IsNot => Some((OperatorPrecedence::Relational, BinaryAssociativity::LeftToRight)),
            Self::Equals => Some((OperatorPrecedence::Equality, BinaryAssociativity::LeftToRight)),
            Self::NotEquals => Some((OperatorPrecedence::Equality, BinaryAssociativity::LeftToRight)),
            Self::StrictEquals => Some((OperatorPrecedence::Equality, BinaryAssociativity::LeftToRight)),
            Self::StrictNotEquals => Some((OperatorPrecedence::Equality, BinaryAssociativity::LeftToRight)),
            Self::BitwiseAnd => Some((OperatorPrecedence::BitwiseAnd, BinaryAssociativity::LeftToRight)),
            Self::BitwiseXor => Some((OperatorPrecedence::BitwiseXor, BinaryAssociativity::LeftToRight)),
            Self::BitwiseOr => Some((OperatorPrecedence::BitwiseOr, BinaryAssociativity::LeftToRight)),
            Self::LogicalAnd => Some((OperatorPrecedence::LogicalAnd, BinaryAssociativity::LeftToRight)),
            Self::LogicalXor => Some((OperatorPrecedence::LogicalXor, BinaryAssociativity::LeftToRight)),
            Self::LogicalOr => Some((OperatorPrecedence::LogicalOrAndOther, BinaryAssociativity::LeftToRight)),
            Self::NullCoalescing => Some((OperatorPrecedence::LogicalOrAndOther, BinaryAssociativity::LeftToRight)),

            Self::Power => Some((OperatorPrecedence::Exponentiation, BinaryAssociativity::RightToLeft)),

            _ => None,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BinaryAssociativity {
    LeftToRight,
    RightToLeft,
}