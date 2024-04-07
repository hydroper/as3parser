use crate::ns::*;
use serde::{Serialize, Deserialize};

/// Represents an ActionScript operator.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
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
    Instanceof,
    In,
    NotIn,
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

/// Represents binary operator associativity.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BinaryAssociativity {
    LeftToRight,
    RightToLeft,
}

/// Represents an ActionScript binary operator.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct BinaryOperator(pub Operator, pub OperatorPrecedence, pub BinaryAssociativity);

impl BinaryOperator {
    pub fn operator(&self) -> Operator {
        self.0
    }

    pub fn precedence(&self) -> OperatorPrecedence {
        self.1
    }

    pub fn associativity(&self) -> BinaryAssociativity {
        self.2
    }

    pub fn right_precedence(&self) -> OperatorPrecedence {
        if self.operator() == Operator::NullCoalescing {
            OperatorPrecedence::BitwiseOr
        } else {
            self.precedence().add(if self.associativity() == BinaryAssociativity::LeftToRight { 1 } else { 0 }).unwrap()
        }
    }
}

impl TryFrom<Operator> for BinaryOperator {
    type Error = ();
    /// Constructs `BinaryOperator` from abstract operator.
    fn try_from(value: Operator) -> Result<Self, Self::Error> {
        match value {
            Operator::Multiply => Ok(BinaryOperator(value, OperatorPrecedence::Multiplicative, BinaryAssociativity::LeftToRight)),
            Operator::Divide => Ok(BinaryOperator(value, OperatorPrecedence::Multiplicative, BinaryAssociativity::LeftToRight)),
            Operator::Remainder => Ok(BinaryOperator(value, OperatorPrecedence::Multiplicative, BinaryAssociativity::LeftToRight)),
            Operator::Add => Ok(BinaryOperator(value, OperatorPrecedence::Additive, BinaryAssociativity::LeftToRight)),
            Operator::Subtract => Ok(BinaryOperator(value, OperatorPrecedence::Additive, BinaryAssociativity::LeftToRight)),
            Operator::ShiftLeft => Ok(BinaryOperator(value, OperatorPrecedence::Shift, BinaryAssociativity::LeftToRight)),
            Operator::ShiftRight => Ok(BinaryOperator(value, OperatorPrecedence::Shift, BinaryAssociativity::LeftToRight)),
            Operator::ShiftRightUnsigned => Ok(BinaryOperator(value, OperatorPrecedence::Shift, BinaryAssociativity::LeftToRight)),
            Operator::Lt => Ok(BinaryOperator(value, OperatorPrecedence::Relational, BinaryAssociativity::LeftToRight)),
            Operator::Gt => Ok(BinaryOperator(value, OperatorPrecedence::Relational, BinaryAssociativity::LeftToRight)),
            Operator::Le => Ok(BinaryOperator(value, OperatorPrecedence::Relational, BinaryAssociativity::LeftToRight)),
            Operator::Ge => Ok(BinaryOperator(value, OperatorPrecedence::Relational, BinaryAssociativity::LeftToRight)),
            Operator::In => Ok(BinaryOperator(value, OperatorPrecedence::Relational, BinaryAssociativity::LeftToRight)),
            Operator::NotIn => Ok(BinaryOperator(value, OperatorPrecedence::Relational, BinaryAssociativity::LeftToRight)),
            Operator::Instanceof => Ok(BinaryOperator(value, OperatorPrecedence::Relational, BinaryAssociativity::LeftToRight)),
            Operator::Is => Ok(BinaryOperator(value, OperatorPrecedence::Relational, BinaryAssociativity::LeftToRight)),
            Operator::IsNot => Ok(BinaryOperator(value, OperatorPrecedence::Relational, BinaryAssociativity::LeftToRight)),
            Operator::Equals => Ok(BinaryOperator(value, OperatorPrecedence::Equality, BinaryAssociativity::LeftToRight)),
            Operator::NotEquals => Ok(BinaryOperator(value, OperatorPrecedence::Equality, BinaryAssociativity::LeftToRight)),
            Operator::StrictEquals => Ok(BinaryOperator(value, OperatorPrecedence::Equality, BinaryAssociativity::LeftToRight)),
            Operator::StrictNotEquals => Ok(BinaryOperator(value, OperatorPrecedence::Equality, BinaryAssociativity::LeftToRight)),
            Operator::BitwiseAnd => Ok(BinaryOperator(value, OperatorPrecedence::BitwiseAnd, BinaryAssociativity::LeftToRight)),
            Operator::BitwiseXor => Ok(BinaryOperator(value, OperatorPrecedence::BitwiseXor, BinaryAssociativity::LeftToRight)),
            Operator::BitwiseOr => Ok(BinaryOperator(value, OperatorPrecedence::BitwiseOr, BinaryAssociativity::LeftToRight)),
            Operator::LogicalAnd => Ok(BinaryOperator(value, OperatorPrecedence::LogicalAnd, BinaryAssociativity::LeftToRight)),
            Operator::LogicalXor => Ok(BinaryOperator(value, OperatorPrecedence::LogicalXor, BinaryAssociativity::LeftToRight)),
            Operator::LogicalOr => Ok(BinaryOperator(value, OperatorPrecedence::LogicalOrAndOther, BinaryAssociativity::LeftToRight)),
            Operator::NullCoalescing => Ok(BinaryOperator(value, OperatorPrecedence::LogicalOrAndOther, BinaryAssociativity::LeftToRight)),

            Operator::Power => Ok(BinaryOperator(value, OperatorPrecedence::Exponentiation, BinaryAssociativity::RightToLeft)),

            _ => Err(()),
        }
    }
}