use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum OperatorPrecedence {
    Postfix,
    Unary,
    Exponentiation,
    Multiplicative,
    Additive,
    Shift,
    Relational,
    Equality,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    LogicalAnd,
    LogicalOr,
    LogicalXor,
    LogicalOrAndNullCoalescing,
    /// Includes assignment, conditional, `yield`, and rest (`...`) operators
    /// and arrow functions.
    AssignmentAndMisc,
    List,
}

impl OperatorPrecedence {
    pub fn higher(&self) -> Option<Self> {
        FromPrimitive::from_u32(*self as u32 - 1)
    }
}