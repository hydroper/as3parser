use std::rc::Rc;

pub struct CompilerOptions {}

impl CompilerOptions {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {})
    }
}