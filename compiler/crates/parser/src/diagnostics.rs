use crate::Location;

#[derive(Clone)]
pub struct Diagnostic {
    location: Location,
    id: Option<u64>,
    kind: DiagnosticKind,
    is_warning: bool,
    is_verify_error: bool,
}

impl PartialEq for Diagnostic {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location &&
        self.id == other.id
    }
}

impl PartialOrd for Diagnostic {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.location.partial_cmp(&other.location)
    }
}

#[derive(Clone)]
pub enum DiagnosticKind {}