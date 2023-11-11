/// Indicates a fatal syntax error that leads parsing
/// to finish without a resulting node.
/// 
/// This ActionScript 3 parser is intolerant in general,
/// thus resulting in a `ParserFailure` for almost any syntax error.
#[derive(Copy, Clone)]
pub struct ParserFailure;

pub(crate) struct NumericRangeError;