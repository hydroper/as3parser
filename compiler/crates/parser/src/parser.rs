use std::rc::Rc;
use crate::*;

pub struct Parser<'input> {
    tokenizer: Tokenizer<'input>,
}

impl<'input> Parser<'input> {
    /// Constructs a parser. The given `source_text` parameter must be the same
    /// as `&source.text()`.
    pub fn new(source: &Rc<Source>, source_text: &'input str) -> Self {
        Self {
            tokenizer: Tokenizer::new(source, source_text),
        }
    }
}