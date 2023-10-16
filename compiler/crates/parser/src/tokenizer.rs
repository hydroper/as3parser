use std::rc::Rc;
use crate::{Source, util::CodePointsReader};

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
    pub fn new(source: Rc<Source>, source_text: &'input str) -> Self {
        assert!(!source.already_tokenized.get(), "A Source must only be tokenized once.");
        source.already_tokenized.set(true);
        Self {
            source,
            current_line_number: 1,
            code_points: CodePointsReader::from(source_text),
        }
    }
}