#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Operator {
    PostIncrement,
    PostDecrement,
    NonNull,
    Delete,
    Void,
    Typeof,
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
    pub fn associativity(&self) -> Option<BinaryAssociativity> {
        match *self {
            Self::Multiply |
            Self::Divide |
            Self::Remainder |
            Self::Add |
            Self::Subtract |
            Self::ShiftLeft |
            Self::ShiftRight |
            Self::ShiftRightUnsigned |
            Self::Lt |
            Self::Gt |
            Self::Le |
            Self::Ge |
            Self::In |
            Self::NotIn |
            Self::Instanceof |
            Self::NotInstanceof |
            Self::Is |
            Self::IsNot |
            Self::Equals |
            Self::NotEquals |
            Self::StrictEquals |
            Self::StrictNotEquals |
            Self::BitwiseAnd |
            Self::BitwiseXor |
            Self::BitwiseOr |
            Self::LogicalAnd |
            Self::LogicalXor |
            Self::LogicalOr |
            Self::NullCoalescing => Some(BinaryAssociativity::LeftToRight),

            Self::Power => Some(BinaryAssociativity::RightToLeft),

            _ => None,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BinaryAssociativity {
    LeftToRight,
    RightToLeft,
}