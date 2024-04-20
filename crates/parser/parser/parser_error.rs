use crate::ns::*;

/// Indicates a fatal syntax error that leads the parser
/// to complete without a resulting node.
#[derive(Copy, Clone, Debug)]
pub enum ParserError {
    Common,
}

/// Returns the identifier name that is specially reserved
/// for invalidated identifiers that could not be parsed.
pub const INVALIDATED_IDENTIFIER: &'static str = "\x00";

#[derive(Clone)]
pub(crate) enum MetadataRefineError {
    Syntax,
}

#[derive(Clone)]
pub(crate) struct MetadataRefineError1(pub MetadataRefineError, pub Location);
