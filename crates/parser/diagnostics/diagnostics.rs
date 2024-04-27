use std::any::Any;

use maplit::hashmap;
use crate::ns::*;

#[path = "diagnostics_english_resources.rs"]
mod diagnostics_english_resources;

/// Represents a diagnostic originated from a compilation unit.
/// 
/// Arguments are formatted using integer keys counted from 1 (one).
#[derive(Clone)]
pub struct Diagnostic {
    pub(crate) location: Location,
    pub(crate) kind: DiagnosticKind,
    pub(crate) is_warning: bool,
    pub(crate) is_verify_error: bool,
    pub(crate) arguments: Vec<Rc<dyn DiagnosticArgument>>,
    pub(crate) custom_kind: RefCell<Option<Rc<dyn Any>>>,
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
    pub fn new_syntax_error(location: &Location, kind: DiagnosticKind, arguments: Vec<Rc<dyn DiagnosticArgument>>) -> Self {
        Self {
            location: location.clone(),
            kind,
            is_verify_error: false,
            is_warning: false,
            arguments,
            custom_kind: RefCell::new(None),
        }
    }

    pub fn new_verify_error(location: &Location, kind: DiagnosticKind, arguments: Vec<Rc<dyn DiagnosticArgument>>) -> Self {
        Self {
            location: location.clone(),
            kind,
            is_verify_error: true,
            is_warning: false,
            arguments,
            custom_kind: RefCell::new(None),
        }
    }

    pub fn new_warning(location: &Location, kind: DiagnosticKind, arguments: Vec<Rc<dyn DiagnosticArgument>>) -> Self {
        Self {
            location: location.clone(),
            kind,
            is_verify_error: false,
            is_warning: true,
            arguments,
            custom_kind: RefCell::new(None),
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

    pub fn is_error(&self) -> bool {
        !self.is_warning
    }

    pub fn is_syntax_error(&self) -> bool {
        !self.is_verify_error && !self.is_warning
    }

    pub fn is_verify_error(&self) -> bool {
        self.is_verify_error
    }

    pub fn arguments(&self) -> Vec<Rc<dyn DiagnosticArgument>> {
        self.arguments.clone()
    }

    pub fn id(&self) -> i32 {
        self.kind.id()
    }

    pub fn custom_kind(&self) -> Option<Rc<dyn Any>> {
        self.custom_kind.borrow().clone()
    }

    pub fn set_custom_kind(&self, id: Option<Rc<dyn Any>>) {
        self.custom_kind.replace(id);
    }

    /// Formats the diagnostic by overriding the message text.
    pub fn format_with_message(&self, message: &str, id: Option<i32>) -> String {
        let category = (if self.is_verify_error {
            "Verify error"
        } else if self.is_warning {
            "Warning"
        } else {
            "Syntax error"
        }).to_owned();

        let file_path = self.location.compilation_unit.file_path.clone().map_or("".to_owned(), |s| format!("{s}:"));
        let line = self.location.first_line_number();
        let column = self.location.first_column() + 1;
        if let Some(id) = id {
            format!("{file_path}{line}:{column}: {category} #{}: {message}", id.to_string())
        } else {
            format!("{file_path}{line}:{column}: {category}: {message}")
        }
    }

    /// Formats the diagnostic in English.
    pub fn format_english(&self) -> String {
        self.format_with_message(&self.format_message_english(), Some(self.id()))
    }

    pub fn format_message_english(&self) -> String {
        self.format_message(&diagnostics_english_resources::DATA)
    }

    pub fn format_message(&self, messages: &HashMap<i32, String>) -> String {
        let mut string_arguments: HashMap<String, String> = hashmap!{};
        let mut i = 1;
        for argument in &self.arguments {
            string_arguments.insert(i.to_string(), argument.to_string());
            i += 1;
        }
        use late_format::LateFormat;
        let Some(msg) = messages.get(&self.id()) else {
            let id = self.id();
            panic!("Message resource is missing for ID {id}");
        };
        msg.late_format(string_arguments)
    }
}

/// The `diagarg![...]` literal is used for initializing
/// diagnostic arguments.
/// 
/// For example: `diagarg![token, "foo".into()]`.
pub macro diagarg {
    ($($value:expr),*) => { vec![ $(Rc::new($value)),* ] },
}

pub trait DiagnosticArgument: Any + ToString + 'static {
}

impl DiagnosticArgument for String {}

impl DiagnosticArgument for Token {}