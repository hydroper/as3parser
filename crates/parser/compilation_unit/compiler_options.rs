use std::rc::Rc;

/// Defines compiler options used within the parser.
pub struct CompilerOptions {}

impl CompilerOptions {
    pub fn default() -> Rc<Self> {
        Rc::new(Self {})
    }
}