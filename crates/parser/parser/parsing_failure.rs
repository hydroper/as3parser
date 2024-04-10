use crate::ns::*;

/// Indicates a fatal syntax error that leads parsing
/// to finish without a resulting node.
/// 
/// This parser is intolerant in general,
/// thus resulting in a `ParsingFailure` for almost any syntax error.
#[derive(Copy, Clone, Debug)]
pub struct ParsingFailure;

/// Returns the identifier name that is specially reserved
/// for invalidated identifiers that could not be parsed.
pub const INVALIDATED_IDENTIFIER: &'static str = "\x00\x00\x00\x00\x00\x00\x00";

#[derive(Clone)]
pub(crate) enum MetadataRefineError {
    Syntax,
    // FailedLoadingFile { path: String },
}

#[derive(Clone)]
pub(crate) struct MetadataRefineError1(pub MetadataRefineError, pub Location);