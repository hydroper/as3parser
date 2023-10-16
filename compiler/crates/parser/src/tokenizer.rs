use std::rc::Rc;
use crate::Source;

pub type Spanned<Token, Location, Error> = Result<(Location, Token, Location), Error>;

pub enum Token {
}

pub struct Tokenizer {
    source: Rc<Source>,
    current_line_number: usize,
}

impl Tokenizer {
    pub fn new(source: Rc<Source>) -> Self {
        assert!(!source.already_tokenized.get(), "A Source must only be tokenized once.");
        source.already_tokenized.set(true);
        Self {
            source,
            current_line_number: 1,
        }
    }
}