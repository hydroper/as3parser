use std::rc::Rc;
use std::cell::{RefCell, Cell};

/// Represents an ActionScript source file.
pub struct Source {
    pub(crate) file_path: Option<String>,
    pub(crate) text: String,
    pub(crate) line_number_offsets: RefCell<Vec<usize>>,
    pub(crate) already_tokenized: Cell<bool>,
}

impl Source {
    /// Constructs a source file in unparsed and non verified state.
    pub fn new(file_path: Option<String>, text: String) -> Rc<Self> {
        Rc::new(Self {
            file_path,
            text,
            line_number_offsets: RefCell::new(vec! [0, 0]),
            already_tokenized: Cell::new(false),
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
}