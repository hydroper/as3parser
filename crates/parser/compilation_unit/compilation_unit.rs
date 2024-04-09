use std::cell::RefMut;
use crate::ns::*;
use hydroper_source_text::SourceText;

/// `CompilationUnit` identifies an AS3 compilation unit and contains
/// a source text.
pub struct CompilationUnit {
    pub(crate) file_path: Option<String>,
    pub(crate) source_text: SourceText,
    pub(crate) already_tokenized: Cell<bool>,
    diagnostics: RefCell<Vec<Diagnostic>>,
    pub(crate) error_count: Cell<u32>,
    pub(crate) warning_count: Cell<u32>,
    pub(crate) invalidated: Cell<bool>,
    pub(crate) compiler_options: Rc<CompilerOptions>,
    pub(crate) comments: RefCell<Vec<Rc<Comment>>>,
    pub(crate) included_from: RefCell<Option<Rc<CompilationUnit>>>,
    pub(crate) nested_compilation_units: RefCell<Vec<Rc<CompilationUnit>>>,
}

impl Default for CompilationUnit {
    fn default() -> Self {
        Self {
            file_path: None,
            source_text: SourceText::new("".into()),
            already_tokenized: Cell::new(false),
            diagnostics: RefCell::new(vec![]),
            invalidated: Cell::new(false),
            error_count: Cell::new(0),
            warning_count: Cell::new(0),
            compiler_options: CompilerOptions::new(),
            comments: RefCell::new(vec![]),
            nested_compilation_units: RefCell::new(vec![]),
            included_from: RefCell::new(None),
        }
    }
}

impl CompilationUnit {
    /// Constructs a source file in unparsed and non verified state.
    pub fn new(file_path: Option<String>, text: String, compiler_options: &Rc<CompilerOptions>) -> Rc<Self> {
        Rc::new(Self {
            file_path,
            source_text: SourceText::new(text),
            already_tokenized: Cell::new(false),
            diagnostics: RefCell::new(vec![]),
            invalidated: Cell::new(false),
            error_count: Cell::new(0),
            warning_count: Cell::new(0),
            compiler_options: compiler_options.clone(),
            comments: RefCell::new(vec![]),
            nested_compilation_units: RefCell::new(vec![]),
            included_from: RefCell::new(None),
        })
    }

    /// File path of the source or `None` if not a file.
    pub fn file_path(&self) -> Option<String> {
        self.file_path.clone()
    }

    /// Source text.
    pub fn text(&self) -> &String {
        &self.source_text.contents
    }

    /// Whether the source contains any errors after parsing
    /// and/or verification.
    pub fn invalidated(&self) -> bool {
        self.invalidated.get()
    }

    /// The comments present in the source file. To get mutable access to the
    /// collection of comments, use the `comments_mut()` method instead.
    pub fn comments(&self) -> Vec<Rc<Comment>> {
        let mut collection = vec![];
        for c in self.comments.borrow().iter() {
            collection.push(c.clone());
        }
        collection
    }

    /// The comments present in the source file, as a mutable collection.
    pub fn comments_mut(&self) -> RefMut<Vec<Rc<Comment>>> {
        self.comments.borrow_mut()
    }

    /// Diagnostics of the source file after parsing and/or
    /// verification.
    pub fn diagnostics(&self) -> Vec<Diagnostic> {
        self.diagnostics.borrow().clone()
    }

    /// Diagnostics of the source file after parsing and/or
    /// verification, including those of nested compilation units.
    pub fn nested_diagnostics(&self) -> Vec<Diagnostic> {
        let mut result = self.diagnostics();
        for unit in self.nested_compilation_units.borrow().iter() {
            result.extend(unit.nested_diagnostics());
        }
        result
    }

    /// Sort diagnostics from the compilation unit
    /// and any nested compilation units.
    pub fn sort_diagnostics(&self) {
        self.diagnostics.borrow_mut().sort();
        for unit in self.nested_compilation_units.borrow().iter() {
            unit.sort_diagnostics();
        }
    }

    /// If this compilation unit is subsequent of an include directive in another
    /// compilation unit, returns the compilation unit of that include directive.
    pub fn included_from(&self) -> Option<Rc<CompilationUnit>> {
        self.included_from.borrow().clone()
    }

    pub(crate) fn set_included_from(&self, included_from: Option<Rc<CompilationUnit>>) {
        self.included_from.replace(included_from);
    }

    pub(crate) fn include_directive_is_circular(&self, file_path: &str) -> bool {
        if canonicalize_path(&self.file_path.clone().unwrap_or("".into())) == canonicalize_path(file_path) {
            return true;
        }
        if let Some(included_from) = self.included_from() {
            return included_from.include_directive_is_circular(file_path);
        }
        return false;
    }

    pub fn nested_compilation_units(&self) -> Vec<Rc<CompilationUnit>> {
        let mut result = vec![];
        for unit in self.nested_compilation_units.borrow().iter() {
            result.push(unit.clone());
        }
        result
    }

    pub fn add_nested_compilation_unit(&self, unit: Rc<CompilationUnit>) {
        self.nested_compilation_units.borrow_mut().push(unit);
    }

    pub fn add_diagnostic(&self, diagnostic: Diagnostic) {
        if diagnostic.is_warning() {
            self.warning_count.set(self.warning_count.get() + 1);
        } else {
            self.error_count.set(self.error_count.get() + 1);
            self.invalidated.set(true);
        }
        self.diagnostics.borrow_mut().push(diagnostic);
    }

    pub fn error_count(&self) -> u32 {
        self.error_count.get()
    }

    pub fn warning_count(&self) -> u32 {
        self.warning_count.get()
    }

    /// Retrieves line number from an offset. The resulting line number
    /// is counted from one.
    pub fn get_line_number(&self, offset: usize) -> usize {
        self.source_text.get_line_number(offset)
    }

    /// Returns the zero based column of an offset.
    pub fn get_column(&self, offset: usize) -> usize {
        self.source_text.get_column(offset)
    }

    /// Retrieves offset from line number (counted from one).
    pub fn get_line_offset(&self, line: usize) -> Option<usize> {
        self.source_text.get_line_offset(line)
    }

    /// Retrieves the offset from the corresponding line of an offset.
    pub fn get_line_offset_from_offset(&self, offset: usize) -> usize {
        self.source_text.get_line_offset_from_offset(offset)
    }

    pub fn get_line_indent(&self, line: usize) -> usize {
        let line_offset = self.get_line_offset(line).unwrap();
        let indent = CharacterValidator::indent_count(&self.source_text.contents[line_offset..]);
        indent - line_offset
    }
}

fn canonicalize_path(path: &str) -> String {
    std::path::Path::new(path).canonicalize().unwrap_or(std::path::PathBuf::new()).to_string_lossy().into_owned()
}