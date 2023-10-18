use std::rc::Rc;
use crate::{Source, util::CodePointsReader, IntolerableError, Location, character_validation};

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

    pub fn next(&mut self, reserved_words: bool) -> Result<(Token, Location), IntolerableError> {
        loop {
            let ch = self.code_points.peek_or_zero();
            if character_validation::is_whitespace(ch) {
                self.code_points.next();
            } else if self.consume_line_terminator() || self.consume_comment() {
                // Consumed line terminator or comment
            } else {
                break;
            }
        }
        let start = self.current_character_location();
        final_result_here
    }

    pub fn current_line_number(&self) -> usize {
        self.current_line_number
    }

    pub fn current_character_location(&self) -> Location {
        let offset = self.code_points.index();
        Location {
            source: Rc::clone(&self.source),
            first_line_number: self.current_line_number,
            last_line_number: self.current_line_number,
            first_offset: offset,
            last_offset: offset,
        }
    }

    // LineTerminator
    fn consume_line_terminator(&mut self) -> bool {
        let ch = self.code_points.peek_or_zero();
        if ch == '\x0D' && self.code_points.peek_at_or_zero(1) == '\x0A' {
            self.code_points.next();
            self.code_points.next();
            self.source.line_number_offsets.borrow_mut().push(self.code_points.index());
            self.current_line_number += 1;
            return true;
        }
        if character_validation::is_line_terminator(ch) {
            self.source.line_number_offsets.borrow_mut().push(self.code_points.index());
            self.current_line_number += 1;
            return true;
        }
        false
    }

    fn consume_comment(&mut self) -> bool {
        let ch = self.code_points.peek_or_zero();
        if ch != '/' {
            return false;
        }
        let ch2 = self.code_points.peek_at_or_zero(1);
        if ch == '*' {}
    }
}