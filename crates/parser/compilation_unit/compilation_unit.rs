use std::cell::RefMut;
use crate::ns::*;

const LINE_SKIP_THRESOLD: usize = 10;
const HIGHER_LINE_SKIP_THRESOLD: usize = 100;
const EXTRA_HIGHER_LINE_SKIP_THRESOLD: usize = 1_000;

/// `CompilationUnit` identifies a As3 compilation unit and contains
/// a source text.
pub struct CompilationUnit {
    pub(crate) file_path: Option<String>,
    pub(crate) text: String,

    /// Collection of ascending line number *skips* used
    /// for optimizing retrieval of line numbers or line offsets.
    pub(crate) line_skips: RefCell<Vec<LineSkip>>,
    pub(crate) line_skips_counter: Cell<usize>,

    /// Collection used before `line_skips` in line lookups
    /// to skip lines in a higher threshold.
    pub(crate) higher_line_skips: RefCell<Vec<HigherLineSkip>>,
    pub(crate) higher_line_skips_counter: Cell<usize>,

    /// Collection used before `higher_line_skips` in line lookups
    /// to skip lines in an extra higher threshold.
    pub(crate) extra_higher_line_skips: RefCell<Vec<HigherLineSkip>>,
    pub(crate) extra_higher_line_skips_counter: Cell<usize>,

    pub(crate) already_tokenized: Cell<bool>,
    diagnostics: RefCell<Vec<Diagnostic>>,
    pub(crate) error_count: Cell<u32>,
    pub(crate) warning_count: Cell<u32>,
    pub(crate) invalidated: Cell<bool>,
    pub(crate) compiler_options: Rc<CompilerOptions>,
    pub(crate) comments: RefCell<Vec<Rc<Comment>>>,
    pub(crate) nested_compilation_units: RefCell<Vec<Rc<CompilationUnit>>>,
}

#[derive(Copy, Clone)]
pub(crate) struct LineSkip {
    /// Line offset.
    pub offset: usize,
    /// Line number counting from one.
    pub line_number: usize,
}

#[derive(Copy, Clone)]
pub(crate) struct HigherLineSkip {
    /// Index to a `LineSkip`, or another `HigherLineSkip` in the case
    /// of extra higher line skips.
    pub skip_index: usize,
    /// Line offset.
    pub offset: usize,
    /// Line number counting from one.
    pub line_number: usize,
}

impl Default for CompilationUnit {
    fn default() -> Self {
        Self {
            file_path: None,
            text: "".into(),
            line_skips: RefCell::new(vec![LineSkip { offset: 0, line_number: 1 }]),
            line_skips_counter: Cell::new(0),
            higher_line_skips: RefCell::new(vec![HigherLineSkip { skip_index: 0, offset: 0, line_number: 1 }]),
            higher_line_skips_counter: Cell::new(0),
            extra_higher_line_skips: RefCell::new(vec![HigherLineSkip { skip_index: 0, offset: 0, line_number: 1 }]),
            extra_higher_line_skips_counter: Cell::new(0),
            already_tokenized: Cell::new(false),
            diagnostics: RefCell::new(vec![]),
            invalidated: Cell::new(false),
            error_count: Cell::new(0),
            warning_count: Cell::new(0),
            compiler_options: CompilerOptions::new(),
            comments: RefCell::new(vec![]),
            nested_compilation_units: RefCell::new(vec![]),
        }
    }
}

impl CompilationUnit {
    /// Constructs a source file in unparsed and non verified state.
    pub fn new(file_path: Option<String>, text: String, compiler_options: &Rc<CompilerOptions>) -> Rc<Self> {
        Rc::new(Self {
            file_path,
            text,
            line_skips: RefCell::new(vec![LineSkip { offset: 0, line_number: 1 }]),
            line_skips_counter: Cell::new(0),
            higher_line_skips: RefCell::new(vec![HigherLineSkip { skip_index: 0, offset: 0, line_number: 1 }]),
            higher_line_skips_counter: Cell::new(0),
            extra_higher_line_skips: RefCell::new(vec![HigherLineSkip { skip_index: 0, offset: 0, line_number: 1 }]),
            extra_higher_line_skips_counter: Cell::new(0),
            already_tokenized: Cell::new(false),
            diagnostics: RefCell::new(vec![]),
            invalidated: Cell::new(false),
            error_count: Cell::new(0),
            warning_count: Cell::new(0),
            compiler_options: compiler_options.clone(),
            comments: RefCell::new(vec![]),
            nested_compilation_units: RefCell::new(vec![]),
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

    pub(crate) fn push_line_skip(&self, line_number: usize, offset: usize) {
        let counter = self.line_skips_counter.get();
        if counter == LINE_SKIP_THRESOLD {
            self.line_skips.borrow_mut().push(LineSkip { line_number, offset });
            self.line_skips_counter.set(0);
        } else {
            self.line_skips_counter.set(counter + 1);
        }

        let counter = self.higher_line_skips_counter.get();
        if counter == HIGHER_LINE_SKIP_THRESOLD {
            self.higher_line_skips.borrow_mut().push(HigherLineSkip { skip_index: self.line_skips.borrow().len() - 1, line_number, offset });
            self.higher_line_skips_counter.set(0);
        } else {
            self.higher_line_skips_counter.set(counter + 1);
        }

        let counter = self.extra_higher_line_skips_counter.get();
        if counter == EXTRA_HIGHER_LINE_SKIP_THRESOLD {
            self.extra_higher_line_skips.borrow_mut().push(HigherLineSkip { skip_index: self.higher_line_skips.borrow().len() - 1, line_number, offset });
            self.extra_higher_line_skips_counter.set(0);
        } else {
            self.extra_higher_line_skips_counter.set(counter + 1);
        }
    }

    /// Retrieves line number from an offset. The resulting line number
    /// is counted from one.
    pub fn get_line_number(&self, offset: usize) -> usize {
        // Extra higher line skips
        let mut last_skip = HigherLineSkip { skip_index: 0, offset: 0, line_number: 1 };
        let skips = self.extra_higher_line_skips.borrow();
        let mut skips = skips.iter();
        while let Some(skip_1) = skips.next() {
            if offset < skip_1.offset {
                break;
            }
            last_skip = *skip_1;
        }

        // Higher line skips
        let skips = self.higher_line_skips.borrow();
        let mut skips = skips[last_skip.skip_index..].iter();
        let mut last_skip = skips.next().unwrap();
        while let Some(skip_1) = skips.next() {
            if offset < skip_1.offset {
                break;
            }
            last_skip = skip_1;
        }

        // Line skips
        let skips = self.line_skips.borrow();
        let mut skips = skips[last_skip.skip_index..].iter();
        let mut last_skip = skips.next().unwrap();
        while let Some(skip_1) = skips.next() {
            if offset < skip_1.offset {
                break;
            }
            last_skip = skip_1;
        }

        let mut current_line = last_skip.line_number;
        let mut characters = CharacterReader::from(&self.text[last_skip.offset..]);
        while last_skip.offset + characters.index() < offset {
            let ch_1 = characters.next();
            if let Some(ch_1) = ch_1 {
                if CharacterValidator::is_line_terminator(ch_1) {
                    if ch_1 == '\r' && characters.peek_or_zero() == '\n' {
                        characters.next();
                    }
                    current_line += 1;
                }
            } else {
                break;
            }
        }
        current_line
    }

    /// Retrieves offset from line number (counted from one).
    pub fn get_line_offset(&self, line: usize) -> Option<usize> {
        // Extra higher line skips
        let mut last_skip = HigherLineSkip { skip_index: 0, offset: 0, line_number: 1 };
        let skips = self.extra_higher_line_skips.borrow();
        let mut skips = skips.iter();
        while let Some(skip_1) = skips.next() {
            if line < skip_1.line_number {
                break;
            }
            last_skip = *skip_1;
        }

        // Higher line skips
        let skips = self.higher_line_skips.borrow();
        let mut skips = skips[last_skip.skip_index..].iter();
        let mut last_skip = skips.next().unwrap();
        while let Some(skip_1) = skips.next() {
            if line < skip_1.line_number {
                break;
            }
            last_skip = skip_1;
        }

        // Line skips
        let skips = self.line_skips.borrow();
        let mut skips = skips[last_skip.skip_index..].iter();
        let mut last_skip = skips.next().unwrap();
        while let Some(skip_1) = skips.next() {
            if line < skip_1.line_number {
                break;
            }
            last_skip = skip_1;
        }

        let mut current_line = last_skip.line_number;
        let mut characters = CharacterReader::from(&self.text[last_skip.offset..]);
        while current_line != line {
            let ch_1 = characters.next();
            if let Some(ch_1) = ch_1 {
                if CharacterValidator::is_line_terminator(ch_1) {
                    if ch_1 == '\r' && characters.peek_or_zero() == '\n' {
                        characters.next();
                    }
                    current_line += 1;
                }
            } else {
                return None;
            }
        }
        Some(last_skip.offset + characters.index())
    }

    /// Retrieves the offset from the corresponding line of an offset.
    pub fn get_line_offset_from_offset(&self, offset: usize) -> usize {
        // Extra higher line skips
        let mut last_skip = HigherLineSkip { skip_index: 0, offset: 0, line_number: 1 };
        let skips = self.extra_higher_line_skips.borrow();
        let mut skips = skips.iter();
        while let Some(skip_1) = skips.next() {
            if offset < skip_1.offset {
                break;
            }
            last_skip = *skip_1;
        }

        // Higher line skips
        let skips = self.higher_line_skips.borrow();
        let mut skips = skips[last_skip.skip_index..].iter();
        let mut last_skip = skips.next().unwrap();
        while let Some(skip_1) = skips.next() {
            if offset < skip_1.offset {
                break;
            }
            last_skip = skip_1;
        }

        // Line skips
        let skips = self.line_skips.borrow();
        let mut skips = skips[last_skip.skip_index..].iter();
        let mut last_skip = skips.next().unwrap();
        while let Some(skip_1) = skips.next() {
            if offset < skip_1.offset {
                break;
            }
            last_skip = skip_1;
        }

        let mut current_line_offset = last_skip.offset;
        let mut characters = CharacterReader::from(&self.text[last_skip.offset..]);
        while last_skip.offset + characters.index() < offset {
            let ch_1 = characters.next();
            if let Some(ch_1) = ch_1 {
                if CharacterValidator::is_line_terminator(ch_1) {
                    if ch_1 == '\r' && characters.peek_or_zero() == '\n' {
                        characters.next();
                    }
                    current_line_offset = last_skip.offset + characters.index();
                }
            } else {
                break;
            }
        }
        current_line_offset
    }

    pub fn get_line_indent(&self, line: usize) -> usize {
        let line_offset = self.get_line_offset(line).unwrap();
        let indent = CharacterValidator::indent_count(&self.text[line_offset..]);
        indent - line_offset
    }
}