use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum OperatorPrecedence {
    Postfix = 17,
    Unary = 16,
    Exponentiation = 15,
    Multiplicative = 14,
    Additive = 13,
    Shift = 12,
    Relational = 11,
    Equality = 10,
    BitwiseAnd = 9,
    BitwiseXor = 8,
    BitwiseOr = 7,
    LogicalAnd = 6,
    LogicalOr = 5,
    LogicalXor = 4,
    /// Includes logical OR and nullish coalescing (`??`).
    LogicalOrAndOther = 3,
    /// Includes assignment, conditional, `yield`, and rest (`...`) operators
    /// and arrow functions.
    AssignmentAndOther = 2,
    List = 1,
}

impl OperatorPrecedence {
    pub fn add_one(&self) -> Option<Self> {
        FromPrimitive::from_u32(*self as u32 + 1)
    }

    pub fn value_of(&self) -> u32 {
        *self as u32
    }

    pub fn includes(&self, other: &Self) -> bool {
        *self <= *other
    }
}

impl TryFrom<u32> for OperatorPrecedence {
    type Error = ();
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if let Some(v) = FromPrimitive::from_u32(value as u32) { Ok(v) } else { Err(()) }
    }
}