use crate::Location;

#[derive(Clone)]
pub struct Diagnostic {
    pub(crate) location: Location,
    pub(crate) kind: DiagnosticKind,
    pub(crate) is_warning: bool,
    pub(crate) is_verify_error: bool,
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
    pub fn new_syntax_error(location: Location, kind: DiagnosticKind) -> Self {
        Self {
            location,
            kind,
            is_verify_error: false,
            is_warning: false,
        }
    }

    pub fn new_verify_error(location: Location, kind: DiagnosticKind) -> Self {
        Self {
            location,
            kind,
            is_verify_error: true,
            is_warning: false,
        }
    }

    pub fn new_warning(location: Location, kind: DiagnosticKind) -> Self {
        Self {
            location,
            kind,
            is_verify_error: false,
            is_warning: true,
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

    pub fn id(&self) -> u64 {
        self.id_and_international_message().0
    }

    pub fn international_message(&self) -> String {
        self.id_and_international_message().1
    }

    pub fn id_and_international_message(&self) -> (u64, String) {
        match self.kind {}
    }
}

#[derive(Eq, PartialEq, Clone)]
pub enum DiagnosticKind {}