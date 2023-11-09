use std::collections::HashMap;
use maplit::hashmap;
use crate::{Location, Token, diagnostics_defaults};

/// Represents a source diagnostic.
/// 
/// Arguments are formatted using integer keys counting from one.
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

    pub fn format_default(&self) -> String {
        let category = (if self.is_verify_error {
            "Verify error"
        } else if self.is_warning {
            "Warning"
        } else {
            "Syntax error"
        }).to_owned();

        let file_path = self.location.source.file_path.clone().map_or("".to_owned(), |s| format!("{s}:"));
        let line = self.location.first_line_number();
        let column = self.location.first_column() + 1;
        let message = self.format_message_default();
        let id = self.id().to_string();
        format!("{file_path}{line}:{column}: {category} #{id}: {message}")
    }

    pub fn format_message_default(&self) -> String {
        self.format_message(&diagnostics_defaults::MESSAGES)
    }

    pub fn format_message(&self, messages: &HashMap<i32, String>) -> String {
        let mut string_arguments: HashMap<String, String> = hashmap!{};
        let mut i = 1;
        for argument in &self.arguments {
            string_arguments.insert(i.to_string(), self.format_argument(*argument.clone()));
            i += 1;
        }
        use late_format::LateFormat;
        let Some(msg) = messages.get(&self.id()) else {
            let id = self.id();
            panic!("Message map is missing message for ID {id}");
        };
        msg.late_format(string_arguments)
    }

    fn format_argument(&self, argument: DiagnosticArgument) -> String {
        match argument {
            DiagnosticArgument::String(s) => s.clone(),
            DiagnosticArgument::Token(t) => t.to_string(),
        }
    }
}

pub macro diagnostic_arguments {
    ($($variant:ident($value:expr)),*) => { vec![ $(Box::new(DiagnosticArgument::$variant($value))),* ] },
}

#[derive(Clone)]
pub enum DiagnosticArgument {
    String(String),
    Token(Token),
}

#[repr(i32)]
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum DiagnosticKind {
    UnexpectedOrInvalidToken = 1024,
    UnexpectedEnd = 1025,
    FailedProcessingNumericLiteral = 1026,
    UnallowedNumericSuffix = 1027,
    UnallowedLineBreak = 1028,
    Expected = 1029,
    ExpectedIdentifier = 1030,
}

impl DiagnosticKind {
    pub fn id(&self) -> i32 {
        *self as i32
    }
}