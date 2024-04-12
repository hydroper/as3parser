use std::rc::Rc;

/// Defines compiler options used within the parser.
/// This structure is empty in the present.
pub struct CompilerOptions {}

impl CompilerOptions {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {})
    }
}