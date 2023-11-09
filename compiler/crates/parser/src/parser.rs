use std::rc::Rc;
use crate::*;

pub struct Parser<'input> {
    tokenizer: Tokenizer<'input>,
}

impl<'input> Parser<'input> {
    /// Constructs a parser.
    pub fn new(source: &'input Rc<Source>) -> Self {
        Self {
            tokenizer: Tokenizer::new(source),
        }
    }
}