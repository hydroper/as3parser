use num_traits::ToPrimitive;
use std::str::FromStr;
use crate::ns::*;

pub struct CssTokenizer<'input> {
    compilation_unit: Rc<CompilationUnit>,
    characters: CharacterReader<'input>,
}

impl<'input> CssTokenizer<'input> {
    /// Constructs a tokenizer.
    pub fn new(compilation_unit: &'input Rc<CompilationUnit>, options: &ParserOptions) -> Self {
        let text: &'input str = compilation_unit.text();
        let compilation_unit = compilation_unit.clone();
        let characters: CharacterReader<'input>;
        if let Some(range) = options.byte_range {
            characters = CharacterReader::from_offset(&text[0..range.1], range.0);
        } else {
            characters = CharacterReader::from(text);
        }
        Self {
            compilation_unit,
            characters,
        }
    }

    pub fn compilation_unit(&self) -> &Rc<CompilationUnit> {
        &self.compilation_unit
    }

    pub fn characters(&self) -> &CharacterReader<'input> {
        &self.characters
    }

    fn character_ahead_location(&self) -> Location {
        if self.characters.reached_end() {
            return self.cursor_location();
        }
        let offset = self.characters.index();
        let mut next_characters = self.characters.clone();
        next_characters.next().unwrap();
        Location::with_offsets(&self.compilation_unit, offset, next_characters.index() + 1)
    }

    pub fn cursor_location(&self) -> Location {
        let offset = self.characters.index();
        Location::with_offset(&self.compilation_unit, offset)
    }

    fn add_unexpected_error(&self) {
        if self.characters.has_remaining() {
            self.compilation_unit.add_diagnostic(Diagnostic::new_syntax_error(&self.character_ahead_location(), DiagnosticKind::UnexpectedCharacter, diagnostic_arguments![String(self.characters.peek_or_zero().to_string())]))
        } else {
            self.compilation_unit.add_diagnostic(Diagnostic::new_syntax_error(&self.cursor_location(), DiagnosticKind::UnexpectedEnd, vec![]))
        }
    }

    fn add_unexpected_eof_error(&self, kind: DiagnosticKind) {
        self.compilation_unit.add_diagnostic(Diagnostic::new_syntax_error(&self.cursor_location(), kind, vec![]));
    }

    pub fn scan(&mut self) -> (Token, Location) {
        while self.consume_whitespace() || self.consume_comment() {
            // Do nothing
        }
        let start = self.cursor_location();
        let ch = self.characters.peek_or_zero();

        if let Some(id) = self.consume_css_id() {
            return (Token::Identifier(id), start.combine_with(self.cursor_location()));
        }

        // DECIMAL
        // DECIMAL.PART
        if CharacterValidator::is_dec_digit(ch) {
            self.characters.next();
            while CharacterValidator::is_dec_digit(self.characters.peek_or_zero()) {
                self.characters.next();
            }
            if self.characters.peek_or_zero() == '.' {
                self.characters.next();
                if !CharacterValidator::is_dec_digit(self.characters.peek_or_zero()) {
                    self.add_unexpected_error();
                }
                while CharacterValidator::is_dec_digit(self.characters.peek_or_zero()) {
                    self.characters.next();
                }
            }
            return self.finish_number(start);
        }

        if ch == '#' {
            self.characters.next();
            let mut word = String::new();
            loop {
                let ch = self.characters.peek_or_zero();
                if  (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') ||
                    (ch >= '0' && ch <= '9') || ch == '-' || ch == '_' {
                    word.push(ch);
                    self.characters.next();
                } else {
                    break;
                }
            }
            if word.is_empty() {
                self.add_unexpected_error();
            }
            return (Token::CssHashWord(word), start.combine_with(self.cursor_location()));
        }

        match ch {
            // .
            // .DECIMAL
            '.' => {
                self.characters.next();
                if CharacterValidator::is_dec_digit(self.characters.peek_or_zero()) {
                    while CharacterValidator::is_dec_digit(self.characters.peek_or_zero()) {
                        self.characters.next();
                    }
                    return self.finish_number(start);
                }
                (Token::Dot, start.combine_with(self.cursor_location()))
            },
            '"' | '\'' => {
                self.scan_string(ch, start)
            },
            ';' => {
                while self.characters.peek_or_zero() == ';' {
                    self.characters.next();
                }
                (Token::Semicolon, start.combine_with(self.cursor_location()))
            },
            _ => {
                if self.characters.reached_end() {
                    return (Token::Eof, start);
                }
                self.add_unexpected_error();
                self.characters.next();
                self.scan()
            },
        }
    }

    fn consume_whitespace(&mut self) -> bool {
        let ch = self.characters.peek_or_zero();
        if [' ', '\t', '\n', '\r'].contains(&ch) {
            self.characters.next();
            true
        } else {
            false
        }
    }

    fn consume_comment(&mut self) -> bool {
        if self.characters.peek_or_zero() == '/' && self.characters.peek_at_or_zero(1) == '*' {
            let start = self.cursor_location();
            self.characters.skip_count_in_place(2);
            loop {
                if self.characters.peek_or_zero() == '*' && self.characters.peek_at_or_zero(1) == '/' {
                    self.characters.skip_count_in_place(2);
                    break;
                } else if self.characters.has_remaining() {
                    self.characters.skip_in_place();
                } else {
                    self.add_unexpected_eof_error(DiagnosticKind::InputEndedBeforeReachingClosingSeqForMultiLineComment);
                    break;
                }
            }

            let location = start.combine_with(self.cursor_location());
            let i = location.first_offset() + 2;
            let j = decrease_last_offset(i, location.last_offset(), 2);

            self.compilation_unit.comments.borrow_mut().push(Rc::new(Comment {
                multiline: true,
                content: RefCell::new(self.compilation_unit.text()[i..j].to_owned()),
                location: RefCell::new(location),
            }));

            true
        } else {
            false
        }
    }

    fn consume_css_id(&mut self) -> Option<String> {
        let i = self.characters.index();
        let mut prefix_n = 0;
        if self.characters.peek_or_zero() == '_' {
            prefix_n += 1;
            if self.characters.peek_at_or_zero(prefix_n) == '_' {
                prefix_n += 1;
                if self.characters.peek_at_or_zero(prefix_n) == '_' {
                    prefix_n += 1;
                }
            }
        } else if self.characters.peek_or_zero() == '-' {
            prefix_n += 1;
        }
        if CharacterValidator::is_css_identifier_start(self.characters.peek_at_or_zero(prefix_n)) {
            self.characters.skip_count_in_place(prefix_n + 1);
            while CharacterValidator::is_css_identifier_part(self.characters.peek_or_zero()) {
                self.characters.next();
            }
            return Some(self.compilation_unit.text()[i..self.characters.index()].to_owned());
        }
        None
    }

    fn finish_number(&mut self, start: Location) -> (Token, Location) {
        let digits = &self.compilation_unit.text()[start.first_offset..self.characters.index()];
        let mut mv = f64::from_str(digits).unwrap_or(f64::NAN);
        let mut unit: Option<String> = None;
        if self.characters.peek_or_zero() == '%' {
            mv /= 100.0;
        } else {
            unit = self.consume_css_id();
        }
        (Token::CssNumber {
            value: mv,
            unit,
        }, start.combine_with(self.cursor_location()))
    }

    fn scan_string(&mut self, delim: char, start: Location) -> (Token, Location) {
        let mut builder = String::new();
        self.characters.next();
        loop {
            let ch = self.characters.peek_or_zero();
            if ch == delim {
                self.characters.next();
                break;
            } else if ch == '\\' {
                let mut loc = self.cursor_location();
                self.characters.next();
                let mut digits = String::new();
                for _ in 0..6 {
                    let ch = self.characters.peek_or_zero();
                    if CharacterValidator::is_hex_digit(ch) {
                        digits.push(ch);
                        self.characters.next();
                    } else {
                        break;
                    }
                }
                if digits.is_empty() {
                    self.add_unexpected_error();
                } else {
                    loc = loc.combine_with(self.cursor_location());
                    let mv = u32::from_str_radix(&digits, 16).ok().and_then(|mv| char::from_u32(mv));
                    if let Some(mv) = mv {
                        builder.push(mv);
                    } else {
                        self.compilation_unit.add_diagnostic(Diagnostic::new_syntax_error(&loc, DiagnosticKind::CssInvalidHexEscape, diagnostic_arguments![String(digits)]));
                    }
                }
            } else if self.characters.reached_end() {
                self.add_unexpected_eof_error(DiagnosticKind::InputEndedBeforeReachingClosingQuoteForString);
                break;
            } else {
                builder.push(ch);
                self.characters.next();
            }
        }
        let loc = start.combine_with(self.cursor_location());
        (Token::StringLiteral(builder), loc)
    }
}

fn _rgb_bytes_to_integer(r: f64, g: f64, b: f64) -> u32 {
    (_calc_rgb_byte(r) << 16) | (_calc_rgb_byte(g) << 8) | _calc_rgb_byte(b)
}

fn _calc_rgb_byte(value: f64) -> u32 {
    // Integer
    if value.round() == value {
        value.round().to_u32().unwrap_or(0).clamp(0, 255)
    // Float
    } else {
        (value * 255.0).round().to_u32().unwrap_or(0).clamp(0, 255)
    }
}