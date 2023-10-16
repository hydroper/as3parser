use std::rc::Rc;
use std::cell::{RefCell, Cell};
use crate::Diagnostic;

/// Represents an ActionScript source file.
pub struct Source {
    pub(crate) file_path: Option<String>,
    pub(crate) text: String,
    pub(crate) line_number_offsets: RefCell<Vec<usize>>,
    pub(crate) already_tokenized: Cell<bool>,
    pub(crate) diagnostics: RefCell<Vec<Diagnostic>>,
    pub(crate) invalidated: Cell<bool>,
}

impl Source {
    /// Constructs a source file in unparsed and non verified state.
    pub fn new(file_path: Option<String>, text: String) -> Rc<Self> {
        Rc::new(Self {
            file_path,
            text,
            line_number_offsets: RefCell::new(vec![0, 0]),
            already_tokenized: Cell::new(false),
            diagnostics: RefCell::new(vec![]),
            invalidated: Cell::new(false),
        })
    }

    /// File path of the source or `None` if not a file.
    pub fn file_path(&self) -> Option<String> {
        self.file_path.clone()
    }

    /// Source text.
    pub fn text(&self) -> &String {
        &self.text
    }

    /// Whether the source contains any errors after parsing
    /// and/or verification.
    pub fn invalidated(&self) -> bool {
        self.invalidated.get()
    }

    /// Diagnostics of the source file after parsing and/or
    /// verification.
    pub fn diagnostics(&self) -> Vec<Diagnostic> {
        self.diagnostics.borrow().clone()
    }

    pub fn sort_diagnostics(&self) {
        self.diagnostics.borrow_mut().sort();
    }

    pub fn add_diagnostic(&self, diagnostic: Diagnostic) {
        if !diagnostic.is_warning() {
            self.invalidated.set(true);
        }
        self.diagnostics.borrow_mut().push(diagnostic);
    }
}