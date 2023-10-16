use std::collections::HashMap;
use crate::{Location, diagnostics_en};

#[derive(Clone)]
pub struct Diagnostic {
    pub(crate) location: Location,
    pub(crate) kind: DiagnosticKind,
    pub(crate) is_warning: bool,
    pub(crate) is_verify_error: bool,
    pub(crate) arguments: Vec<Box<DiagnosticArgument>>,
}

impl Eq for Diagnostic {}

impl PartialEq for Diagnostic {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location &&
        self.kind == other.kind
    }
}

impl Ord for Diagnostic {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.location.cmp(&other.location)
    }
}

impl PartialOrd for Diagnostic {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.location.partial_cmp(&other.location)
    }
}

impl Diagnostic {
    pub fn new_syntax_error(location: Location, kind: DiagnosticKind, arguments: Vec<Box<DiagnosticArgument>>) -> Self {
        Self {
            location,
            kind,
            is_verify_error: false,
            is_warning: false,
            arguments,
        }
    }

    pub fn new_verify_error(location: Location, kind: DiagnosticKind, arguments: Vec<Box<DiagnosticArgument>>) -> Self {
        Self {
            location,
            kind,
            is_verify_error: true,
            is_warning: false,
            arguments,
        }
    }

    pub fn new_warning(location: Location, kind: DiagnosticKind, arguments: Vec<Box<DiagnosticArgument>>) -> Self {
        Self {
            location,
            kind,
            is_verify_error: false,
            is_warning: true,
            arguments,
        }
    }

    pub fn location(&self) -> Location {
        self.location.clone()
    }

    pub fn kind(&self) -> DiagnosticKind {
        self.kind.clone()
    }

    pub fn is_warning(&self) -> bool {
        self.is_warning
    }

    pub fn is_verify_error(&self) -> bool {
        self.is_verify_error
    }

    pub fn arguments(&self) -> Vec<Box<DiagnosticArgument>> {
        self.arguments.clone()
    }

    pub fn id(&self) -> i32 {
        self.kind.id()
    }

    pub fn format_en(&self) -> String {
        self.format(&diagnostics_en::MESSAGES)
    }

    pub fn format(&self, message_map: &HashMap<i32, String>) -> String {}
}

#[derive(Clone)]
pub enum DiagnosticArgument {
    String(String),
}

#[repr(i32)]
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum DiagnosticKind {
    UnexpectedOrInvalidToken = 1024,
}

impl DiagnosticKind {
    pub fn id(&self) -> i32 {
        *self as i32
    }
}