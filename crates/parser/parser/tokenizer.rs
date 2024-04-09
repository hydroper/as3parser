use crate::ns::*;

pub struct Tokenizer<'input> {
    compilation_unit: Rc<CompilationUnit>,
    line_number: usize,
    characters: CharacterReader<'input>,
}

impl<'input> Tokenizer<'input> {
    /// Constructs a tokenizer.
    pub fn new(compilation_unit: &'input Rc<CompilationUnit>) -> Self {
        let text: &'input str = compilation_unit.text();
        let compilation_unit = compilation_unit.clone();
        assert!(!compilation_unit.already_tokenized.get(), "A CompilationUnit must be tokenized at most once.");
        compilation_unit.already_tokenized.set(true);
        Self {
            compilation_unit,
            line_number: 1,
            characters: CharacterReader::from(text),
        }
    }

    pub fn compilation_unit(&self) -> &Rc<CompilationUnit> {
        &self.compilation_unit
    }

    /// Scans for an *InputElementDiv* token.
    pub fn scan_ie_div(&mut self) -> Result<(Token, Location), ParsingFailure> {
        loop {
            let ch = self.characters.peek_or_zero();
            if CharacterValidator::is_whitespace(ch) {
                self.characters.next();
            } else if self.consume_line_terminator() || self.consume_comment()? {
                // Consumed line terminator or comment
            } else {
                break;
            }
        }
        if let Some(result) = self.scan_identifier()? {
            return Ok(result);
        }
        if let Some(result) = self.scan_dot_or_numeric_literal()? {
            return Ok(result);
        }
        if let Some(result) = self.scan_string_literal(false)? {
            return Ok(result);
        }
        let start = self.cursor_location();
        match self.characters.peek_or_zero() {
            ',' => {
                // Comma
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                return Ok((Token::Comma, location));
            },
            '(' => {
                // LeftParen
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                return Ok((Token::LeftParen, location));
            },
            ')' => {
                // RightParen
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                return Ok((Token::RightParen, location));
            },
            '[' => {
                // LeftBracket
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                return Ok((Token::LeftBracket, location));
            },
            ']' => {
                // RightBracket
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                return Ok((Token::RightBracket, location));
            },
            '{' => {
                // LeftBrace
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                return Ok((Token::LeftBrace, location));
            },
            '}' => {
                // RightBrace
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                return Ok((Token::RightBrace, location));
            },
            ':' => {
                self.characters.next();
                // ColonColon
                if self.characters.peek_or_zero() == ':' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::ColonColon, location));
                }
                // Colon
                let location = start.combine_with(self.cursor_location());
                return Ok((Token::Colon, location));
            },
            '=' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // StrictEquals
                if ch == '=' && self.characters.peek_at_or_zero(1) == '=' {
                    self.characters.skip_count_in_place(2);
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::StrictEquals, location));
                }
                // Equals
                if ch == '=' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::Equals, location));
                }
                // Assign
                let location = start.combine_with(self.cursor_location());
                return Ok((Token::Assign, location));
            },
            '!' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // StrictNotEquals
                if ch == '=' && self.characters.peek_at_or_zero(1) == '=' {
                    self.characters.skip_count_in_place(2);
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::StrictNotEquals, location));
                }
                // NotEquals
                if ch == '=' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::NotEquals, location));
                }
                // Exclamation
                let location = start.combine_with(self.cursor_location());
                return Ok((Token::Exclamation, location));
            },
            '?' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // OptionalChaining
                if ch == '.' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::OptionalChaining, location));
                }
                // NullCoalescingAssign
                if ch == '?' && self.characters.peek_at_or_zero(1) == '=' {
                    self.characters.skip_count_in_place(2);
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::NullCoalescingAssign, location));
                }
                // NullCoalescing
                if ch == '?' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::NullCoalescing, location));
                }
                // Question
                let location = start.combine_with(self.cursor_location());
                return Ok((Token::Question, location));
            },
            ';' => {
                // Semicolon
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                return Ok((Token::Semicolon, location));
            },
            '<' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // Le
                if ch == '=' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::Le, location));
                }
                // LeftShiftAssign
                if ch == '<' && self.characters.peek_at_or_zero(1) == '=' {
                    self.characters.skip_count_in_place(2);
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::LeftShiftAssign, location));
                }
                // LeftShift
                if ch == '<' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::LeftShift, location));
                }
                // Lt
                let location = start.combine_with(self.cursor_location());
                return Ok((Token::Lt, location));
            },
            '>' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // Ge
                if ch == '=' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::Ge, location));
                }
                // RightShiftAssign
                if ch == '>' && self.characters.peek_at_or_zero(1) == '=' {
                    self.characters.skip_count_in_place(2);
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::RightShiftAssign, location));
                }
                // UnsignedRightShiftAssign
                if ch == '>' && self.characters.peek_seq(3) == ">>=" {
                    self.characters.skip_count_in_place(3);
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::UnsignedRightShiftAssign, location));
                }
                // UnsignedRightShift
                if ch == '>' && self.characters.peek_at_or_zero(1) == '>' {
                    self.characters.skip_count_in_place(2);
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::UnsignedRightShift, location));
                }
                // RightShift
                if ch == '<' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::RightShift, location));
                }
                // Gt
                let location = start.combine_with(self.cursor_location());
                return Ok((Token::Gt, location));
            },
            '@' => {
                // Attribute
                self.characters.next();
                if let Some(token) = self.scan_string_literal(true)? {
                    return Ok(token);
                }
                let location = start.combine_with(self.cursor_location());
                return Ok((Token::Attribute, location));
            },
            '+' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // Increment
                if ch == '+' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::Increment, location));
                }
                // AddAssign
                if ch == '=' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::AddAssign, location));
                }
                // Plus
                let location = start.combine_with(self.cursor_location());
                return Ok((Token::Plus, location));
            },
            '-' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // Decrement
                if ch == '-' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::Decrement, location));
                }
                // SubtractAssign
                if ch == '=' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::SubtractAssign, location));
                }
                // Minus
                let location = start.combine_with(self.cursor_location());
                return Ok((Token::Minus, location));
            },
            '*' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // PowerAssign
                if ch == '*' && self.characters.peek_at_or_zero(1) == '=' {
                    self.characters.skip_count_in_place(2);
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::PowerAssign, location));
                }
                // Power
                if ch == '*' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::Power, location));
                }
                // MultiplyAssign
                if ch == '=' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::MultiplyAssign, location));
                }
                // Times
                let location = start.combine_with(self.cursor_location());
                return Ok((Token::Times, location));
            },
            '/' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // DivideAssign
                if ch == '=' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::DivideAssign, location));
                }
                // Div
                let location = start.combine_with(self.cursor_location());
                return Ok((Token::Div, location));
            },
            '%' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // RemainderAssign
                if ch == '=' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::RemainderAssign, location));
                }
                // Remainder
                let location = start.combine_with(self.cursor_location());
                return Ok((Token::Remainder, location));
            },
            '&' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // LogicalAndAssign
                if ch == '&' && self.characters.peek_at_or_zero(1) == '=' {
                    self.characters.skip_count_in_place(2);
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::LogicalAndAssign, location));
                }
                // LogicalAnd
                if ch == '&' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::LogicalAnd, location));
                }
                // BitwiseAndAssign
                if ch == '=' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::BitwiseAndAssign, location));
                }
                // BitwiseAnd
                let location = start.combine_with(self.cursor_location());
                return Ok((Token::BitwiseAnd, location));
            },
            '^' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // LogicalXorAssign
                if ch == '^' && self.characters.peek_at_or_zero(1) == '=' {
                    self.characters.skip_count_in_place(2);
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::LogicalXorAssign, location));
                }
                // LogicalXor
                if ch == '^' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::LogicalXor, location));
                }
                // BitwiseXorAssign
                if ch == '=' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::BitwiseXorAssign, location));
                }
                // BitwiseXor
                let location = start.combine_with(self.cursor_location());
                return Ok((Token::BitwiseXor, location));
            },
            '|' => {
                self.characters.next();
                let ch = self.characters.peek_or_zero();
                // LogicalOrAssign
                if ch == '|' && self.characters.peek_at_or_zero(1) == '=' {
                    self.characters.skip_count_in_place(2);
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::LogicalOrAssign, location));
                }
                // LogicalOr
                if ch == '|' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::LogicalOr, location));
                }
                // BitwiseOrAssign
                if ch == '=' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::BitwiseOrAssign, location));
                }
                // BitwiseOr
                let location = start.combine_with(self.cursor_location());
                return Ok((Token::BitwiseOr, location));
            },
            '~' => {
                // BitwiseNot
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                return Ok((Token::BitwiseNot, location));
            },
            _ => {
                if self.characters.has_remaining() {
                    self.add_unexpected_error();
                    self.characters.next();
                    return self.scan_ie_div();
                // Eof
                } else {
                    return Ok((Token::Eof, start))
                }
            },
        }
    }

    /// Scans regular expression after a `/` or `/=` token has been scanned by
    /// `scan_ie_div`.
    pub fn scan_regexp_literal(&mut self, start: Location) -> Result<(Token, Location), ParsingFailure> {
        let mut body = String::new();
        loop {
            let ch = self.characters.peek_or_zero();
            if ch == '/' {
                self.characters.next();
                break;
            } else if ch == '\\' {
                self.characters.next();
                body.push('\\');
                let ch = self.characters.peek_or_zero();
                if self.characters.reached_end() {
                    self.add_unexpected_error();
                    return Err(ParsingFailure);
                } else if CharacterValidator::is_line_terminator(ch) {
                    self.add_unexpected_error();
                }
                self.consume_line_terminator();
                body.push(ch);
            } else if CharacterValidator::is_line_terminator(ch) {
                body.push('\n');
                self.consume_line_terminator();
            } else if self.characters.reached_end() {
                self.add_unexpected_error();
                return Err(ParsingFailure);
            } else {
                body.push(ch);
                self.characters.next();
            }
        }

        let mut flags = String::new();
        while let Some((ch, _)) = self.consume_identifier_part()? {
            flags.push(ch);
        }
        
        let location = start.combine_with(self.cursor_location());
        Ok((Token::RegExpLiteral { body, flags }, location))
    }

    /// Indicates the current line number, counted from 1 (one).
    pub fn line_number(&self) -> usize {
        self.line_number
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
            self.compilation_unit.add_diagnostic(Diagnostic::new_syntax_error(&self.character_ahead_location(), DiagnosticKind::UnexpectedOrInvalidToken, vec![]))
        } else {
            self.compilation_unit.add_diagnostic(Diagnostic::new_syntax_error(&self.cursor_location(), DiagnosticKind::UnexpectedEnd, vec![]))
        }
    }

    // LineTerminator
    fn consume_line_terminator(&mut self) -> bool {
        let ch = self.characters.peek_or_zero();
        if ch == '\x0D' && self.characters.peek_at_or_zero(1) == '\x0A' {
            self.characters.skip_count_in_place(2);
            self.line_number += 1;
            return true;
        }
        if CharacterValidator::is_line_terminator(ch) {
            self.characters.next();
            self.line_number += 1;
            return true;
        }
        false
    }

    fn consume_comment(&mut self) -> Result<bool, ParsingFailure> {
        let ch = self.characters.peek_or_zero();
        if ch != '/' {
            return Ok(false);
        }
        let ch2 = self.characters.peek_at_or_zero(1);
        if ch2 == '/' {
            let start = self.cursor_location();
            self.characters.skip_count_in_place(2);
            while !CharacterValidator::is_line_terminator(self.characters.peek_or_zero()) && self.characters.has_remaining() {
                self.characters.skip_in_place();
            }
            let location = start.combine_with(self.cursor_location());
            self.consume_line_terminator();

            self.compilation_unit.comments.borrow_mut().push(Rc::new(Comment {
                multiline: false,
                content: RefCell::new(self.compilation_unit.text()[(location.first_offset() + 2)..location.last_offset()].to_owned()),
                location: RefCell::new(location),
            }));

            return Ok(true);
        }
        if ch2 == '*' {
            let start = self.cursor_location();
            self.characters.skip_count_in_place(2);

            loop {
                if self.characters.peek_or_zero() == '*' && self.characters.peek_at_or_zero(1) == '/' {
                    self.characters.skip_count_in_place(2);
                    break;
                } else if self.consume_line_terminator() {
                    // Consumed LineTerminator
                } else if self.characters.has_remaining() {
                    self.characters.skip_in_place();
                } else {
                    self.add_unexpected_error();
                    return Err(ParsingFailure);
                }
            }

            let location = start.combine_with(self.cursor_location());

            self.compilation_unit.comments.borrow_mut().push(Rc::new(Comment {
                multiline: true,
                content: RefCell::new(self.compilation_unit.text()[(location.first_offset() + 2)..(location.last_offset() - 2)].to_owned()),
                location: RefCell::new(location),
            }));

            return Ok(true);
        }
        Ok(false)
    }

    fn scan_identifier(&mut self) -> Result<Option<(Token, Location)>, ParsingFailure> {
        let start = self.cursor_location();
        let mut escaped = false;
        let Some((ch, escaped_2)) = self.consume_identifier_start()? else {
            return Ok(None);
        };
        escaped = escaped || escaped_2;
        let mut name = String::new();
        name.push(ch);
        while let Some((ch, escaped_2)) = self.consume_identifier_part()? {
            escaped = escaped || escaped_2;
            name.push(ch);
        }

        let location = start.combine_with(self.cursor_location());
        if !escaped {
            if let Some(token) = As3ReservedWord::token(name.as_ref()) {
                return Ok(Some((token, location)));
            }
        }
        Ok(Some((Token::Identifier(name), location)))
    }

    /// Returns a tuple in the form (*character*, *escaped*).
    fn consume_identifier_start(&mut self) -> Result<Option<(char, bool)>, ParsingFailure> {
        let ch = self.characters.peek_or_zero();
        if CharacterValidator::is_identifier_start(ch) {
            self.characters.next();
            return Ok(Some((ch, false)));
        }
        if self.characters.peek_or_zero() == '\\' {
            self.characters.next();
            return Ok(Some((self.expect_unicode_escape_sequence()?, true)));
        }
        Ok(None)
    }

    /// Returns a tuple in the form (*character*, *escaped*).
    fn consume_identifier_part(&mut self) -> Result<Option<(char, bool)>, ParsingFailure> {
        let ch = self.characters.peek_or_zero();
        if CharacterValidator::is_identifier_part(ch) {
            self.characters.next();
            return Ok(Some((ch, false)));
        }
        if self.characters.peek_or_zero() == '\\' {
            self.characters.next();
            return Ok(Some((self.expect_unicode_escape_sequence()?, true)));
        }
        Ok(None)
    }

    /// Expects UnicodeEscapeSequence starting from `u`.
    fn expect_unicode_escape_sequence(&mut self) -> Result<char, ParsingFailure> {
        let start = self.cursor_location();
        if self.characters.peek_or_zero() != 'u' {
            self.add_unexpected_error();
            return Ok('\x5F');
        }
        self.characters.next();

        // Scan \uXXXX
        if CharacterValidator::is_hex_digit(self.characters.peek_or_zero()) {
            let r = char::from_u32(self.expect_hex_digit()? << 12
                | (self.expect_hex_digit()? << 8)
                | (self.expect_hex_digit()? << 4)
                | self.expect_hex_digit()?);
            let Some(r) = r else {
                self.compilation_unit.add_diagnostic(Diagnostic::new_syntax_error(&start.combine_with(self.cursor_location()), DiagnosticKind::UnexpectedOrInvalidToken, vec![]));
                return Ok('\x5F');
            };
            return Ok(r);
        }

        // Scan \u{}
        if self.characters.peek_or_zero() != '{' {
            self.add_unexpected_error();
            return Ok('\x5F');
        }
        self.characters.next();
        while CharacterValidator::is_hex_digit(self.characters.peek_or_zero()) {
            self.characters.next();
        }
        if self.characters.peek_or_zero() != '}' {
            self.add_unexpected_error();
            return Ok('\x5F');
        }
        self.characters.next();
        let location = start.combine_with(self.cursor_location());
        let r = u32::from_str_radix(&self.compilation_unit.text()[(start.first_offset + 2)..(location.last_offset - 1)], 16);
        let Ok(r) = r else {
            self.compilation_unit.add_diagnostic(Diagnostic::new_syntax_error(&location, DiagnosticKind::UnexpectedOrInvalidToken, vec![]));
            return Ok('\x5F');
        };
        let r = char::from_u32(r);
        let Some(r) = r else {
            self.compilation_unit.add_diagnostic(Diagnostic::new_syntax_error(&location, DiagnosticKind::UnexpectedOrInvalidToken, vec![]));
            return Ok('\x5F');
        };
        Ok(r)
    }

    fn expect_hex_digit(&mut self) -> Result<u32, ParsingFailure> {
        let ch = self.characters.peek_or_zero();
        let mv = CharacterValidator::hex_digit_mv(ch);
        if mv.is_none() {
            self.add_unexpected_error();
            return Ok(0x5F);
        }
        self.characters.next();
        Ok(mv.unwrap())
    }

    fn scan_dot_or_numeric_literal(&mut self) -> Result<Option<(Token, Location)>, ParsingFailure> {
        let start = self.cursor_location();
        let ch = self.characters.peek_or_zero();
        let mut initial_dot = false;
        if ch == '.' {
            initial_dot = true;
            self.characters.next();

            let seq = self.characters.peek_seq(2);
            // Ellipsis
            if seq == ".." {
                self.characters.skip_count_in_place(2);
                return Ok(Some((Token::Ellipsis, start.combine_with(self.cursor_location()))));
            }
            let ch = seq.get(..1).map(|ch| ch.chars().next().unwrap()).unwrap_or('\x00');
            // Descendants
            if ch == '.' {
                self.characters.next();
                return Ok(Some((Token::Descendants, start.combine_with(self.cursor_location()))));
            }
            // Dot
            if !CharacterValidator::is_dec_digit(ch) {
                return Ok(Some((Token::Dot, start.combine_with(self.cursor_location()))));
            }

            // NumericLiteral
            while CharacterValidator::is_dec_digit(self.characters.peek_or_zero()) {
                self.characters.next();
                self.consume_underscore_followed_by_dec_digit()?;
            }
        } else if ch == '0' {
            self.characters.next();
            let ch_2 = self.characters.peek_or_zero();

            // HexLiteral
            if ['X', 'x'].contains(&ch_2) {
                self.characters.next();
                return self.scan_hex_literal(start.clone());
            }

            // BinLiteral;
            if ['B', 'b'].contains(&ch_2) {
                self.characters.next();
                return self.scan_bin_literal(start.clone());
            }
        } else if CharacterValidator::is_dec_digit(ch) {
            while CharacterValidator::is_dec_digit(self.characters.peek_or_zero()) {
                self.characters.next();
                self.consume_underscore_followed_by_dec_digit()?;
            }
        } else {
            return Ok(None);
        }

        if !initial_dot && self.characters.peek_or_zero() == '.' {
            self.characters.next();
            if !CharacterValidator::is_dec_digit(self.characters.peek_or_zero()) {
                self.add_unexpected_error();
            }
            while CharacterValidator::is_dec_digit(self.characters.peek_or_zero()) {
                self.characters.next();
                self.consume_underscore_followed_by_dec_digit()?;
            }
        }

        // Decimal exponent
        if ['E', 'e'].contains(&self.characters.peek_or_zero()) {
            self.characters.next();
            if ['+', '-'].contains(&self.characters.peek_or_zero()) {
                self.characters.next();
            }
            if !CharacterValidator::is_dec_digit(self.characters.peek_or_zero()) {
                self.add_unexpected_error();
            }
            while CharacterValidator::is_dec_digit(self.characters.peek_or_zero()) {
                self.characters.next();
                self.consume_underscore_followed_by_dec_digit()?;
            }
        }

        let string = self.compilation_unit.text()[start.first_offset..self.characters.index()].to_owned();

        let mut suffix = NumberSuffix::None;
        if self.characters.peek_or_zero() == 'f' || self.characters.peek_or_zero() == 'F' {
            suffix = NumberSuffix::F;
            self.characters.next();
        }
        self.unallow_numeric_suffix();

        let location = start.combine_with(self.cursor_location());

        Ok(Some((Token::NumericLiteral(string, suffix), location)))
    }

    fn scan_hex_literal(&mut self, start: Location) -> Result<Option<(Token, Location)>, ParsingFailure> {
        if !CharacterValidator::is_hex_digit(self.characters.peek_or_zero()) {
            self.add_unexpected_error();
        }
        while CharacterValidator::is_hex_digit(self.characters.peek_or_zero()) {
            self.characters.next();
            self.consume_underscore_followed_by_hex_digit()?;
        }

        let suffix = NumberSuffix::None;
        self.unallow_numeric_suffix();

        let location = start.combine_with(self.cursor_location());
        let s = self.compilation_unit.text()[location.first_offset..location.last_offset].to_owned();
        Ok(Some((Token::NumericLiteral(s, suffix), location)))
    }

    fn scan_bin_literal(&mut self, start: Location) -> Result<Option<(Token, Location)>, ParsingFailure> {
        if !CharacterValidator::is_bin_digit(self.characters.peek_or_zero()) {
            self.add_unexpected_error();
        }
        while CharacterValidator::is_bin_digit(self.characters.peek_or_zero()) {
            self.characters.next();
            self.consume_underscore_followed_by_bin_digit()?;
        }

        let suffix = NumberSuffix::None;
        self.unallow_numeric_suffix();

        let location = start.combine_with(self.cursor_location());
        let s = self.compilation_unit.text()[location.first_offset..location.last_offset].to_owned();
        Ok(Some((Token::NumericLiteral(s, suffix), location)))
    }

    fn consume_underscore_followed_by_dec_digit(&mut self) -> Result<(), ParsingFailure> {
        if self.characters.peek_or_zero() == '_' {
            self.characters.next();
            if !CharacterValidator::is_dec_digit(self.characters.peek_or_zero()) {
                self.add_unexpected_error();
            }
            self.characters.next();
        }
        Ok(())
    }

    fn consume_underscore_followed_by_hex_digit(&mut self) -> Result<(), ParsingFailure> {
        if self.characters.peek_or_zero() == '_' {
            self.characters.next();
            if !CharacterValidator::is_hex_digit(self.characters.peek_or_zero()) {
                self.add_unexpected_error();
            }
            self.characters.next();
        }
        Ok(())
    }

    fn consume_underscore_followed_by_bin_digit(&mut self) -> Result<(), ParsingFailure> {
        if self.characters.peek_or_zero() == '_' {
            self.characters.next();
            if !CharacterValidator::is_bin_digit(self.characters.peek_or_zero()) {
                self.add_unexpected_error();
            }
            self.characters.next();
        }
        Ok(())
    }

    fn unallow_numeric_suffix(&self) {
        if CharacterValidator::is_identifier_start(self.characters.peek_or_zero()) {
            self.add_unexpected_error();
        }
    }

    fn scan_string_literal(&mut self, raw: bool) -> Result<Option<(Token, Location)>, ParsingFailure> {
        let delim = self.characters.peek_or_zero();
        if !['"', '\''].contains(&delim) {
            return Ok(None);
        }
        let start = self.cursor_location();
        self.characters.next();

        // Triple string literal
        if self.characters.peek_or_zero() == delim && self.characters.peek_at_or_zero(1) == delim {
            self.characters.skip_count_in_place(2);
            return self.scan_triple_string_literal(delim, start, raw);
        }

        let mut value = String::new();

        if raw {
            loop {
                let ch = self.characters.peek_or_zero();
                if ch == delim {
                    self.characters.next();
                    break;
                } else if CharacterValidator::is_line_terminator(ch) {
                    self.compilation_unit.add_diagnostic(Diagnostic::new_syntax_error(&self.character_ahead_location(), DiagnosticKind::UnallowedLineBreak, vec![]));
                    self.consume_line_terminator();
                } else if !self.characters.has_remaining() {
                    self.add_unexpected_error();
                    return Err(ParsingFailure);
                } else {
                    value.push(ch);
                    self.characters.next();
                }
            }
        } else {
            loop {
                if let Some(s) = self.consume_escape_sequence()? {
                    value.push_str(&s);
                } else {
                    let ch = self.characters.peek_or_zero();
                    if ch == delim {
                        self.characters.next();
                        break;
                    } else if CharacterValidator::is_line_terminator(ch) {
                        self.compilation_unit.add_diagnostic(Diagnostic::new_syntax_error(&self.character_ahead_location(), DiagnosticKind::UnallowedLineBreak, vec![]));
                        self.consume_line_terminator();
                    } else if !self.characters.has_remaining() {
                        self.add_unexpected_error();
                        return Err(ParsingFailure);
                    } else {
                        value.push(ch);
                        self.characters.next();
                    }
                }
            }
        }

        let location = start.combine_with(self.cursor_location());
        Ok(Some((Token::StringLiteral(value), location)))
    }

    fn scan_triple_string_literal(&mut self, delim: char, start: Location, raw: bool) -> Result<Option<(Token, Location)>, ParsingFailure> {
        let mut lines: Vec<String> = vec![];
        let mut builder = String::new();

        if raw {
            loop {
                let ch = self.characters.peek_or_zero();
                if ch == delim && self.characters.peek_at_or_zero(1) == delim && self.characters.peek_at_or_zero(2) == delim {
                    self.characters.skip_count_in_place(3);
                    lines.push(builder.clone());
                    break;
                } else if CharacterValidator::is_line_terminator(ch) {
                    lines.push(builder.clone());
                    builder.clear();
                    self.consume_line_terminator();
                } else if !self.characters.has_remaining() {
                    self.add_unexpected_error();
                    return Err(ParsingFailure);
                } else {
                    builder.push(ch);
                    self.characters.next();
                }
            }
        } else {
            loop {
                if let Some(s) = self.consume_escape_sequence()? {
                    builder.push_str(&s);
                } else {
                    let ch = self.characters.peek_or_zero();
                    if ch == delim && self.characters.peek_at_or_zero(1) == delim && self.characters.peek_at_or_zero(2) == delim {
                        self.characters.skip_count_in_place(3);
                        lines.push(builder.clone());
                        break;
                    } else if CharacterValidator::is_line_terminator(ch) {
                        lines.push(builder.clone());
                        builder.clear();
                        self.consume_line_terminator();
                    } else if !self.characters.has_remaining() {
                        self.add_unexpected_error();
                        return Err(ParsingFailure);
                    } else {
                        builder.push(ch);
                        self.characters.next();
                    }
                }
            }
        }

        let location = start.combine_with(self.cursor_location());

        if lines[0].is_empty() {
            lines.remove(0);
        }

        let last_line = lines.pop().unwrap();

        let base_indent = CharacterValidator::indent_count(&last_line);

        let mut lines: Vec<String> = lines.iter().map(|line| {
            let indent = CharacterValidator::indent_count(line);
            line[usize::min(base_indent, indent)..].to_owned()
        }).collect();

        let last_line = last_line[base_indent..].to_owned();
        if !last_line.is_empty() {
            lines.push(last_line);
        }

        let value = lines.join("\n");
        Ok(Some((Token::StringLiteral(value), location)))
    }

    fn consume_escape_sequence(&mut self) -> Result<Option<String>, ParsingFailure> {
        if self.characters.peek_or_zero() != '\\' {
            return Ok(None);
        }
        self.characters.next();
        if !self.characters.has_remaining() {
            self.add_unexpected_error();
            return Err(ParsingFailure);
        }
        if self.consume_line_terminator() {
            return Ok(Some("".into()));
        }
        let ch = self.characters.peek_or_zero();
        match ch {
            '\'' | '"' | '\\' => {
                self.characters.next();
                Ok(Some(ch.into()))
            },
            'u' => {
                Ok(Some(self.expect_unicode_escape_sequence()?.into()))
            },
            'x' => {
                self.characters.next();
                let v = (self.expect_hex_digit()? << 4) | self.expect_hex_digit()?;
                let v = char::from_u32(v).unwrap();
                Ok(Some(v.into()))
            },
            'b' => {
                self.characters.next();
                Ok(Some('\x08'.into()))
            },
            'f' => {
                self.characters.next();
                Ok(Some('\x0C'.into()))
            },
            'n' => {
                self.characters.next();
                Ok(Some('\x0A'.into()))
            },
            'r' => {
                self.characters.next();
                Ok(Some('\x0D'.into()))
            },
            't' => {
                self.characters.next();
                Ok(Some('\x09'.into()))
            },
            'v' => {
                self.characters.next();
                Ok(Some('\x0B'.into()))
            },
            '0' => {
                self.characters.next();
                if CharacterValidator::is_dec_digit(self.characters.peek_or_zero()) {
                    self.add_unexpected_error();
                }
                Ok(Some('\x00'.into()))
            },
            ch => {
                if CharacterValidator::is_dec_digit(ch) {
                    self.add_unexpected_error();
                }
                self.characters.next();
                Ok(Some(ch.into()))
            },
        }
    }

    /// Scans for an *InputElementXMLTag* token.
    pub fn scan_ie_xml_tag(&mut self) -> Result<(Token, Location), ParsingFailure> {
        let start = self.cursor_location();
        let ch = self.characters.peek_or_zero();

        // XmlName
        if CharacterValidator::is_xml_name_start(ch) {
            self.characters.next();
            while CharacterValidator::is_xml_name_part(self.characters.peek_or_zero()) {
                self.characters.next();
            }
            let location = start.combine_with(self.cursor_location());
            let name = self.compilation_unit.text()[location.first_offset..location.last_offset].to_owned();
            return Ok((Token::XmlName(name), location));
        }

        // XmlWhitespace
        if CharacterValidator::is_xml_whitespace(ch) {
            while CharacterValidator::is_xml_whitespace(self.characters.peek_or_zero()) {
                if !self.consume_line_terminator() {
                    self.characters.next();
                }
            }
            let location = start.combine_with(self.cursor_location());
            return Ok((Token::XmlWhitespace, location));
        }

        match ch {
            // Assign
            '=' => {
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                Ok((Token::Assign, location))
            },

            // Gt
            '>' => {
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                Ok((Token::Gt, location))
            },

            // XmlSlashGt
            '/' => {
                self.characters.next();
                if self.characters.peek_or_zero() != '>' {
                    self.add_unexpected_error();
                    while self.characters.has_remaining() {
                        self.characters.next();
                        if self.characters.peek_or_zero() == '>' {
                            self.characters.next();
                            let location = start.combine_with(self.cursor_location());
                            return Ok((Token::XmlSlashGt, location));
                        }
                    }
                    return Err(ParsingFailure);
                }
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                Ok((Token::XmlSlashGt, location))
            },

            // XmlAttributeValue
            '"' | '\'' => {
                let delim = ch;
                self.characters.next();
                while self.characters.peek_or_zero() != delim && self.characters.has_remaining() {
                    if !self.consume_line_terminator() {
                        self.characters.next();
                    }
                }
                if self.characters.reached_end() {
                    self.add_unexpected_error();
                    return Err(ParsingFailure)
                }
                let value = self.compilation_unit.text()[(start.first_offset + 1)..self.cursor_location().first_offset].to_owned();
                self.characters.next();
                
                let location = start.combine_with(self.cursor_location());
                Ok((Token::XmlAttributeValue(value), location))
            },

            // LeftBrace
            '{' => {
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                Ok((Token::LeftBrace, location))
            },

            _ => {
                self.add_unexpected_error();
                self.characters.next();
                self.scan_ie_xml_tag()
            },
        }
    }

    /// Scans for an *InputElementXMLContent* token.
    pub fn scan_ie_xml_content(&mut self) -> Result<(Token, Location), ParsingFailure> {
        let start = self.cursor_location();
        let ch = self.characters.peek_or_zero();

        match ch {
            '<' => {
                self.characters.next();

                // XmlMarkup
                if let Some(r) = self.scan_xml_markup(start.clone())? {
                    return Ok(r);
                }

                // XmlLtSlash
                if self.characters.peek_or_zero() == '/' {
                    self.characters.next();
                    let location = start.combine_with(self.cursor_location());
                    return Ok((Token::XmlLtSlash, location));
                }

                // Lt
                let location = start.combine_with(self.cursor_location());
                Ok((Token::Lt, location))
            },
            
            // LeftBrace
            '{' => {
                self.characters.next();
                let location = start.combine_with(self.cursor_location());
                Ok((Token::LeftBrace, location))
            },

            // XmlName
            _ => {
                loop {
                    let ch = self.characters.peek_or_zero();
                    if ['<', '{'].contains(&ch) {
                        break;
                    }
                    if CharacterValidator::is_line_terminator(ch) {
                        self.consume_line_terminator();
                    } else if self.characters.reached_end() {
                        self.add_unexpected_error();
                        return Err(ParsingFailure);
                    } else {
                        self.characters.next();
                    }
                }

                let location = start.combine_with(self.cursor_location());
                let content = self.compilation_unit.text()[location.first_offset..location.last_offset].to_owned();
                Ok((Token::XmlText(content), location))
            },
        }
    }

    /// Attempts to scan a XMLMarkup token after a `<` character.
    pub fn scan_xml_markup(&mut self, start: Location) -> Result<Option<(Token, Location)>, ParsingFailure> {
        // XMLComment
        if self.characters.peek_seq(3) == "!--" {
            self.characters.skip_count_in_place(3);
            loop {
                if self.characters.peek_or_zero() == '-' && self.characters.peek_seq(3) == "-->" {
                    self.characters.skip_count_in_place(3);
                    break;
                } else if CharacterValidator::is_line_terminator(self.characters.peek_or_zero()) {
                    self.consume_line_terminator();
                } else if self.characters.reached_end() {
                    self.add_unexpected_error();
                    return Err(ParsingFailure);
                } else {
                    self.characters.next();
                }
            }

            let location = start.combine_with(self.cursor_location());
            let content = self.compilation_unit.text()[location.first_offset..location.last_offset].to_owned();

            return Ok(Some((Token::XmlMarkup(content), location)));
        }

        // XMLCDATA
        if self.characters.peek_seq(8) == "![CDATA[" {
            self.characters.skip_count_in_place(8);
            loop {
                if self.characters.peek_or_zero() == ']' && self.characters.peek_seq(3) == "]]>" {
                    self.characters.skip_count_in_place(3);
                    break;
                } else if CharacterValidator::is_line_terminator(self.characters.peek_or_zero()) {
                    self.consume_line_terminator();
                } else if self.characters.reached_end() {
                    self.add_unexpected_error();
                    return Err(ParsingFailure);
                } else {
                    self.characters.next();
                }
            }

            let location = start.combine_with(self.cursor_location());
            let content = self.compilation_unit.text()[location.first_offset..location.last_offset].to_owned();

            return Ok(Some((Token::XmlMarkup(content), location)));
        }

        // XMLPI
        if self.characters.peek_or_zero() == '?' {
            self.characters.next();
            loop {
                if self.characters.peek_or_zero() == '?' && self.characters.peek_at_or_zero(1) == '>' {
                    self.characters.skip_count_in_place(2);
                    break;
                } else if CharacterValidator::is_line_terminator(self.characters.peek_or_zero()) {
                    self.consume_line_terminator();
                } else if self.characters.reached_end() {
                    self.add_unexpected_error();
                    return Err(ParsingFailure);
                } else {
                    self.characters.next();
                }
            }

            let location = start.combine_with(self.cursor_location());
            let content = self.compilation_unit.text()[location.first_offset..location.last_offset].to_owned();

            return Ok(Some((Token::XmlMarkup(content), location)));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::ns::*;

    #[test]
    fn tokenize_n_per_n() {
        let _n = "n".to_owned();
        let source = CompilationUnit::new(None, "n * n".into(), &CompilerOptions::new());
        let mut tokenizer = Tokenizer::new(&source);
        let Ok((Token::Identifier(name), _)) = tokenizer.scan_ie_div() else { panic!() };
        assert_eq!(name, "n");
        assert!(matches!(tokenizer.scan_ie_div(), Ok((Token::Times, _))));
        let Ok((Token::Identifier(name), _)) = tokenizer.scan_ie_div() else { panic!() };
        assert_eq!(name, "n");
    }

    #[test]
    fn tokenize_comments() {
        let _n = "n".to_owned();
        let source = CompilationUnit::new(None, "
            // Single-line comment
            /* Multi-line comment */
        ".into(), &CompilerOptions::new());
        let mut tokenizer = Tokenizer::new(&source);
        assert!(matches!(tokenizer.scan_ie_div(), Ok((Token::Eof, _))));
        assert_eq!(source.comments()[0].content(), " Single-line comment");
        assert_eq!(source.comments()[1].content(), " Multi-line comment ");
    }

    #[test]
    fn tokenize_strings() {
        let source = CompilationUnit::new(None, r###"
            "Some \u{41}\u0041\x41 content"
            """
            Another
                common
            content
            """
            "a\b"
            @"a\b"
        "###.into(), &CompilerOptions::new());
        let mut tokenizer = Tokenizer::new(&source);

        let Ok((Token::StringLiteral(s), _)) = tokenizer.scan_ie_div() else { panic!() };
        assert_eq!(s, "Some AAA content");

        let Ok((Token::StringLiteral(s), _)) = tokenizer.scan_ie_div() else { panic!() };
        assert_eq!(s, "Another\n    common\ncontent");

        let Ok((Token::StringLiteral(s), _)) = tokenizer.scan_ie_div() else { panic!() };
        assert_eq!(s, "a\x08");

        let Ok((Token::StringLiteral(s), _)) = tokenizer.scan_ie_div() else { panic!() };
        assert_eq!(s, "a\\b");
    }

    #[test]
    fn tokenize_numbers() {
        let numbers: Vec<f64> = vec![
            0.0,
            50.0,
            1_000.0,
            0.5,
            0.5,
            1_000.0,
            1_000.0,
            0.001,
            0.0,
            0.0,
        ];
        let source = CompilationUnit::new(None, r###"
            0
            50
            1_000
            0.5
            .5
            1e3
            1e+3
            1e-3
            0x00_00
            0b0000_0000
            0f
        "###.into(), &CompilerOptions::new());
        let mut tokenizer = Tokenizer::new(&source);
        for n in numbers {
            let Ok((Token::NumericLiteral(n2, suffix), location)) = tokenizer.scan_ie_div() else { panic!() };
            assert_eq!(n, NumericLiteral { value: n2, location, suffix }.parse_double(false).unwrap());
        }
    }

    #[test]
    fn tokenize_regexp() {
        let source = CompilationUnit::new(None, r###"
            /(?:)/
            /(?:)/gi
        "###.into(), &CompilerOptions::new());

        let mut tokenizer = Tokenizer::new(&source);

        let Ok((Token::Div, start)) = tokenizer.scan_ie_div() else { panic!() };
        let Ok((Token::RegExpLiteral { body, flags }, _)) = tokenizer.scan_regexp_literal(start) else { panic!() };
        assert_eq!(body, "(?:)");
        assert_eq!(flags, "");

        let Ok((Token::Div, start)) = tokenizer.scan_ie_div() else { panic!() };
        let Ok((Token::RegExpLiteral { body, flags }, _)) = tokenizer.scan_regexp_literal(start) else { panic!() };
        assert_eq!(body, "(?:)");
        assert_eq!(flags, "gi");
    }
}