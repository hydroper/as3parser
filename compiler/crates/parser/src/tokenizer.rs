use std::rc::Rc;
use crate::{Source, util::CodePointsReader, IntolerableError};

#[derive(Copy, Clone, PartialEq)]
pub enum Token {
    Eof,
}

pub struct Tokenizer<'input> {
    source: Rc<Source>,
    current_line_number: usize,
    code_points: CodePointsReader<'input>,
}

impl<'input> Tokenizer<'input> {
    pub fn new(source: &Rc<Source>, source_text: &'input str) -> Self {
        let source = Rc::clone(source);
        assert!(!source.already_tokenized.get(), "A Source must only be tokenized once.");
        source.already_tokenized.set(true);
        Self {
            source,
            current_line_number: 1,
            code_points: CodePointsReader::from(source_text),
        }
    }

    pub fn next(&mut self, reserved_words: bool) -> Result<Token, IntolerableError> {
        loop {
            let ch = self.code_points.peek_or_zero();
        }
        final_result_here
    }
}