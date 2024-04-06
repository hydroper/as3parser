use crate::ns::*;

/// Indicates a fatal syntax error that leads parsing
/// to finish without a resulting node.
/// 
/// This parser is intolerant in general,
/// thus resulting in a `ParsingFailure` for almost any syntax error.
#[derive(Copy, Clone, Debug)]
pub struct ParsingFailure;

#[derive(Clone)]
pub(crate) enum MetadataRefineError {
    Syntax,
    // FailedLoadingFile { path: String },
}

#[derive(Clone)]
pub(crate) struct MetadataRefineError1(pub MetadataRefineError, pub Location);